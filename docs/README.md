# docs/ — документация прототипа CREDO

Спецификация, язык, требования, журнал вопросов и решений, задачи по коду —
всё для `prototypes/credo2`. Точка входа — этот файл; правила журнала —
[`BRIEF.md`](BRIEF.md).

## Карта

```
docs/
  README.md          # этот файл: карта и каноны
  BRIEF.md           # процесс журнала Q/D: ID, шаблоны, рецепты, миграция
  TRACEABILITY.md    # связи Q → D → feature
  OPEN_QUESTIONS.md  # архив: очередь миграции (перенесённые — указателями)
  SPECIFICATION.md   # целевая архитектура; §10 — краткие формулировки решений
  GRAMMAR.md         # язык v0.1 (EBNF включён)
  CHANGELOG.md       # хронология прототипа
  questions/         # Q1.md, Q2.md, … — по файлу на вопрос
  decisions/         # D15-….md — по файлу на решение (Dn = № строки SPEC §10)
  features/          # Gherkin-требования + README (статусы/приоритеты/счётчики)
  tasks/             # реестр задач по коду (сводка + карточки T-XX)
```

## Каноны (политика Q41: «один факт — один канон»)

| Документ | Канон чего |
|---|---|
| `SPECIFICATION.md` | целевая архитектура; §10 — краткие формулировки решений |
| `questions/Qn.md` + `decisions/Dn-<слаг>.md` | полный контекст «вопрос → решение» |
| `features/README.md` | статусы, приоритеты и счётчики сценариев (проверяет `../tests/features_inventory.rs`) |
| `GRAMMAR.md` | язык v0.1 (EBNF включён) |
| `tasks/` | реестр задач по коду (не норматив) |
| `CHANGELOG.md` | хронология прототипа |
| корневой `DECISIONS.md` | сквозные архитектурные ADR |
| корневой `AGENTS.md` | операционная инструкция агентам |

## Куда за чем

| Задача | Куда |
|---|---|
| Понять, почему решение принято | [`TRACEABILITY.md`](TRACEABILITY.md) → D → Q |
| Завести вопрос или решение | [`BRIEF.md`](BRIEF.md) §4–5 |
| Понять, что и как реализовано | [`features/README.md`](features/README.md) |
| Взять задачу по коду | [`tasks/README.md`](tasks/README.md) |
| Проверить требования тестом | `cargo test --all` в `prototypes/credo2` |

## Состояние миграции

`OPEN_QUESTIONS.md` (42 записи) разбирается на отдельные файлы. Пилот — **Q1**
(2026-09-26): [`questions/Q1.md`](questions/Q1.md) +
[`decisions/D15-evolution-credo2.md`](decisions/D15-evolution-credo2.md).
Прогресс — в [`TRACEABILITY.md`](TRACEABILITY.md); процедура —
[`BRIEF.md`](BRIEF.md) §7.
