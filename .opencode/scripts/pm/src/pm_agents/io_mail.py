"""Tolerant-парсер лент `.opencode/mail/*.md`.

События — заголовки вида `## <роль> · <дата> · <статус>`; строки
`## iteration N` и прочие заголовки без даты пропускаются.
"""

from __future__ import annotations

import datetime as dt
import logging
import re
from dataclasses import dataclass
from pathlib import Path

log = logging.getLogger("pm_agents")

_HEADING = re.compile(
    r"^##\s+(?P<role>[^·#]+?)\s*·\s*(?P<date>\d{2}\.\d{2}\.\d{4})"
    r"(?:\s*·\s*(?P<status>.*?))?\s*$"
)

_ROLE_ALIASES = {
    "сервисная сессия": "service-session",
}


def normalize_role(raw: str) -> str:
    """Нормализует имя роли: снимает кавычки/пробелы, приводит к kebab-case."""
    role = raw.strip().strip("`").strip()
    key = role.lower()
    if key in _ROLE_ALIASES:
        return _ROLE_ALIASES[key]
    return re.sub(r"\s+", "-", key)


@dataclass(frozen=True)
class MailEntry:
    case_id: str
    role: str
    date: dt.date | None
    status: str
    line_no: int


def parse_letter(path: Path) -> list[MailEntry]:
    """Собирает события одной ленты (кейс = имя файла без расширения)."""
    entries: list[MailEntry] = []
    case_id = path.stem
    text = path.read_text(encoding="utf-8", errors="replace")
    for line_no, line in enumerate(text.splitlines(), start=1):
        match = _HEADING.match(line)
        if not match:
            continue
        try:
            date: dt.date | None = dt.datetime.strptime(
                match.group("date"), "%d.%m.%Y"
            ).date()
        except ValueError:
            date = None
        entries.append(
            MailEntry(
                case_id=case_id,
                role=normalize_role(match.group("role")),
                date=date,
                status=(match.group("status") or "").strip().rstrip("."),
                line_no=line_no,
            )
        )
    return entries


def load_mail(mail_dir: Path) -> list[MailEntry]:
    """Собирает события всех лент каталога (сортировка по имени файла)."""
    if not mail_dir.is_dir():
        log.warning("Каталог лент не найден: %s", mail_dir)
        return []
    entries: list[MailEntry] = []
    for path in sorted(mail_dir.glob("*.md")):
        if path.name.startswith("."):
            continue
        entries.extend(parse_letter(path))
    return entries
