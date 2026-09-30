"""CLI `pm-agents`: пайплайн «состояние+ленты → event log → DFG/варианты/метрики → артефакты»."""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import logging
import sys
import tempfile
from pathlib import Path

from . import __version__
from .events import build_event_log
from .io_state import StateError
from .mining import compute_metrics, compute_variants, discover_dfg
from .report import render_report_md
from .viz import VizUnavailable

log = logging.getLogger("pm_agents")

DEFAULT_STATE_DIR = Path(".opencode/state/current")
DEFAULT_MAIL_DIR = Path(".opencode/mail")


def default_output_dir() -> Path:
    """По умолчанию артефакты — вне репозитория (конвенция session-analysis)."""
    return Path(tempfile.gettempdir()) / "opencode" / f"pm-{dt.date.today():%Y-%m-%d}"


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
        default=DEFAULT_STATE_DIR,
        help="каталог состояния (progress/receipts/next_action)",
    )
    parser.add_argument(
        "--mail-dir",
        type=Path,
        default=DEFAULT_MAIL_DIR,
        help="каталог лент (mail/*.md)",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help="каталог артефактов (по умолчанию temp/opencode/pm-<дата>)",
    )
    parser.add_argument(
        "--source",
        choices=("all", "state", "mail"),
        default="all",
        help="источники событий (state = progress)",
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


def _configure_stdout() -> None:
    """Windows-консоль (cp1251) не должна падать на «×», «→» и т.п."""
    try:
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    except (AttributeError, OSError):  # pragma: no cover - зависит от окружения
        pass


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    _configure_stdout()
    logging.basicConfig(
        level=getattr(logging, args.log_level),
        format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
        datefmt="%H:%M:%S",
    )

    output_dir: Path = args.output_dir or default_output_dir()
    output_dir.mkdir(parents=True, exist_ok=True)

    try:
        logbook = build_event_log(
            args.state_dir,
            args.mail_dir,
            source=args.source,
            include_service=not args.exclude_service,
            since=args.since,
        )
    except (RuntimeError, StateError) as exc:
        log.error("%s", exc)
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
        state_dir=args.state_dir,
        mail_dir=args.mail_dir if args.source != "state" else None,
        output_dir=output_dir,
        viz_done=viz_done,
        notes=notes,
    )
    (output_dir / "summary.md").write_text(report, encoding="utf-8")
    print(report)
    log.info("Артефакты: %s", output_dir)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
