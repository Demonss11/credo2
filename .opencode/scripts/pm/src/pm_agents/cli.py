"""CLI `pm-agents`: пайплайн «состояние+ленты → event log → DFG/варианты/метрики → артефакты»."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import logging
import sys
from pathlib import Path

from . import __version__
from .archive import ARCHIVE_FILENAME, merge_archive, next_number
from .events import finalize_event_log, parse_events
from .io_state import StateError
from .mining import compute_metrics, compute_variants, discover_dfg
from .report import render_report_md
from .viz import VizUnavailable

log = logging.getLogger("pm_agents")

DEFAULT_STATE_DIR = Path(".opencode/state/current")
DEFAULT_MAIL_DIR = Path(".opencode/mail")
DEFAULT_PM_DIR = Path(".opencode/scripts/pm")


def default_output_dir() -> Path:
    """Каталог артефактов по умолчанию — `output/` внутри `pm` (вне git)."""
    root = find_repo_root(Path.cwd())
    pm_dir = (
        root / DEFAULT_PM_DIR
        if root is not None
        else Path(__file__).resolve().parents[2]
    )
    return pm_dir / "output"


def find_repo_root(start: Path) -> Path | None:
    """Корень репозитория вверх по дереву (маркер — `.opencode/state/current`)."""
    current = start.resolve()
    for candidate in (current, *current.parents):
        if (candidate / ".opencode" / "state" / "current").is_dir():
            return candidate
    return None


def resolve_state_dir(explicit: Path | None) -> Path:
    """`--state-dir` либо каталог состояния от корня репозитория."""
    if explicit is not None:
        return explicit
    root = find_repo_root(Path.cwd())
    return root / DEFAULT_STATE_DIR if root is not None else DEFAULT_STATE_DIR


def resolve_mail_dir(explicit: Path | None) -> Path:
    """`--mail-dir` либо каталог лент от корня репозитория."""
    if explicit is not None:
        return explicit
    root = find_repo_root(Path.cwd())
    return root / DEFAULT_MAIL_DIR if root is not None else DEFAULT_MAIL_DIR


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="pm-agents",
        description="Process mining агентского процесса CREDO2 (D81)",
    )
    parser.add_argument(
        "--version", action="version", version=f"pm-agents {__version__}"
    )
    parser.add_argument(
        "--state-dir",
        type=Path,
        default=None,
        help="каталог состояния (по умолчанию — от корня репозитория)",
    )
    parser.add_argument(
        "--mail-dir",
        type=Path,
        default=None,
        help="каталог лент (по умолчанию — от корня репозитория)",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help="каталог артефактов (по умолчанию .opencode/scripts/pm/output)",
    )
    parser.add_argument(
        "--archive",
        type=Path,
        default=None,
        metavar="PATH",
        help="файл архива событий (по умолчанию <output-dir>/events.jsonl)",
    )
    parser.add_argument(
        "--no-archive",
        action="store_true",
        help="разовый прогон без чтения и записи архива",
    )
    parser.add_argument(
        "--source",
        choices=("all", "state", "mail"),
        default="all",
        help="срез отчёта по источникам (на архив не влияет)",
    )
    parser.add_argument(
        "--since",
        type=dt.date.fromisoformat,
        metavar="YYYY-MM-DD",
        default=None,
        help="только события с датой >= значения",
    )
    parser.add_argument(
        "--exclude-service",
        action="store_true",
        help="исключить служебные кейсы (service-*)",
    )
    parser.add_argument(
        "--threshold",
        type=int,
        default=2,
        help="порог частоты для «популярных» переходов",
    )
    parser.add_argument(
        "--no-viz",
        action="store_true",
        help="не строить графики (достаточно core-зависимостей)",
    )
    parser.add_argument(
        "--log-level",
        default="INFO",
        choices=("DEBUG", "INFO", "WARNING", "ERROR"),
    )
    return parser.parse_args(argv)


def _write_csv(path: Path, header: list[str], rows: list[list[object]]) -> None:
    with open(path, "w", encoding="utf-8-sig", newline="") as fh:
        writer = csv.writer(fh, delimiter=";")
        writer.writerow(header)
        writer.writerows(rows)


def _configure_stdio() -> None:
    """Windows-консоль/pipe (cp1251) не должна искажать «×», «→», кириллицу."""
    for stream in (sys.stdout, sys.stderr):
        try:
            stream.reconfigure(encoding="utf-8", errors="replace")
        except (AttributeError, OSError):  # pragma: no cover - зависит от окружения
            pass


def main(argv: list[str] | None = None) -> int:
    _configure_stdio()
    args = parse_args(argv)
    logging.basicConfig(
        level=getattr(logging, args.log_level),
        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
        datefmt="%H:%M:%S",
    )

    output_dir: Path = args.output_dir or default_output_dir()
    output_dir.mkdir(parents=True, exist_ok=True)

    state_dir = resolve_state_dir(args.state_dir)
    mail_dir = resolve_mail_dir(args.mail_dir)

    # Источники читаются всегда (оба), чтобы архив не терял события из-за
    # фильтров среза: --source/--since/--exclude-service действуют ниже.
    try:
        parsed_events = parse_events(state_dir, mail_dir)
    except (RuntimeError, StateError, OSError) as exc:
        log.error(
            "%s (подсказка: запускайте из репозитория или укажите "
            "--state-dir/--mail-dir)",
            exc,
        )
        return 2

    archive_path: Path | None = None
    archive_added: int | None = None
    archive_total: int | None = None
    archive_next: int | None = None
    merged_events = parsed_events
    if not args.no_archive:
        archive_path = args.archive or output_dir / ARCHIVE_FILENAME
        try:
            merged_events, archive_added = merge_archive(archive_path, parsed_events)
        except OSError as exc:
            log.error("Архив недоступен: %s (разовый прогон — с --no-archive)", exc)
            return 2
        archive_total = len(merged_events)
        archive_next = next_number(merged_events)
        log.info(
            "Архив: +%d новых, всего %d событий; следующее — №%d (%s).",
            archive_added,
            archive_total,
            archive_next,
            archive_path,
        )

    try:
        logbook = finalize_event_log(
            state_dir,
            merged_events,
            source=args.source,
            include_service=not args.exclude_service,
            since=args.since,
        )
    except (RuntimeError, StateError) as exc:
        log.error(
            "%s (подсказка: запускайте из репозитория или укажите "
            "--state-dir/--mail-dir)",
            exc,
        )
        return 2

    dfg = discover_dfg(logbook)
    variants = compute_variants(logbook)
    metrics = compute_metrics(logbook)

    notes = [
        "Даты в состоянии — без времени: задержки в секундах не измеряются, "
        "используйте позиции активностей.",
        "События mail — только заголовки `## <роль> · <дата> · <статус>`; "
        "остальной текст лент не парсится.",
    ]
    if archive_path is not None:
        notes.append(
            "Архив событий: повторный прогон на тех же данных добавляет 0 "
            "событий; фильтры (--source/--since/--exclude-service) на архив "
            "не влияют."
        )
    else:
        notes.append(
            "Прогон без архива (--no-archive): события только из текущих источников."
        )

    (output_dir / "metrics.json").write_text(
        json.dumps(metrics, ensure_ascii=False, indent=2, default=str),
        encoding="utf-8",
    )
    _write_csv(
        output_dir / "dfg_edges.csv",
        ["from", "to", "count"],
        [
            [src, dst, count]
            for (src, dst), count in sorted(dfg.edges.items(), key=lambda kv: -kv[1])
        ],
    )
    _write_csv(
        output_dir / "variants.csv",
        ["variant", "count"],
        [[" -> ".join(sequence), count] for sequence, count in variants],
    )
    _write_csv(
        output_dir / "events.csv",
        [
            "n",
            "date",
            "source",
            "case_id",
            "activity",
            "role",
            "action",
            "result",
            "iteration",
            "expect_match",
        ],
        [
            [
                event.n or "",
                event.date.isoformat() if event.date else "",
                event.source,
                event.case_id,
                event.activity,
                event.role or "",
                event.action,
                event.result,
                event.iteration if event.iteration is not None else "",
                event.expect_match if event.expect_match is not None else "",
            ]
            for event in logbook.events
        ],
    )

    viz_done = False
    if not args.no_viz:
        try:
            from . import viz

            activities = sorted(metrics["activity_frequency"])
            viz.plot_dfg(
                dfg.edges,
                output_dir / "agent_graph_all.png",
                threshold=0,
                title="Граф взаимодействия ролей",
            )
            viz.plot_dfg(
                dfg.edges,
                output_dir / "agent_graph_popular.png",
                threshold=args.threshold,
                title="Популярные переходы",
            )
            viz.plot_heatmap(dfg.edges, activities, output_dir / "agent_heatmap.png")
            viz.plot_frequency(
                metrics["activity_frequency"], output_dir / "agent_frequencies.png"
            )
            viz.plot_sankey(dfg.edges, activities, output_dir / "agent_sankey.html")
            viz_done = True
        except VizUnavailable as exc:
            log.warning("Визуализация пропущена: %s", exc)
            notes.append(str(exc))

    report = render_report_md(
        metrics=metrics,
        dfg_edges=dfg.edges,
        variants=variants,
        generated_at=dt.datetime.now(),
        state_dir=state_dir,
        mail_dir=mail_dir if args.source != "state" else None,
        output_dir=output_dir,
        archive_path=archive_path,
        archive_added=archive_added,
        archive_total=archive_total,
        archive_next=archive_next,
        viz_done=viz_done,
        notes=notes,
    )
    (output_dir / "summary.md").write_text(report, encoding="utf-8")
    print(report)
    log.info("Артефакты: %s", output_dir)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
