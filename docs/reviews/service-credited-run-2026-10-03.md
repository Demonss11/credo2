# Приёмка: сервисная операция r14 — предикат «зачётного прогона» (T-15, C17)

- **Дата:** 2026-10-03
- **Роль:** `validator`
- **Тип:** адресная документная сверка + адресный `cargo test --test docs_journal`
  (F74); полный DoD не запускался (D50 — `src/**`, `tests/**`, `Cargo.toml`
  не менялись)
- **Версия:** `develop` = `origin/develop`, HEAD `eeac481` + рабочее дерево
  (пакет не закоммичен)
- **Вердикт:** принято (P1/P2/P3 нет)

## Предмет

Сервисная операция r14: определение «зачётного прогона» (чистота для
`F26`/`F27`/`F15`). Лента `.opencode/mail/service-mcp-ready-r14.md`
(«правки канона r14 — готово», «аудит», «migrator … P3-1/P3-2»); журнал
[Q89](../questions/Q89.md) → [D92](../decisions/D92-credited-run-predicate.md).

## Проверки

| Проверка | Команда / метод | Результат |
|---|---|---|
| Адресный `docs_journal` | `cargo test --test docs_journal` | **14 passed / 0 failed** (0.06s), в т.ч. `ids_are_unique_and_contiguous`, `questions_are_listed_in_catalog_and_traceability`, `traceability_tasks_exist_and_match_registry`, `traceability_lifecycle_matches_task_openness`, `no_addresses_to_removable_or_session_data` |
| Границы (код) | `git diff --numstat -- src tests Cargo.toml` | пусто |
| Границы (whitespace) | `git diff --check` | пусто |
| Периметр | `git status --porcelain` | только `.opencode/**` + `docs/**` + лента/память/state; `??` Q89/D92/лента r14 |
| Снимок | `git rev-parse HEAD` | `eeac481126b0ab4d981afd771958213b1dc93d20` |

## D92 ↔ факт

**Ядро (4 условия):** D92:31–37 ≡ `state-schema.md`:169–175 —

1. ведущий — `lead` (атрибуция верна);
2. нет упоров лимитов `steps` ни у одной роли;
3. нет незапланированных прерываний владельца — каждый `owner_response`
   соответствует `surface_to_user`, **стоявшему в плане участка** (`next[]`);
4. нет ручных восстановлений (пустые финалы, backfill, ручная реконструкция).

**Не-дефекты:** D92:38–40 ≡ `state-schema.md`:177–179 — плановые гейты,
`owner_override`-решения, обязательный rework `validator` (`-rN`), CCSN-шаг,
session-commit.

**Машинная проверка:** D92:41–43 ≡ `state-schema.md`:181–184 — `progress.yaml`
+ `session-analysis/` + `metrics-report.mjs`; вердикт — записью.

**Неретроактивность:** D92:44–45 ≡ `state-schema.md`:187–188 — зачёт только
прогонам после принятия решения; «одним зачётным прогоном».

**Ссылки F26/F27/F15:** D92:9–10, :95; `state-schema.md`:185–186.

**P3 аудита r14 — закрыты (оба, в схеме и в D92):**

- **P3-1 (путь):** D92:41–42 разнесены `.opencode/scripts/session-analysis/` и
  `.opencode/scripts/metrics-report.mjs`; `state-schema.md`:182–183 — то же.
  Оба целевых пути существуют (`metrics-report.mjs`; 7 файлов в
  `session-analysis/`, glob).
- **P3-2 (тавтология условия 3):** D92:33–35 и `state-schema.md`:171–173 —
  «`surface_to_user`, стоявшему в плане участка (`next[]`); ответов вне плана
  и „продолжай“ нет» — различающий признак назван.

## Журнал и границы

- Q89 (`resolved by D92`) ↔ D92 (`Resolves: Q89`); `questions/README.md:111`,
  `decisions/README.md:121`.
- `TRACEABILITY.md:93` — Q89: D92 / `in work` / T-15 🚧; колонка «Реализация» —
  `.opencode/rules/state-schema.md` (раздел «Зачётный прогон»).
- Карточка T-15: `C17` 🚧 (`:230`, `service-mcp-ready-r14, 03.10`); аннотации
  `B1-F26`/`B1-F27`/`B1-F15` (`:211–213`, ⏸ с данными 02.10).
- Реестр находок: F83/F84 (`:95–96`), уникальны, открыты, ссылаются на
  D58/D86/D91.
- Границы: изменены только `.opencode/rules/state-schema.md`,
  `docs/{questions,decisions}/**`, `docs/TRACEABILITY.md`, карточка T-15,
  findings-registry, лента/память; `src/**`, `tests/**`, `Cargo.toml`,
  `opencode.json`, иной канон — не тронуты. `.credo/sandbox.json` — вне пакета
  (F83). Номера строк и адресов `docs/analysis/**` в каноне/решении нет.

## Что проверено и ок

- D92 ↔ `state-schema.md` §«Зачётный прогон» — все блоки совпадают.
- P3-1/P3-2 аудита r14 закрыты в обоих местах; целевые пути существуют.
- Журнальные связи (Q89↔D92, каталоги, TRACEABILITY, карточка, findings).
- Границы пакета; `.credo/sandbox.json` — вне пакета (F83).
- Адресный `docs_journal` — 14/0.

## P1/P2/P3

Критичных проблем нет.
