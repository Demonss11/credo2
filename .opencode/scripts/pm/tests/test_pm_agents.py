"""Юнит-тесты pm_agents (core; без extra viz)."""

from __future__ import annotations

import datetime as dt
from pathlib import Path

import pytest

from pm_agents.cli import main
from pm_agents.events import build_event_log
from pm_agents.io_mail import normalize_role, parse_letter
from pm_agents.io_state import StateError, load_records
from pm_agents.mining import compute_metrics, compute_variants, discover_dfg

FIXTURES = Path(__file__).parent / "fixtures"


def test_load_records_strict(tmp_path: Path) -> None:
    bad = tmp_path / "progress.yaml"
    bad.write_text("foo: bar\n", encoding="utf-8")
    with pytest.raises(StateError):
        load_records(bad, context="progress")


def test_mail_parser() -> None:
    entries = parse_letter(FIXTURES / "mail" / "T-01.md")
    assert [entry.role for entry in entries] == ["migrator", "validator", "coder"]
    assert entries[0].date == dt.date(2026, 9, 28)
    assert normalize_role("`сервисная сессия`") == "service-session"


def test_unknown_case_not_fallback() -> None:
    log = build_event_log(FIXTURES / "state", None)
    cases = {event.case_id for event in log.events}
    assert "unknown-3" in cases
    assert "T-02" not in cases


def test_dfg_and_variants() -> None:
    log = build_event_log(FIXTURES / "state", FIXTURES / "mail")
    dfg = discover_dfg(log)
    assert dfg.edges[("git", "coder")] == 1
    variants = compute_variants(log)
    assert variants
    counts = [count for _, count in variants]
    assert counts == sorted(counts, reverse=True)


def test_metrics() -> None:
    log = build_event_log(FIXTURES / "state", FIXTURES / "mail")
    metrics = compute_metrics(log)
    assert metrics["receipts"]["accepted_with_notes"] == 1
    assert metrics["acceptance_rate"] == pytest.approx(2 / 3, abs=1e-3)
    assert metrics["queue_length"] == 2
    assert metrics["expect_partial"] == 1
    assert metrics["case_kinds"]["tasks"] >= 1


def test_undated_events_sort_last(tmp_path: Path) -> None:
    state = tmp_path / "state"
    state.mkdir()
    (state / "progress.yaml").write_text(
        '- at: "2026-09-28"\n'
        "  task: T-99\n"
        "  action: dispatch\n"
        "  role: git\n"
        '- at: "не дата"\n'
        "  task: T-99\n"
        "  action: complete\n",
        encoding="utf-8",
    )
    log = build_event_log(state, None)
    assert [event.activity for event in log.events] == ["git", "complete"]


def test_cli_end_to_end(tmp_path: Path) -> None:
    rc = main(
        [
            "--state-dir",
            str(FIXTURES / "state"),
            "--mail-dir",
            str(FIXTURES / "mail"),
            "--output-dir",
            str(tmp_path),
            "--no-viz",
            "--log-level",
            "ERROR",
        ]
    )
    assert rc == 0
    assert (tmp_path / "metrics.json").exists()
    assert (tmp_path / "summary.md").exists()
    assert (tmp_path / "dfg_edges.csv").exists()
    assert (tmp_path / "variants.csv").exists()
