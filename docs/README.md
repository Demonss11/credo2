# docs/ — документация прототипа CREDO

Спецификация, язык, требования, журнал вопросов и решений, задачи по коду —
всё для `prototypes/credo2`. Точка входа — этот файл; правила журнала —
[`BRIEF.md`](BRIEF.md).

## Карта

```
docs/
  README.md          # этот файл: карта и каноны
  BRIEF.md           # процесс журнала Q/D: структура, ID, шаблоны, рецепты, целостность
  TRACEABILITY.md    # связи и жизненный цикл Q → D → задача → реализация
  SPECIFICATION.md   # целевая архитектура; §10 — краткие формулировки решений
  GRAMMAR.md         # язык v0.1 (EBNF включён)
  CHANGELOG.md       # хронология прототипа
  questions/         # Q1.md, Q2.md, … + README.md (сводка вопросов)
  decisions/         # Dn-….md + README.md (сводка решений); Dn = № строки SPEC §10
  features/          # Gherkin-требования + README (статусы/приоритеты/счётчики)
  tasks/             # реестр задач по коду (сводка + карточки T-XX)
  analysis/          # досье analyst по задачам (улики решения, не канон; пишет analyst)
  reviews/           # отчёты приёмки (улики, не канон; пишет validator)
  research/          # внешние обзоры (не канон; пишет researcher)
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

`analysis/`, `reviews/` и `research/` — первичные артефакты (досье решений,
улики приёмки и внешние обзоры), не канон: на них ссылаются, их не дублируют.

## Куда за чем

| Задача | Куда |
|---|---|
| Понять, почему решение принято | [`TRACEABILITY.md`](TRACEABILITY.md) → D → Q |
| Быстро обозреть принятые решения | [`decisions/README.md`](decisions/README.md) (сводка, не канон) |
| Завести вопрос или решение | [`BRIEF.md`](BRIEF.md) §4–5 |
| Решить, нужна ли задача по коду | [`BRIEF.md`](BRIEF.md) §5.3 (сверка с кодом) → [`tasks/README.md`](tasks/README.md) |
| Понять, что и как реализовано | [`features/README.md`](features/README.md) |
| Взять задачу по коду | [`tasks/README.md`](tasks/README.md) |
| Понять, кто из команды агентов что делает | [`../AGENTS.md`](../AGENTS.md) §Рабочая группа агентов |
| Проверить требования и DoD | [`../AGENTS.md`](../AGENTS.md) §Сборка; процесс приёмки — [`.opencode/rules/dispatch-loop.md`](../.opencode/rules/dispatch-loop.md) |

## Журнал и связи

Журнал Q/D — единственный источник контекста решений: [`questions/`](questions/)
(Q → D) и [`decisions/`](decisions/) (D → контекст); связи и жизненный цикл —
[`TRACEABILITY.md`](TRACEABILITY.md)
([D63](decisions/D63-journal-index-lifecycle.md)). Миграция архива завершена
([D61](decisions/D61-archive-removal.md)); правила ведения журнала —
[`BRIEF.md`](BRIEF.md).
