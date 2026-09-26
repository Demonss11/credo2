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
| `tasks/` | реестр задач по коду (не норматив; Источник — `Dn`/`Qn`) |
| `CHANGELOG.md` | хронология прототипа |
| корневой `DECISIONS.md` | сквозные архитектурные ADR |
| корневой `AGENTS.md` | операционная инструкция агентам |

## Куда за чем

| Задача | Куда |
|---|---|
| Понять, почему решение принято | [`TRACEABILITY.md`](TRACEABILITY.md) → D → Q |
| Завести вопрос или решение | [`BRIEF.md`](BRIEF.md) §4–5 |
| Решить, нужна ли задача по коду | [`BRIEF.md`](BRIEF.md) §5.3 (сверка с кодом) → [`tasks/README.md`](tasks/README.md) |
| Понять, что и как реализовано | [`features/README.md`](features/README.md) |
| Взять задачу по коду | [`tasks/README.md`](tasks/README.md) |
| Понять, кто из команды агентов что делает | [`../AGENTS.md`](../AGENTS.md) §Рабочая группа агентов |
| Проверить требования тестом | `cargo test --all` в `prototypes/credo2` |

## Состояние миграции

`OPEN_QUESTIONS.md` (42 записи) разбирается на отдельные файлы. Пилот — **Q1**
(2026-09-26): [`questions/Q1.md`](questions/Q1.md) +
[`decisions/D15-evolution-credo2.md`](decisions/D15-evolution-credo2.md).
Каждый перенос сопровождается сверкой решения с кодом и решением «нужна ли
задача» ([`BRIEF.md`](BRIEF.md) §5.3); миграция идёт блоками параллельно
кодингу — интерфейс потоков — [`tasks/`](tasks/README.md) ([`BRIEF.md`](BRIEF.md) §8).
Прогресс — в [`TRACEABILITY.md`](TRACEABILITY.md); процедура —
[`BRIEF.md`](BRIEF.md) §7.
