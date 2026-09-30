"""Визуализации (extra `viz`): DFG PNG, heatmap, частоты, Sankey HTML.

Модуль без тяжёлых импортов на уровне модуля: библиотеки подгружаются лениво,
поэтому core-режим работает без extra `viz` (`--no-viz` не обязателен —
функции сами сообщат о недоступности).
"""

from __future__ import annotations

import importlib
import logging
from pathlib import Path
from typing import Any, Mapping

log = logging.getLogger("pm_agents")

FIG_DPI = 150
NODE_SIZE = 2000
EDGE_WIDTH_SCALE = 0.4


class VizUnavailable(RuntimeError):
    """Не установлен extra `viz` (networkx/matplotlib/plotly)."""


def _require(name: str) -> Any:
    try:
        return importlib.import_module(name)
    except ImportError as exc:  # pragma: no cover - зависит от окружения
        raise VizUnavailable(
            f"Нет модуля {name!r}: установите extra viz — `uv sync --extra viz`"
        ) from exc


def _pyplot() -> Any:
    matplotlib = _require("matplotlib")
    matplotlib.use("Agg")
    return _require("matplotlib.pyplot")


def _save(fig: Any, path: Path, *, dpi: int = FIG_DPI) -> None:
    fig.tight_layout()
    fig.savefig(path, dpi=dpi)
    import matplotlib.pyplot as plt  # noqa: PLC0415 - уже загружен выше

    plt.close(fig)
    log.info("Сохранено: %s", path)


def plot_dfg(
    edges: Mapping[tuple[str, str], int],
    path: Path,
    *,
    threshold: int = 0,
    title: str = "DFG",
    dpi: int = FIG_DPI,
) -> None:
    selected = {edge: count for edge, count in edges.items() if count >= threshold}
    if not selected:
        log.warning("DFG пуст при пороге %d — граф не построен.", threshold)
        return
    nx = _require("networkx")
    plt = _pyplot()

    graph = nx.DiGraph()
    for (src, dst), count in selected.items():
        graph.add_edge(src, dst, weight=count)

    pos = nx.spring_layout(graph, seed=42, k=0.9)
    fig, ax = plt.subplots(figsize=(12, 8))
    widths = [graph[u][v]["weight"] * EDGE_WIDTH_SCALE for u, v in graph.edges()]
    nx.draw_networkx_nodes(
        graph, pos, node_color="lightblue", node_size=NODE_SIZE, ax=ax
    )
    nx.draw_networkx_labels(graph, pos, font_size=10, ax=ax)
    nx.draw_networkx_edges(
        graph,
        pos,
        arrowstyle="->",
        arrowsize=20,
        edge_color="gray",
        width=widths,
        ax=ax,
    )
    nx.draw_networkx_edge_labels(
        graph,
        pos,
        edge_labels={edge: str(count) for edge, count in selected.items()},
        font_size=8,
        ax=ax,
    )
    ax.set_title(f"{title} (частота >= {threshold})")
    ax.axis("off")
    _save(fig, path, dpi=dpi)


def plot_heatmap(
    edges: Mapping[tuple[str, str], int],
    activities: list[str],
    path: Path,
    *,
    dpi: int = FIG_DPI,
) -> None:
    if not edges:
        log.warning("DFG пуст — heatmap не построена.")
        return
    plt = _pyplot()

    index = {activity: i for i, activity in enumerate(activities)}
    size = len(activities)
    matrix = [[0] * size for _ in range(size)]
    for (src, dst), count in edges.items():
        if src in index and dst in index:
            matrix[index[src]][index[dst]] = count

    fig, ax = plt.subplots(figsize=(max(8, size), max(6, size * 0.6)))
    image = ax.imshow(matrix, cmap="Blues")
    fig.colorbar(image, ax=ax)
    ax.set_xticks(range(size), labels=activities, rotation=45, ha="right")
    ax.set_yticks(range(size), labels=activities)
    if size <= 15:
        for row in range(size):
            for col in range(size):
                if matrix[row][col]:
                    ax.text(
                        col,
                        row,
                        str(matrix[row][col]),
                        ha="center",
                        va="center",
                        fontsize=8,
                    )
    ax.set_title("Тепловая карта переходов")
    _save(fig, path, dpi=dpi)


def plot_frequency(
    frequency: Mapping[str, int], path: Path, *, dpi: int = FIG_DPI
) -> None:
    if not frequency:
        log.warning("Частоты пусты — график не построен.")
        return
    plt = _pyplot()

    items = sorted(frequency.items(), key=lambda kv: (kv[1], kv[0]))
    labels = [key for key, _ in items]
    values = [value for _, value in items]
    fig, ax = plt.subplots(figsize=(10, max(4, len(labels) * 0.35)))
    ax.barh(labels, values, color="steelblue")
    ax.set_title("Частота вызовов ролей и действий")
    ax.set_xlabel("Количество событий")
    _save(fig, path, dpi=dpi)


def plot_sankey(
    edges: Mapping[tuple[str, str], int], activities: list[str], path: Path
) -> None:
    if not edges:
        log.warning("DFG пуст — Sankey не построен.")
        return
    go = _require("plotly.graph_objects")

    index = {activity: i for i, activity in enumerate(activities)}
    links = [
        (index[src], index[dst], count)
        for (src, dst), count in edges.items()
        if src in index and dst in index
    ]
    if not links:
        log.warning("Нет валидных рёбер для Sankey.")
        return

    figure = go.Figure(
        data=[
            go.Sankey(
                node=dict(
                    pad=15,
                    thickness=20,
                    line=dict(color="black", width=0.5),
                    label=activities,
                    color="rgba(100,150,220,0.8)",
                ),
                link=dict(
                    source=[link[0] for link in links],
                    target=[link[1] for link in links],
                    value=[link[2] for link in links],
                ),
            )
        ]
    )
    figure.update_layout(title_text="Потоки задач между ролями", font_size=11)
    figure.write_html(str(path))
    log.info("Сохранено: %s", path)
