"""Загрузка состояния цикла (`.opencode/state/current/*.yaml`).

Формы данных и ограничения — досье `docs/analysis/pm-tool-design-2026-09-30.md`.
"""

from __future__ import annotations

import logging
from pathlib import Path
from typing import Any

import yaml

log = logging.getLogger("pm_agents")

_MULTI_KEYS = ("entries", "actions", "items", "log", "records", "history")


class StateError(RuntimeError):
    """Структура состояния не соответствует ожидаемой."""


def load_yaml(path: Path) -> Any:
    """Безопасно читает YAML; отсутствующий файл — это `None`."""
    if not path.exists():
        log.warning("Файл не найден: %s", path)
        return None
    try:
        with open(path, encoding="utf-8") as fh:
            return yaml.safe_load(fh)
    except yaml.YAMLError as exc:
        raise StateError(f"Ошибка парсинга YAML {path}: {exc}") from exc


def load_records(path: Path, *, context: str, allow_single: bool = False) -> list[dict]:
    """Возвращает список записей.

    Для `progress`/`receipts` ожидается список (или контейнер с известным
    ключом); словарь без контейнера — структурная ошибка (не маскируем).
    Для `next_action` допустим одиночный словарь (`allow_single=True`).
    """
    data = load_yaml(path)
    if data is None:
        return []
    if isinstance(data, list):
        return [row for row in data if isinstance(row, dict)]
    if isinstance(data, dict):
        for key in _MULTI_KEYS:
            value = data.get(key)
            if isinstance(value, list):
                return [row for row in value if isinstance(row, dict)]
        if allow_single:
            return [data]
        keys = ", ".join(sorted(str(k) for k in data)[:8])
        raise StateError(
            f"{context}: ожидался список записей или контейнер {_MULTI_KEYS}; "
            f"получены ключи: {keys}"
        )
    raise StateError(f"{context}: неожиданный тип {type(data).__name__}")
