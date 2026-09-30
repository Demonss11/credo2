"""Накопительный архив событий между прогонами (`events.jsonl`).

Источники (состояние, ленты) читаются заново при каждом прогоне, но события
не теряются: новые дописываются в архив, повторы распознаются по стабильному
ключу — набор полей записи плюс номер повтора среди одинаковых. Повторный
прогон на тех же данных архив не меняет; очистка лент (`clean-logs.mjs`) на
историю в архиве не влияет.

Сквозная нумерация: событие получает номер `n` при первом попадании в архив и
не меняется; после очистки лент нумерация продолжается с последнего
заархивированного + 1.
"""

from __future__ import annotations

import datetime as dt
import json
import logging
import os
from collections import Counter
from collections.abc import Iterator
from pathlib import Path
from typing import Any

from .events import Event

log = logging.getLogger("pm_agents")

ARCHIVE_FILENAME = "events.jsonl"


def _identity(event: Event) -> tuple[Any, ...]:
    """Стабильные поля события (без seq/timestamp и позиции в файле).

    Позиционные псевдокейсы `progress` (`unknown-<idx>`) в ключ не входят:
    усечение журнала сдвигает номера позиций, а события должны распознаваться.
    """
    case = event.case_id
    if event.source == "progress" and case.startswith("unknown-"):
        case = ""
    return (
        event.source,
        case,
        event.activity,
        event.kind,
        event.action,
        event.role or "",
        event.date.isoformat() if event.date else "",
        "" if event.iteration is None else str(event.iteration),
        event.result,
        json.dumps(event.expect_match, ensure_ascii=False, sort_keys=True),
    )


def _key(event: Event, occurrence: int) -> str:
    """Ключ события: стабильные поля + номер повтора среди одинаковых."""
    return json.dumps([*_identity(event), occurrence], ensure_ascii=False)


def _keyed(events: list[Event]) -> Iterator[tuple[Event, int]]:
    counts: Counter[tuple[Any, ...]] = Counter()
    for event in events:
        identity = _identity(event)
        counts[identity] += 1
        yield event, counts[identity]


def _row(event: Event, occurrence: int) -> dict[str, Any]:
    return {
        "key": _key(event, occurrence),
        "n": event.n,
        "source": event.source,
        "case_id": event.case_id,
        "activity": event.activity,
        "kind": event.kind,
        "action": event.action,
        "role": event.role,
        "date": event.date.isoformat() if event.date else None,
        "iteration": event.iteration,
        "result": event.result,
        "expect_match": event.expect_match,
    }


def _from_row(row: dict[str, Any]) -> Event:
    raw_date = row.get("date")
    date: dt.date | None = None
    if isinstance(raw_date, str) and raw_date:
        try:
            date = dt.date.fromisoformat(raw_date)
        except ValueError:
            date = None
    role = row.get("role")
    raw_n = row.get("n")
    return Event(
        seq=0,
        case_id=str(row.get("case_id") or ""),
        activity=str(row.get("activity") or ""),
        kind=str(row.get("kind") or "action"),
        source=str(row.get("source") or "progress"),
        date=date,
        timestamp=dt.datetime.min,
        action=str(row.get("action") or ""),
        role=str(role) if role else None,
        iteration=row.get("iteration"),
        result=str(row.get("result") or ""),
        expect_match=row.get("expect_match"),
        n=raw_n if isinstance(raw_n, int) and raw_n > 0 else 0,
    )


def load_archive(path: Path) -> list[Event]:
    """События архива в порядке записи; битые строки — предупреждение и пропуск."""
    if not path.exists():
        return []
    events: list[Event] = []
    skipped = 0
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        try:
            row = json.loads(line)
        except json.JSONDecodeError:
            skipped += 1
            continue
        if isinstance(row, dict):
            events.append(_from_row(row))
        else:
            skipped += 1
    if skipped:
        log.warning("Архив %s: пропущено битых строк — %d.", path, skipped)
    return events


def _write(path: Path, events: list[Event]) -> None:
    """Перезаписывает архив целиком (атомарно: временный файл + `os.replace`)."""
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(f"{path.name}.tmp")
    counts: Counter[tuple[Any, ...]] = Counter()
    with open(tmp, "w", encoding="utf-8", newline="\n") as fh:
        for event in events:
            identity = _identity(event)
            counts[identity] += 1
            row = _row(event, counts[identity])
            fh.write(json.dumps(row, ensure_ascii=False) + "\n")
    os.replace(tmp, path)


def merge_archive(path: Path, new_events: list[Event]) -> tuple[list[Event], int]:
    """Сливает текущий разбор с архивом, дописывая только новые события.

    Возвращает (все события: архив + новые, сколько добавлено). Новым событиям
    присваивается сквозной номер `n` — продолжение нумерации архива. Архив без
    номеров (старый формат) нумеруется по порядку и перезаписывается разово.
    Повторный вызов с теми же `new_events` не добавляет ни событий, ни номеров.
    """
    existing = load_archive(path)
    legacy = any(event.n <= 0 for event in existing)
    counter = 1
    for event in existing:
        if event.n > 0:
            counter = max(counter, event.n + 1)
        else:
            event.n = counter
            counter += 1
    keys = {_key(event, occurrence) for event, occurrence in _keyed(existing)}
    counts: Counter[tuple[Any, ...]] = Counter()
    added: list[Event] = []
    for event in new_events:
        identity = _identity(event)
        counts[identity] += 1
        key = _key(event, counts[identity])
        if key in keys:
            continue
        keys.add(key)
        event.n = counter
        counter += 1
        added.append(event)
    merged = [*existing, *added]
    if added or legacy:
        _write(path, merged)
    return merged, len(added)


def next_number(events: list[Event]) -> int:
    """Следующий номер события архива (для отчёта и логов)."""
    return max((event.n for event in events), default=0) + 1
