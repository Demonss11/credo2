"""Event log: слияние `progress` (действия lead) и `mail` (вызовы ролей).

Порядок событий — `(дата, приоритет источника progress<mail, локальный индекс)`;
даты в состоянии — без времени, поэтому «секундные» задержки не измеряются.
"""

from __future__ import annotations

import datetime as dt
import logging
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from .io_mail import MailEntry, load_mail
from .io_state import load_records

log = logging.getLogger("pm_agents")

_EMPTY_DATE = dt.date(1970, 1, 1)
_SOURCE_RANK = {"progress": 0, "mail": 1}


@dataclass
class Event:
    seq: int
    case_id: str
    activity: str
    kind: str  # role | action
    source: str  # progress | mail
    date: dt.date | None
    timestamp: dt.datetime
    action: str = ""
    role: str | None = None
    iteration: Any = None
    result: str = ""
    expect_match: Any = None
    n: int = 0  # сквозной номер в архиве (0 — вне архива)


@dataclass
class EventLog:
    events: list[Event]
    receipts: list[dict] = field(default_factory=list)
    next_action: dict | None = None


def _parse_date(value: Any) -> dt.date | None:
    if isinstance(value, dt.datetime):
        return value.date()
    if isinstance(value, dt.date):
        return value
    if isinstance(value, str):
        for fmt in ("%Y-%m-%d", "%d.%m.%Y"):
            try:
                return dt.datetime.strptime(value, fmt).date()
            except ValueError:
                continue
    return None


def _normalize_role(raw: str) -> str:
    return raw.strip().strip("`").strip().lower()


def _progress_case(rec: dict, idx: int) -> str:
    task = rec.get("task")
    if task:
        return str(task)
    return f"unknown-{idx}"


def _progress_events(records: list[dict]) -> list[Event]:
    events: list[Event] = []
    for idx, rec in enumerate(records):
        role_raw = rec.get("role") or rec.get("agent")
        role = _normalize_role(str(role_raw)) if role_raw else None
        action = str(rec.get("action") or "")
        activity = role or action
        if not activity:
            log.debug("Запись progress без роли/действия — пропуск: %s", rec)
            continue
        events.append(
            Event(
                seq=0,
                case_id=_progress_case(rec, idx),
                activity=activity,
                kind="role" if role else "action",
                source="progress",
                date=_parse_date(rec.get("at")),
                timestamp=dt.datetime.min,
                action=action,
                role=role,
                iteration=rec.get("iteration"),
                result=str(rec.get("result") or ""),
                expect_match=rec.get("expect_match"),
            )
        )
    return events


def _mail_events(entries: list[MailEntry]) -> list[Event]:
    return [
        Event(
            seq=0,
            case_id=entry.case_id,
            activity=entry.role,
            kind="role",
            source="mail",
            date=entry.date,
            timestamp=dt.datetime.min,
            action="mail-entry",
            role=entry.role,
            result=entry.status,
        )
        for entry in entries
    ]


def parse_events(
    state_dir: Path, mail_dir: Path | None, *, source: str = "all"
) -> list[Event]:
    """Разбор источников без фильтров среза (события «как есть» — для архива)."""
    events: list[Event] = []
    if source in ("all", "state"):
        progress = load_records(state_dir / "progress.yaml", context="progress")
        events.extend(_progress_events(progress))
    if source in ("all", "mail") and mail_dir is not None:
        events.extend(_mail_events(load_mail(mail_dir)))
    return events


def finalize_event_log(
    state_dir: Path,
    events: list[Event],
    *,
    source: str = "all",
    include_service: bool = True,
    since: dt.date | None = None,
) -> EventLog:
    """Фильтры среза, порядок и синтетические метки времени; читает receipts/next_action."""
    if source == "state":
        events = [e for e in events if e.source == "progress"]
    elif source == "mail":
        events = [e for e in events if e.source == "mail"]
    if not include_service:
        events = [e for e in events if not e.case_id.startswith("service-")]
    if since is not None:
        events = [e for e in events if e.date is None or e.date >= since]

    receipts = load_records(state_dir / "receipts.yaml", context="receipts")
    next_action_records = load_records(
        state_dir / "next_action.yaml", context="next_action", allow_single=True
    )

    if not events:
        raise RuntimeError(
            "Event log пуст: нет событий (проверьте --source и фильтры)."
        )

    events = list(events)
    for local_idx, event in enumerate(events):
        event.seq = local_idx
    events.sort(
        key=lambda e: (
            e.date is None,
            e.date or _EMPTY_DATE,
            _SOURCE_RANK[e.source],
            e.seq,
        )
    )

    for seq, event in enumerate(events, start=1):
        event.seq = seq
        base = dt.datetime.combine(event.date or _EMPTY_DATE, dt.time())
        event.timestamp = base + dt.timedelta(seconds=seq)

    log.info(
        "Event log: %d событий, %d кейсов (progress=%d, mail=%d).",
        len(events),
        len({e.case_id for e in events}),
        sum(1 for e in events if e.source == "progress"),
        sum(1 for e in events if e.source == "mail"),
    )
    return EventLog(
        events=events,
        receipts=receipts,
        next_action=next_action_records[0] if next_action_records else None,
    )


def build_event_log(
    state_dir: Path,
    mail_dir: Path | None = None,
    *,
    source: str = "all",
    include_service: bool = True,
    since: dt.date | None = None,
) -> EventLog:
    """Совместимая обёртка: разбор источников + финализация (без архива)."""
    return finalize_event_log(
        state_dir,
        parse_events(state_dir, mail_dir, source=source),
        source=source,
        include_service=include_service,
        since=since,
    )
