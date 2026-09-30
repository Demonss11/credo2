"""DFG, варианты и метрики поверх event log."""

from __future__ import annotations

from collections import Counter, defaultdict
from dataclasses import dataclass
from typing import Any

from .events import Event, EventLog

_ACCEPTED_VERDICTS = ("accepted", "accepted_with_notes")


@dataclass
class DfgResult:
    edges: dict[tuple[str, str], int]
    start_activities: dict[str, int]
    end_activities: dict[str, int]


def group_by_case(log: EventLog) -> dict[str, list[Event]]:
    """Кейсы → события, упорядоченные по глобальному номеру."""
    cases: dict[str, list[Event]] = defaultdict(list)
    for event in log.events:
        cases[event.case_id].append(event)
    for events in cases.values():
        events.sort(key=lambda e: e.seq)
    return dict(cases)


def discover_dfg(log: EventLog) -> DfgResult:
    """Directly-Follows Graph по каждому кейсу."""
    edges: Counter[tuple[str, str]] = Counter()
    start: Counter[str] = Counter()
    end: Counter[str] = Counter()
    for events in group_by_case(log).values():
        sequence = [e.activity for e in events]
        if not sequence:
            continue
        start[sequence[0]] += 1
        end[sequence[-1]] += 1
        edges.update(zip(sequence, sequence[1:]))
    return DfgResult(
        edges=dict(edges), start_activities=dict(start), end_activities=dict(end)
    )


def compute_variants(
    log: EventLog, top: int | None = None
) -> list[tuple[tuple[str, ...], int]]:
    """Уникальные маршруты кейсов, по убыванию частоты."""
    variants: Counter[tuple[str, ...]] = Counter(
        tuple(e.activity for e in events) for events in group_by_case(log).values()
    )
    ordered = sorted(variants.items(), key=lambda kv: (-kv[1], len(kv[0]), kv[0]))
    return ordered[:top] if top else ordered


def compute_metrics(log: EventLog) -> dict[str, Any]:
    """Метрики процесса (без выводов о «секундах» — даты без времени)."""
    cases = group_by_case(log)
    events = log.events
    metrics: dict[str, Any] = {}

    metrics["total_events"] = len(events)
    metrics["total_cases"] = len(cases)
    metrics["case_kinds"] = {
        "tasks": sum(1 for c in cases if c.startswith("T-")),
        "service": sum(1 for c in cases if c.startswith("service-")),
        "other": sum(1 for c in cases if not c.startswith(("T-", "service-"))),
    }
    metrics["unique_activities"] = len({e.activity for e in events})
    metrics["events_by_source"] = dict(Counter(e.source for e in events))
    metrics["activity_frequency"] = dict(
        Counter(e.activity for e in events).most_common()
    )

    # Приёмки (receipts): accepted_with_notes считается приёмкой.
    verdicts = Counter(str(row.get("verdict") or "?") for row in log.receipts)
    accepted = sum(verdicts.get(v, 0) for v in _ACCEPTED_VERDICTS)
    rework = verdicts.get("rework", 0)
    total = accepted + rework
    metrics["receipts"] = dict(verdicts)
    metrics["acceptance_rate"] = round(accepted / total, 4) if total else None
    metrics["rework_rate"] = round(rework / total, 4) if total else None

    # Несовпадения expect в progress.
    expect_mismatch = 0
    expect_partial = 0
    for event in events:
        if event.source != "progress" or event.expect_match is None:
            continue
        if event.expect_match is True:
            continue
        if isinstance(event.expect_match, str) and event.expect_match.startswith(
            "partial"
        ):
            expect_partial += 1
        else:
            expect_mismatch += 1
    metrics["expect_mismatch"] = expect_mismatch
    metrics["expect_partial"] = expect_partial

    # Итерации по кейсам (максимум из событий с числом).
    iterations: dict[str, int] = {}
    for case, case_events in cases.items():
        values = [e.iteration for e in case_events if isinstance(e.iteration, int)]
        if values:
            iterations[case] = max(values)
    if iterations:
        metrics["avg_iterations"] = round(sum(iterations.values()) / len(iterations), 2)
        metrics["max_iterations"] = max(iterations.values())

    # Позиции активностей (доля пути кейса; «секунды» не измеряются).
    positions: dict[str, list[float]] = defaultdict(list)
    for case_events in cases.values():
        size = len(case_events)
        for idx, event in enumerate(case_events):
            positions[event.activity].append(idx / (size - 1) if size > 1 else 0.0)
    metrics["activity_position"] = {
        activity: round(sum(values) / len(values), 3)
        for activity, values in sorted(
            positions.items(), key=lambda kv: (-len(kv[1]), kv[0])
        )
    }

    # Длительность кейса в днях (по датам; без времени).
    spans = [
        (
            max(e.date for e in case_events if e.date)
            - min(e.date for e in case_events if e.date)
        ).days
        for case_events in cases.values()
        if any(e.date for e in case_events)
    ]
    if spans:
        metrics["avg_span_days"] = round(sum(spans) / len(spans), 2)
        metrics["max_span_days"] = max(spans)

    metrics["cases"] = dict(
        sorted(
            ((case, len(case_events)) for case, case_events in cases.items()),
            key=lambda kv: (-kv[1], kv[0]),
        )
    )

    next_action = log.next_action
    if next_action:
        queue = next_action.get("next")
        metrics["current_task"] = next_action.get("task")
        metrics["current_iteration"] = next_action.get("iteration")
        metrics["current_status"] = next_action.get("status")
        metrics["current_class"] = next_action.get("class")
        metrics["queue_length"] = len(queue) if isinstance(queue, list) else None

    return metrics
