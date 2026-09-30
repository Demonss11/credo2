"""Юнит-тесты pm_agents (core; без extra viz)."""

from __future__ import annotations

import datetime as dt
import json
import shutil
from pathlib import Path

import pytest

from pm_agents.cli import main
from pm_agents.events import build_event_log
from pm_agents.io_mail import normalize_role, parse_letter
from pm_agents.io_state import StateError, load_records
from pm_agents.mining import compute_metrics, compute_variants, discover_dfg

FIXTURES = Path(__file__).parent / "fixtures"


def _run_cli(
    output_dir: Path,
    *,
    state: Path = FIXTURES / "state",
    mail: Path = FIXTURES / "mail",
    extra: list[str] | None = None,
) -> int:
    argv = [
        "--state-dir",
        str(state),
        "--mail-dir",
        str(mail),
        "--output-dir",
        str(output_dir),
        "--no-viz",
        "--log-level",
        "ERROR",
    ]
    if extra:
        argv.extend(extra)
    return main(argv)


def _mail_archive_lines(output_dir: Path) -> int:
    lines = (output_dir / "events.jsonl").read_text(encoding="utf-8").splitlines()
    return sum(1 for line in lines if '"source": "mail"' in line)


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


def test_repo_root_resolution(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    from pm_agents.cli import (
        default_output_dir,
        find_repo_root,
        resolve_mail_dir,
        resolve_state_dir,
    )

    root = tmp_path / "repo"
    (root / ".opencode" / "state" / "current").mkdir(parents=True)
    nested = root / ".opencode" / "scripts" / "pm"
    nested.mkdir(parents=True)
    monkeypatch.chdir(nested)

    assert find_repo_root(Path.cwd()) == root.resolve()
    assert resolve_state_dir(None) == root.resolve() / ".opencode" / "state" / "current"
    assert resolve_mail_dir(None) == root.resolve() / ".opencode" / "mail"
    assert (
        default_output_dir()
        == root.resolve() / ".opencode" / "scripts" / "pm" / "output"
    )
    explicit = Path("custom")
    assert resolve_state_dir(explicit) == explicit


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
    assert (tmp_path / "events.jsonl").exists()


def test_archive_is_idempotent(tmp_path: Path) -> None:
    assert _run_cli(tmp_path) == 0
    archive = tmp_path / "events.jsonl"
    first_archive = archive.read_text(encoding="utf-8")
    first_metrics = (tmp_path / "metrics.json").read_text(encoding="utf-8")
    assert first_archive.strip()

    assert _run_cli(tmp_path) == 0
    assert archive.read_text(encoding="utf-8") == first_archive
    assert (tmp_path / "metrics.json").read_text(encoding="utf-8") == first_metrics


def test_archive_survives_mail_removal(tmp_path: Path) -> None:
    mail = tmp_path / "mail"
    shutil.copytree(FIXTURES / "mail", mail)
    out = tmp_path / "out"
    assert _run_cli(out, mail=mail) == 0
    before = json.loads((out / "metrics.json").read_text(encoding="utf-8"))

    for letter in mail.glob("*.md"):
        letter.unlink()

    assert _run_cli(out, mail=mail) == 0
    after = json.loads((out / "metrics.json").read_text(encoding="utf-8"))
    assert after["total_events"] == before["total_events"]
    assert after["events_by_source"]["mail"] == before["events_by_source"]["mail"]


def test_archive_keeps_identical_headings(tmp_path: Path) -> None:
    mail = tmp_path / "mail"
    mail.mkdir()
    (mail / "T-77.md").write_text(
        "## migrator · 30.09.2026 · готово\n\n- Первый.\n\n"
        "## migrator · 30.09.2026 · готово\n\n- Второй.\n",
        encoding="utf-8",
    )
    out = tmp_path / "out"
    assert _run_cli(out, mail=mail) == 0
    metrics = json.loads((out / "metrics.json").read_text(encoding="utf-8"))
    assert metrics["cases"]["T-77"] == 2

    archive = (out / "events.jsonl").read_text(encoding="utf-8")
    assert _run_cli(out, mail=mail) == 0
    assert (out / "events.jsonl").read_text(encoding="utf-8") == archive


def test_archive_accumulates_new_events(tmp_path: Path) -> None:
    mail = tmp_path / "mail"
    mail.mkdir()
    letter = mail / "T-78.md"
    letter.write_text("## coder · 30.09.2026 · готово\n", encoding="utf-8")
    out = tmp_path / "out"
    assert _run_cli(out, mail=mail) == 0
    assert _mail_archive_lines(out) == 1

    with open(letter, "a", encoding="utf-8") as fh:
        fh.write("## validator · 30.09.2026 · принято\n")

    assert _run_cli(out, mail=mail) == 0
    assert _mail_archive_lines(out) == 2


def test_since_filters_view_not_archive(tmp_path: Path) -> None:
    assert _run_cli(tmp_path) == 0
    archive = (tmp_path / "events.jsonl").read_text(encoding="utf-8")
    full = json.loads((tmp_path / "metrics.json").read_text(encoding="utf-8"))

    assert _run_cli(tmp_path, extra=["--since", "2026-09-29"]) == 0
    assert (tmp_path / "events.jsonl").read_text(encoding="utf-8") == archive
    view = json.loads((tmp_path / "metrics.json").read_text(encoding="utf-8"))
    assert view["total_events"] < full["total_events"]


def test_no_archive_flag(tmp_path: Path) -> None:
    assert _run_cli(tmp_path, extra=["--no-archive"]) == 0
    assert not (tmp_path / "events.jsonl").exists()
    assert (tmp_path / "metrics.json").exists()


def test_custom_archive_path(tmp_path: Path) -> None:
    custom = tmp_path / "custom-events.jsonl"
    assert _run_cli(tmp_path, extra=["--archive", str(custom)]) == 0
    assert custom.exists()
    assert not (tmp_path / "events.jsonl").exists()
