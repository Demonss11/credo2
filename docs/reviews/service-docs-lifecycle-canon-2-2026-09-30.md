# Приёмка: service-docs-lifecycle-canon-2 (операция №2 — дельта allowlist `git ls-files`/`git check-ignore`)

**Проверка:** дельта служебной зоны сервисной волны `service-docs-lifecycle`,
операция №2 — расширение allowlist: `validator` (+`git ls-files *`,
+`git check-ignore *`), `auditor` (+`git check-ignore *`); синхронизация
`review.md` §«Доступные команды»; второй пункт «Обновление 30.09.2026» в D49;
чекпойнты памяти `migrator`/`auditor`; лента.

**Версия:** `develop` @ `f28c8cb` (= `origin/develop` = `HEAD`) + рабочее
дерево, 30.09.2026.

**Вердикт:** **принято** (P1/P2/P3 нет; критичных проблем нет).

**P1:** — нет.
**P2:** — нет.
**P3:** — нет.

## Проверки

| Проверка | Команда | Результат |
|---|---|---|
| База | `git rev-parse develop origin/develop HEAD` | `f28c8cb` ×3 — совпадает с брифом |
| Границы | `git diff --stat -- src tests Cargo.toml AGENTS.md` | пусто |
| Фронтматтер `validator` | `git diff -- ./.opencode/agents/validator.md` | ровно +2 строки: `git ls-files *`, `git check-ignore *` (:31–32) |
| Фронтматтер `auditor` | `git diff -- ./.opencode/agents/auditor.md` | ровно +1 строка: `git check-ignore *` (:25) |
| `review.md` | `git diff -- ./.opencode/rules/review.md` | `validator` +`git ls-files`, +`git check-ignore` (:89); `auditor` +`git check-ignore` (:96) |
| Права ×2 | `node .opencode/scripts/agents-perms.mjs` (дважды) | `agents: 11 из 18`; у `validator` эффективно `allow:git ls-files *` + `allow:git check-ignore *`; у `auditor` `allow:git check-ignore *`; остальные роли без сдвигов; расхождений нет |
| Проба `validator` | `git ls-files .opencode/mail/service-docs-lifecycle.md` | путь возвращён — право работает |
| Проба `auditor` | `git check-ignore -v .opencode/mail/service-docs-lifecycle.md` | exit 1 (не игнорируется) — право работает |
| D49 | `git diff -- docs/decisions/D49-validator-branch-contains.md` | ровно +8 строк нового пункта «Обновление 30.09.2026 — права `git ls-files *`/`git check-ignore *` (дополняет п.1)» (:50–57), после первого «Обновления» (`rev-parse`, :42–49) |
| Границы `.opencode/**` | `git diff --stat -- ./.opencode` | 6 файлов дельты: `agents/{validator,auditor}.md`, `rules/review.md`, `memory/{migrator,auditor}.md`, `mail/service-docs-lifecycle.md` |
| Ссылка канона | `grep "^## Доступные команды" .opencode/rules/review.md` | `:86` — раздел существует, ссылки из фронтматтеров/брифов живые |
| Память роли | `git diff -- ./.opencode/memory/{migrator,auditor}.md` | чекпойнты append-only, чужих записей не переписано |

`cargo` не запускался (D50) — в пакете нет `src/**`, `tests/**`,
`Cargo.toml`.

## Что проверено и ок

- **Инструкция ↔ права:** фронтматтеры `validator.md` (:31–32) и `auditor.md`
  (:25) точно совпадают с новыми строками `review.md` (:89, :96); автопроверка
  прав ×2 даёт `11 из 18`, эффективные allow-строки соответствуют D49;
  остальные роли без сдвигов (списки `coder`/`tester`/`analyst`/`lead`/
  `docs-writer`/`migrator`/`researcher`/`git`/`rust-expert` совпадают в двух
  прогонах).
- **D49:** новый пункт на месте (конец «Следствий», :50–57), озаглавлен
  «дополняет п.1», read-only, разграничение ролей (`validator` — `ls-files` +
  `check-ignore`; `auditor` — только `check-ignore`) согласовано с фронтматтерами;
  ссылка `review.md` §«Доступные команды» живая (`review.md:86`); с п.1 (:27–29)
  и первым «Обновлением» (`rev-parse`, :42–49) не противоречит; поводы
  (`W8-config` 28.09.2026, `service-pm-numbering` 30.09.2026) совпадают с
  летной/брифом.
- **Границы:** `src/**`/`tests/**`/`Cargo.toml`/`AGENTS.md` не тронуты; в
  `.opencode/**` — только перечисленные файлы дельты + чекпойнты памяти
  (`migrator`, `auditor`) + лента. `memory/validator.md` дельтой не менялся.
  Вне периметра приёмки (иные операции волны, не оценивались): 68
  ` D docs/reviews/**`, карточки T-01/T-11/T-12/T-13/T-16,
  `docs/analysis/findings-registry.md` (итер. 9), `memorandum-W8-run5.md`
  (итер. 10).
- **Лента:** записи дельты идут по порядку (решение владельца → `migrator` D49
  → `auditor` аудит), факты с деревом сходятся (8 строк D49, строки
  фронтматтеров, строки `review.md`); аудит `auditor` — P1/P2/P3 нет.
- **Технические проблемы:** нет. Команды выполнены с `./`-префиксом одиночных
  путей (`git diff -- ./.opencode/...`) в обход известного квика движка прав
  по нескольким dot-путям под `--` (`review.md` §«Доступные команды»); среза
  вывода харнессом не было.

**Снимок приёмки:** `develop` @ `f28c8cb` + рабочее дерево (30.09.2026).
Правки после приёмки — по порогу существенности (`.opencode/rules/review.md`,
«Возврат на доработку»).
