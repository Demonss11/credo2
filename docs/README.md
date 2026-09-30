# docs/ — документация прототипа CREDO

Спецификация, язык, требования, журнал вопросов и решений, задачи по коду — всё
для `prototypes/credo2`. Точка входа — этот файл; правила ведения журнала —
[`../.opencode/rules/journal.md`](../.opencode/rules/journal.md).

## Карта

```
docs/
  README.md          # этот файл: карта
  TRACEABILITY.md    # связи и жизненный цикл Q → D → задача → реализация
  SPECIFICATION.md   # целевая архитектура; решения — журнал decisions/
  GRAMMAR.md         # язык v0.1 (EBNF включён)
  CHANGELOG.md       # только кодовые изменения продукта
  questions/         # Qn.md, … + README.md (сводка вопросов)
  decisions/         # Dn-….md + README.md (сводка решений)
  features/          # Gherkin-требования + README (статусы/приоритеты/счётчики)
  tasks/             # реестр задач по коду (сводка + карточки T-XX)
  analysis/          # досье analyst по задачам (улики решения, не канон)
  reviews/           # отчёты приёмки (улики, не канон; пишет validator)
  research/          # внешние обзоры (не канон; пишет researcher)
```

`analysis/`, `reviews/` и `research/` — рабочие артефакты (досье решений, улики
приёмки, внешние обзоры), не канон: канон на их файлы не ссылается — факты
живут в Q/D, задачах, фичах; неактуальные файлы удаляются (архив —
git-история; [D65](decisions/D65-reference-policy.md)).

## Куда за чем

| Задача | Куда |
|---|---|
| Понять, почему решение принято | [`TRACEABILITY.md`](TRACEABILITY.md) → D → Q |
| Быстро обозреть принятые решения | [`decisions/README.md`](decisions/README.md) (сводка, не канон) |
| Завести вопрос или решение | [`../.opencode/rules/journal.md`](../.opencode/rules/journal.md) §4–5 |
| Решить, нужна ли задача по коду | [`../.opencode/rules/journal.md`](../.opencode/rules/journal.md) §5.3 (сверка с кодом) → [`tasks/README.md`](tasks/README.md) |
| Понять, что и как реализовано | [`features/README.md`](features/README.md) |
| Взять задачу по коду | [`tasks/README.md`](tasks/README.md) |
| Понять, кто из команды агентов что делает | [`../AGENTS.md`](../AGENTS.md) §Рабочая группа агентов |
| Проверить требования и DoD | [`../AGENTS.md`](../AGENTS.md) §Сборка; процесс приёмки — [`.opencode/rules/dispatch-loop.md`](../.opencode/rules/dispatch-loop.md) |

Канон — таблица в [`../AGENTS.md`](../AGENTS.md) §Документы и решения; политика
«один факт — один канон» — [`D60`](decisions/D60-docs-ownership-sync.md).

## Журнал и связи

Журнал Q/D — единственный источник контекста решений: [`questions/`](questions/)
(Q → D) и [`decisions/`](decisions/) (D → контекст); связи и жизненный цикл —
[`TRACEABILITY.md`](TRACEABILITY.md).
