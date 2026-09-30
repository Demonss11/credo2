"""Markdown-отчёт (stdout и `summary.md`)."""

from __future__ import annotations

import datetime as dt
from pathlib import Path
from typing import Any, Mapping

_CASE_ROWS = 15
_ACTIVITY_ROWS = 20
_DFG_ROWS = 15
_VARIANT_ROWS = 10


def render_report_md(
    *,
    metrics: Mapping[str, Any],
    dfg_edges: Mapping[tuple[str, str], int],
    variants: list[tuple[tuple[str, ...], int]],
    generated_at: dt.datetime,
    state_dir: Path,
    mail_dir: Path | None,
    output_dir: Path,
    archive_path: Path | None,
    archive_added: int | None,
    archive_total: int | None,
    archive_next: int | None,
    viz_done: bool,
    notes: list[str],
) -> str:
    lines: list[str] = []

    def add(text: str = "") -> None:
        lines.append(text)

    add("# pm-agents — отчёт process mining")
    add()
    add(f"- Сгенерировано: {generated_at:%Y-%m-%d %H:%M}")
    add(f"- Источник состояния: `{state_dir}`")
    if mail_dir is not None:
        add(f"- Источник лент: `{mail_dir}`")
    add(
        f"- Каталог артефактов: `{output_dir}`"
        + ("" if viz_done else " (без визуализаций)")
    )
    if archive_path is not None:
        add(
            f"- Архив событий: `{archive_path}`"
            f" (+{archive_added or 0} новых, всего {archive_total or 0};"
            f" следующее событие — №{archive_next or 0})"
        )
    else:
        add("- Архив событий: отключён (`--no-archive`)")
    add()

    kinds = metrics.get("case_kinds", {})
    add("## Сводка")
    add()
    add(
        f"- Событий: **{metrics.get('total_events')}**; кейсов: **{metrics.get('total_cases')}**"
        f" (задачи: {kinds.get('tasks', 0)}, служебные: {kinds.get('service', 0)},"
        f" прочие: {kinds.get('other', 0)})."
    )
    add(
        f"- Уникальных активностей: {metrics.get('unique_activities')};"
        f" источники событий: {metrics.get('events_by_source')}."
    )
    if metrics.get("average_span_days") or metrics.get("avg_span_days"):
        span = metrics.get("avg_span_days", metrics.get("average_span_days"))
        add(
            f"- Средняя длительность кейса: {span} дн.; максимальная: {metrics.get('max_span_days')} дн."
        )
    add()

    frequency = metrics.get("activity_frequency", {})
    if frequency:
        add(f"## Активности (топ-{_ACTIVITY_ROWS})")
        add()
        add("| Активность | Событий |")
        add("|---|---:|")
        for activity, count in list(frequency.items())[:_ACTIVITY_ROWS]:
            add(f"| `{activity}` | {count} |")
        add()

    if dfg_edges:
        add(f"## Переходы (DFG, топ-{_DFG_ROWS})")
        add()
        add("| Из | В | Переходов |")
        add("|---|---|---:|")
        for (src, dst), count in sorted(dfg_edges.items(), key=lambda kv: -kv[1])[
            :_DFG_ROWS
        ]:
            add(f"| `{src}` | `{dst}` | {count} |")
        add()

    if variants:
        add(f"## Варианты маршрутов (топ-{_VARIANT_ROWS})")
        add()
        for sequence, count in variants[:_VARIANT_ROWS]:
            add(f"- **{count}×**: {' → '.join(sequence)}")
        add()

    receipts = metrics.get("receipts", {})
    if receipts:
        add("## Метрики приёмки")
        add()
        for verdict, count in sorted(receipts.items(), key=lambda kv: -kv[1]):
            add(f"- `{verdict}`: {count}")
        acceptance = metrics.get("acceptance_rate")
        rework = metrics.get("rework_rate")
        if acceptance is not None:
            add(f"- Приёмки: {acceptance:.1%}; rework: {(rework or 0):.1%}.")
        add(
            f"- Несовпадения `expect`: mismatch {metrics.get('expect_mismatch', 0)},"
            f" partial {metrics.get('expect_partial', 0)}."
        )
        if "avg_iterations" in metrics:
            add(
                f"- Итерации: средняя {metrics['avg_iterations']},"
                f" максимальная {metrics['max_iterations']}."
            )
        add()

    if metrics.get("current_task"):
        add("## Текущий план (`next_action`)")
        add()
        add(
            f"- Задача: `{metrics.get('current_task')}` (итерация"
            f" {metrics.get('current_iteration')}), статус: {metrics.get('current_status')},"
            f" класс: {metrics.get('current_class')}."
        )
        add(f"- Длина очереди действий: {metrics.get('queue_length')}.")
        add()

    cases = metrics.get("cases", {})
    if cases:
        add(f"## Кейсы (топ-{_CASE_ROWS} по событиям)")
        add()
        add("| Кейс | Событий |")
        add("|---|---:|")
        for case, count in list(cases.items())[:_CASE_ROWS]:
            add(f"| `{case}` | {count} |")
        add()

    add("## Примечания")
    add()
    for note in notes:
        add(f"- {note}")
    add()

    return "\n".join(lines)
