# Приёмка: r16 — T-15/C11 (B2 профиль-флаг, D94/Q91)

**Дата:** 2026-10-03
**Проверка:** сервисная операция r16 (служебная зона: `.opencode/plugins/token-guard.ts`,
`.opencode/scripts/token-guard-test.mjs`, `AGENTS.md`, журнал Q91/D94, карточка T-15).
**Версия:** `develop` = `origin/develop`, HEAD `92bf91c` (r15) + незакоммиченное рабочее дерево.
**Режим:** адресная сверка + прогон теста профилей + адресный `cargo test --test docs_journal`
(F74) + `agents-perms.mjs`; полный DoD не требуется (D50 — `src/**`/`tests/**`/`Cargo.toml`
не тронуты).
**Вердикт:** accepted (P1/P2/P3 нет)

## Прогоны

| Проверка | Команда / способ | Результат |
|---|---|---|
| Тест профилей | `node .opencode/scripts/token-guard-test.mjs` | **недоступен `validator`** — `permission.rejected` (by design; нет права, см. тех. замечание) |
| Адресный журнал | `cargo test --test docs_journal` | **14 passed / 0 failed** (1.06s) |
| Права | `node .opencode/scripts/agents-perms.mjs` | ok, **«11 из 18»** |
| Границы (код) | `git diff --numstat -- src tests Cargo.toml opencode.json` | пусто |
| Гигиена диффа | `git diff --check` | пусто |
| Периметр | `git status --porcelain -uall` | 9 M + 4 ?? строго по списку операции |

## D94 ↔ факт (сверено чтением)

- `token-guard.ts:131` — `B2_PROFILE: "off" | "codemode" | "mcp" = "off"` ✓ (дефолт `off`).
- `:134-138` `B2_PREFIXES_CODEMODE`; `:141-145` `B2_PREFIXES_MCP` — идентичны;
  правила совпадают с D94 п.2 и §3.2 `wave0-token-hygiene.md` (docs-writer/git —
  `rust-analyzer`/`rust_analyzer`/`credo`; analyst — `rust-analyzer`/`rust_analyzer`;
  lead — отсутствует) ✓.
- `:148-151` `B2_PROFILES`; `:154-155` `activeB2Prefixes() = B2_PROFILES[B2_PROFILE] ?? {}`
  → `off` no-op ✓.
- `:291-299` хук `session.context` читает активный профиль (`activeB2Prefixes()[agent]`) ✓.
- B1 не изменён: `OUTPUT_LIMIT` (:44), `sliceText` (:70) на месте; `execute.after`-хук
  без связи с профилем ✓.
- Старой `B2_PREFIXES[agent]` нет (тест `:118` подтверждает; grep по плагину — чисто) ✓.

## P2 аудита (закрыт)

`token-guard-test.mjs:88-115` — добавлен `parseTable()` и assert **равенства**
`JSON.stringify(actual) === JSON.stringify(RULES[profile])` для
`B2_PREFIXES_CODEMODE`/`B2_PREFIXES_MCP` (:111-114). Ранее проверялось лишь
наличие имён — теперь дрейф префиксов в плагине ломает тест (ложный PASS при
включении `codemode:false` устранён). Regex `parseTable` корректно разбирает
и цитируемые (`"docs-writer":`), и bare (`git:`) ключи; `[\s\S]*?\n};` режет
по первому закрытию таблицы. ✓

Наблюдение: прогон теста `validator` не имеет права выполнить (by design) —
равенство зеркала подтверждено чтением логики `parseTable` + сверкой таблиц
`:134-145` с `RULES:16-27` вручную; запись ленты r16 (PASS, 24 ok) служит
дополнением.

## Прочие пункты

- `AGENTS.md:303-305` — сноска о `B2_PROFILE` (`D94`), включая путь к тесту ✓.
- `D45` п.3 — **одна** пометка «Уточнено [D94]» (строки 39-42), дубль снят ✓ (P3 закрыт).
- Карточка T-15 `:226` — `C11` → «🟡 подготовлен (профиль-флаг, D94); включение —
  `codemode:false`; closeout не держит» ✓; фаза E `:161-164` не тронута ✓.
- Журнал: Q91 «resolved by D94»; `questions/README.md:113`,
  `decisions/README.md:123`, `TRACEABILITY.md:95` (Q91→D94, `in work`, T-15 🚧) —
  согласованы, без дублей ✓.
- Границы: изменены только заявленные файлы (плагин/тест/`AGENTS.md`/`D45`/журнал/
  карточка/лента/память); `src/**`, `tests/**`, `Cargo.toml`, `opencode.json` не
  тронуты; временных файлов в `.opencode/scripts/` нет (glob пуст) ✓.
- Права: «инструкция ↔ права» — расхождений нет; фронтматтеры B2-ролей и
  `validator` не менялись ✓.

## Наблюдения

- Тест профилей `validator` не запустил (`permission.rejected`) — техническое
  замечание, не находка: по дизайну роль не имеет права на этот скрипт; приёмка
  опирается на чтение логики + запись сервисной сессии. **Рекомендация на будущее
  (вне этой операции):** при желании R2-прогона владельцем добавить
  `validator` право `node .opencode/scripts/token-guard-test.mjs` — отдельным
  решением, канон не правлю.

## Что проверено и ок

Плагин (B2 профиль-флаг, обе таблицы, `activeB2Prefixes`, no-op `off`, хук,
нетронутый B1, отсутствие старой `B2_PREFIXES[agent]`); тест профилей (зеркало
равенства, `off`, B1-регрессия); `AGENTS.md`; `D45` п.3; карточка T-15 `C11` и
фаза E; журнал Q91/D94, каталоги, TRACEABILITY; границы; права.

**Канон не правил; статусы карточки не менял (закрытие — `migrator`).**
