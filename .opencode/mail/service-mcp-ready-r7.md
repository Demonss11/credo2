# service-mcp-ready-r7 — лента операции: T-15, CC Safety Net (перенос)

Открыта: 02.10.2026. **Сервисная операция** (продолжение программы T-15; после
C10). Предмет: перенос CC Safety Net (кандидат B0-i5, вердикт 🟢) — решение
владельца 28.09.2026 «позже, отдельным шагом; пресет **standard**»; сейчас —
«делаем CC Safety Net».

**Входы:** [`wave0b-probes.md`](../../docs/tasks/T-15-mcp-ready-process/wave0b-probes.md)
§B0-i5 (v2.4.11; форма `opencode plugin add cc-safety-net@latest` +
`options.shell = "powershell"`; standard; вопрос paranoid — в C);
[`wave0b-report.md`](../../docs/tasks/T-15-mcp-ready-process/wave0b-report.md)
§4/§6; текущий контур C10 (4 правила + `wave0-guard`).

**Рамка:** правка — служебная зона (глобальный конфиг + npm-плагин); канон
агентов не правится; `cargo` не запускается (D50); улики — `target/wave0b-csn/`.

## сервисная сессия · 02.10.2026 · открытие

- Текущее состояние: `opencode plugin list` — token-guard, wave0-guard,
  wave0-observe (local); CC Safety Net не установлен; `~/.cc-safety-net/`
  существует (compile-cache + logs — след пробы B0-i5).
- План: бэкап глобального конфига → `opencode plugin add cc-safety-net@latest`
  → `options.shell = "powershell"` (object-форма) → reload → пробы
  (dangerous/allow/аудит) → `auditor` (перенос после аудита, B0-i5) →
  `validator` → гейт → коммит записей.

## сервисная сессия · 02.10.2026 · установка и проверки — готово

- **Установка:** `opencode plugin add cc-safety-net@latest` → в глобальном
  конфиге `plugins` (object-форма с `options.shell = "powershell"`);
  `opencode plugin list` → `cc-safety-net 2.5.1`; лог — `loading plugin`
  (без WARN); бэкап конфига — `target/wave0b-csn/global-before.jsonc`,
  после — `global-after.jsonc`.
- **Диагностика:** `ccsn doctor` — OpenCode **Detected | Configured |
  Verified**, self-test 3/3; `ccsn status` — level **standard**, policy
  `~\.cc-safety-net\policy.json`; CLI — в кэше
  (`~\.cache\opencode\npm\cc-safety-net@latest\…\.bin\ccsn.cmd`), на PATH нет.
- **Живая проба** (scratch git-репо): `git clean -fdx` → **BLOCKED by CC
  Safety Net** (`Rule: git.clean-force`, intent use_alternative, подсказка
  `git clean -n`); `echo CSN-OK` — allow; модель сама использовала безопасный
  `git clean -ndx`; файл `untracked.txt` не удалён.
- **Аудит:** `~\.cc-safety-net\logs\<проект>\<год-месяц>\<дата>-<сессия>.jsonl`
  — `decision/ruleId/intent/cwd` для deny и allow; копия —
  `target/wave0b-csn/logs/` (включая след B0-i5, v2.4.11).
- **Наблюдения:** (1) CCSN покрывает то, чего нет в C10-контуре (например,
  `git clean -fdx`); (2) где пересекается — первыми срабатывают наши policies
  (финальны: `git push *--force*`, `read:*.env`); (3) нюанс: policy
  `shell:git push *--force*` матчит и `--force-with-lease` (рекомендацию CCSN)
  — в CREDO заблокирован: строгость, кандидат на уточнение; (4) аудит CCSN
  пишет и allow-строки (прунинг есть — `.last-prune`).
- **Дальше:** `auditor` (перенос B0-i5 — после аудита) → `validator` → гейт →
  коммит записей.

## auditor · 02.10.2026 · аудит переноса — P1 нет; P2/P3 (см. секцию роли)

- **P1** — нет. **P2-1** (отчёт §4 CCSN — самопротиворечие) и **P3** (строка
  guard в B0-own отчёте — CCSN смешан с C10) — **закрыты** правками `migrator`.
  **P2-2** — `.opencode/agents/auditor.md:11` `read deny **/target/**`
  инертно для корневого `target/**` — решение владельца: **убрать правило**
  (отдельным шагом канона).
- Подтверждено: 4 правила C10 целы; `plugins` object-форма; doctor OpenCode
  Detected|Configured|Verified, self-test 3/3, v2.5.1; проба deny/allow +
  аудит; границы чисты; `agents-perms.mjs` ×2 → `11 из 18`.

## validator · 02.10.2026 · принято (CCSN)

- Вердикт: **принято, P1/P2/P3 нет**; отчёт
  `docs/reviews/service-ccsn-2026-10-02.md`, квитанция `service-ccsn`
  (iteration 1, accepted) в `receipts.yaml`; `cargo` не запускался (D50).

## сервисная сессия · 02.10.2026 · гейт пакета CCSN пройден

- Владелец подтвердил (**question**): «Коммит + push develop»; по P2-2 —
  «Убрать правило» (правка `auditor.md` — отдельным шагом канона: правка →
  аудит → приёмка → коммит).
- Пакет — 12 путей + запись роли `git` (F43) = 13: `??` —
  `.opencode/mail/service-mcp-ready-r7.md`,
  `docs/reviews/service-ccsn-2026-10-02.md`; `M` —
  `.opencode/mail/service-mcp-ready-r6.md` (записи полигона),
  `.opencode/memory/{auditor,migrator,service,validator}.md`,
  `.opencode/state/current/receipts.yaml`,
  `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own-report.md,wave0b-own.md,wave0b-report.md}`,
  `.opencode/memory/git.md`.
- Сообщение: `chore(process): T-15 — CC Safety Net включён (standard) + полигон
  B0-own закрыт (записи); приёмка`.
- База: `develop` = `origin/develop` = `38800cb`; master не трогаем.
- Следующее действие — `dispatch git`; затем правка `auditor.md` (P2-2).

## git · 02.10.2026 · готово (пакет CCSN)

- Коммит **`606055f`** (13 файлов: 11 `M` + 2 `A`; +465/−6) → push
  `38800cb..606055f` в `origin/develop`; ветки не создавались; master не
  трогался; дерево чистое.

## сервисная сессия · 02.10.2026 · P2-2: снятие инертного правила `auditor.md`

- Решение владельца (question): «Убрать правило» — `.opencode/agents/auditor.md`
  строка 11 `read deny **/target/**` снимается: паттерн инертен для корневого
  `target/**` (требует сегмент перед `target/`), а чтение улик `target/**`
  необходимо протоколу аудита (B0-own/C10-аудиты им пользовались).
- Протокол канона: правка (сервисная сессия) → независимый аудит `auditor` →
  приёмка `validator` → гейт → коммит.
- Дальше: правка файла → `auditor`.

## сервисная сессия · 02.10.2026 · P2-2: применение (reload)

- Правка внесена (снята строка `read deny **/target/**`); **`opencode reload`
  выполнен** (02.10.2026, после правки) — фронтматтер применён; runtime:
  чтение `target/**` доступно (подтверждено аудитом).
- Дальше: `validator` (приёмка) → гейт → коммит.

## сервисная сессия · 02.10.2026 · P2-2: гейт пройден

- `auditor`: P1/P2 нет; P3 (запись о reload) закрыт; «инструкция ↔ права» —
  расхождений нет (`agents-perms.mjs` ×2 → `11 из 18`).
- `validator`: **принято** (P1/P2/P3 нет; полный DoD — 135/0); отчёт
  `docs/reviews/service-permissions-auditor-target-2026-10-02.md`, квитанция
  `service-permissions-auditor-target` (iteration 1, accepted) в `receipts.yaml`.
- Владелец подтвердил (**question**): «Коммит + push develop».
- Пакет — 6 путей + запись роли `git` (F43) = 7: `M` —
  `.opencode/agents/auditor.md`, `.opencode/mail/service-mcp-ready-r7.md`,
  `.opencode/memory/{auditor,validator}.md`,
  `.opencode/state/current/receipts.yaml`; `??` —
  `docs/reviews/service-permissions-auditor-target-2026-10-02.md`;
  + `.opencode/memory/git.md`.
- Сообщение: `chore(agents): auditor — снято инертное правило read deny
  **/target/** (P2-2); приёмка`.
- База: `develop` = `origin/develop` = `606055f`; master не трогаем.
- Следующее действие — `dispatch git`.

## git · 02.10.2026 · готово (пакет P2-2)

- Коммит **`e861bba`** (7 файлов: 6 `M` + 1 `A`; +246/−1) → push
  `606055f..e861bba` в `origin/develop`; ветки не создавались; master не
  трогался; дерево чистое.

## сервисная сессия · 02.10.2026 · CCSN + P2-2 — итог

- **CC Safety Net включён** (коммит `606055f`): `cc-safety-net` 2.5.1,
  глобальный конфиг (object-форма, `options.shell=powershell`), пресет
  **standard**; doctor — OpenCode Detected|Configured|Verified, self-test 3/3;
  проба — `git clean -fdx` BLOCKED (`git.clean-force`), allow-поток ок; аудит
  `~\.cc-safety-net\logs` (копия `target/wave0b-csn/`); параллельно закоммичены
  записи о закрытии полигона B0-own.
- **P2-2** (коммит `e861bba`): снято инертное правило `read deny **/target/**`
  в `.opencode/agents/auditor.md`; аудит — расхождений нет; приёмка — принято
  (полный DoD 135/0).
- **Открытые пункты:** вопрос paranoid CCSN (`Remove-Item -Recurse -Force` —
  standard не блокирует; в CREDO закрыт `wave0-guard`); нюанс
  `--force-with-lease` под policy `git push *--force*` (кандидат на уточнение);
  улики/логи CCSN — в `target/wave0b-csn/`.
- Дальше по T-15: «чистый» S-прогон (F26/F27) или фаза C (C1).

## сервисная сессия · 02.10.2026 · F63 — проверка и закрытие

- Вопрос владельца: задача под F63 или закрыть тут. **Проверка:** CCSN
  (глобальный, standard) анализирует shell: `ccsn explain "Get-Content .env"`
  и `"Get-Content …\.ssh\known_hosts"` → **BLOCKED** (`secret-protection`);
  живая проба в полигоне `wave0b-c10` (без плагинов CREDO): `Get-Content .env`
  → **BLOCKED by CC Safety Net** (`secret.basename.env`, hard_stop).
- **Вывод:** остаток F63 (shell-чтение `.env`/`.ssh` вне CREDO) **закрыт
  включением CCSN**; в CREDO дублируется `wave0-guard`. Задача не требуется —
  статус F63 в реестре → «закрыт 02.10.2026».
- Улика: `target/wave0b-csn/f63-shell-secret-deny.jsonl`.

## сервисная сессия · 02.10.2026 · гейт F63 пройден

- Владелец подтвердил (**question**): «Коммит + push develop».
- Пакет — 4 пути + запись роли `git` (F43) = 5: `M` —
  `docs/analysis/findings-registry.md` (F63 закрыт),
  `.opencode/mail/service-mcp-ready-r7.md`,
  `.opencode/memory/{migrator,service}.md`; + `.opencode/memory/git.md`.
- Сообщение: `docs(T-15): F63 закрыт — CCSN standard покрывает shell-чтение
  секретов глобально`.
- База: `develop` = `origin/develop` = `e861bba`; master не трогаем.
- Следующее действие — `dispatch git`.

## git · 02.10.2026 · готово (пакет F63)

- Коммит **`ef39b33`** (5 файлов: 5 `M`; +111/−1) → push `e861bba..ef39b33`
  в `origin/develop`; ветки не создавались; master не трогался; дерево чистое.

## сервисная сессия · 02.10.2026 · F63 закрыт (итог)

- **Ответ на вопрос владельца:** задача под F63 не нужна — остаток закрыт
  включением CCSN; реестр обновлён (F63 → «закрыт 02.10.2026»), коммит
  `ef39b33`.
- **Итог контура страховок (02.10.2026):** C10 (4 точных policy +
  `wave0-guard`) + CC Safety Net standard + `token-guard`; F62/F63/F64
  закрыты, F65/F66 — учётные (задач не требуют).
- Дальше по T-15: «чистый» S-прогон (F26/F27) или фаза C (C1).

## сервисная сессия · 02.10.2026 · F65/F66 — решение: учёт

- Владелец (question): «Оставить как учёт» — задач не заводить, реестр не
  менять. Триггеры: **F65** — проверка на новой версии OpenCode (воспроизводится
  ли churn MCP после `reload`); **F66** — пункты при своих входах (transform-
  плагин в служебной зоне; доработка `metrics-report`/`wave0-observe`;
  косметика npm-шима), часть уже обойдена/учтена в протоколах.
- Дальше по T-15: «чистый» S-прогон (F26/F27) или фаза C (C1).

## сервисная сессия · 02.10.2026 · F65 закрыт (факт-чек версии)

- Уточнение владельца: «мы уже на новой версии» — факт-чек: **v2.0.22
  (channel=latest)**; churn MCP после `reload` наблюдался **на ней же**
  (сегодня, дважды) — «ждать новой версии» нечего.
- **Правило практики:** `reload` — только по необходимости; после — проверять
  MCP-каталог; в temp-полигоне без нужды не вызывать (эффект на общий сервис);
  при будущем обновлении — ре-проверка. Записано в память сервиса; F65 →
  «закрыт 02.10.2026» (реестр).
- **F66** — остаётся учётной (решение владельца от 02.10: задач не заводим).
- Владелец подтвердил (**question**): «Коммит + push develop».
- Пакет — 4 пути + запись роли `git` (F43) = 5: `M` —
  `docs/analysis/findings-registry.md` (F65 закрыт),
  `.opencode/mail/service-mcp-ready-r7.md`,
  `.opencode/memory/{migrator,service}.md`; + `.opencode/memory/git.md`.
- Сообщение: `docs(T-15): F65 закрыт — reload-эффект подтверждён на 2.0.22,
  принято в практику`.
- База: `develop` = `origin/develop` = `ef39b33`; master не трогаем.
- Следующее действие — `dispatch git`.

## migrator · 02.10.2026 · готово (CCSN включён: карточка/отчёт)

- Сделано: две мини-правки по включению CC Safety Net. `docs/tasks/T-15-mcp-ready-process/README.md`, строка **C10** сводной таблицы: «CC Safety Net — позже» → «CC Safety Net — включён 02.10 (standard, `cc-safety-net` 2.5.1)» (остальной текст строки сохранён). `docs/tasks/T-15-mcp-ready-process/wave0b-report.md` §4, пункт про CC Safety Net: в конец дописано «— **исполнено 02.10.2026** (v2.5.1; пресет standard; перенос — после аудита)». Иных строк/файлов не трогал.
- Проверки: `git diff -- <2 файла>` → ровно 2 правки (C10 1+/1−; report 1+/1−); `git diff --check` по ним пусто; `git diff --numstat -- src tests Cargo.toml` пусто; `cargo` не запускал (D50).
- Дальше / риски: коммит не делал (гейт впереди).

## auditor · 02.10.2026 · готово (аудит переноса CCSN)

- **Аудит:** перенос CC Safety Net (T-15; `service-mcp-ready-r7`; B0-i5 «перенос — после аудита»). Предмет — глобальный конфиг (копии `target/wave0b-csn/global-before|after.jsonc`), установка/диагностика (`ccsn-status.txt`, `ccsn-doctor.txt`), пробы (`target/wave0b-csn/logs/**`), записи (лента r7, карточка T-15, `wave0b-report.md` §4, память `service`), границы.
- **Вердикт: P1 нет. P2 — 2 (одна вне диффа переноса), P3 — 1 (вне диффа).**
- **P2:** `docs/tasks/T-15-mcp-ready-process/wave0b-report.md:87` — правка этого шага даёт противоречие: «перенос — после `auditor` — **исполнено 02.10.2026** (…; перенос — после аудита)». Хвост «(перенос — после аудита)» повторяет снятое условие → улика противоречит себе. Правка: убрать хвост.
- **P2 (права, вне диффа переноса):** `.opencode/agents/auditor.md:11` `read deny **/target/**` — правило **инертно** для корневого `target/**`: чтение `target/wave0b-csn/**` этой сессией прошло, тогда как `read .git/**` (стр. 12) отклонён. Последствие: страховка не страхует; а если бы сработала — заблокировала бы чтение улик аудита в `target/`, требуемое протоколом (B0-own, CCSN). Нужно уточнение владельца: чинить паттерн или снять правило. Пре-существующее, не из переноса.
- **P3 (вне диффа):** `docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md:58` — CCSN-строку связывает с C10-артефактами («CC Safety Net — … исполнено: перенесён, 4 точных правила глобально + плагин `wave0-guard`»); фактический перенос CCSN — запись `plugins` в глобальном конфиге, а 4 правила + guard — C10. Кандидат на уточнение при следующей правке отчёта.
- **Проверки (подтверждено):** before/after — C10 4 правила дословно целы; после — добавлен только блок `plugins` (object-форма `package` + `options.shell=powershell`). `ccsn-status.txt`: level standard, policy `~\.cc-safety-net\policy.json`. `ccsn-doctor.txt`: OpenCode Detected|Configured|Verified, self-test 3/3, v2.5.1, preset standard. Проба (scratch-сессия): deny `git clean -fdx` (`git.clean-force`, intent `use_alternative`) + allow `echo CSN-OK` + allow `git clean -ndx`; аудит пишет allow-строки; след B0-i5 (v2.4.11, `.../wave0b-probe/2026-09/`) сохранён.
- **Границы (независимо):** `git status --porcelain` → изменены только `docs/tasks/T-15-…/{README,wave0b-report,wave0b-own-report,wave0b-own}.md`, `.opencode/mail/service-mcp-ready-r6.md`, `.opencode/memory/{migrator,service}.md`; `?? mail/r7`. `git diff --numstat -- src tests Cargo.toml AGENTS.md opencode.json` пусто; `.opencode/agents`/`rules` в `status` нет; `grep` CCSN по `src`/`tests`/`opencode.json` — 0. Цело.
- **Машинно:** `node .opencode/scripts/agents-perms.mjs` ×2 → `11 из 18`, состав стабилен; фронтматтеры не менялись (reload не требуется).
- **Нюансы вердикта (приняты, не находки):** policy `shell:git push *--force*` матчит и `--force-with-lease` — кандидат на уточнение (уже в ленте/памяти); аудит-журнал растёт (allow-строки; прунинг `.last-prune` есть); `ccsn` не на PATH (путь в кэше — корректен); `doctor` печатает Node-ассерт (`ccsn-doctor.txt:103–109`) при exit 1 — косметика.
- **Техническая проблема:** `shell` роли — узкий allowlist; `Get-ChildItem …` и `git diff -- <≥2 dot-путей>` отклонены (ожидаемо); прямые `git status`/`git diff <1 путь>`/`node agents-perms.mjs` доступны. Бюджет: ~20 чтений (вкл. каталоги/логи), 0 агентных `rg`-обёрток; ≤ 15 файлов каталогов не считая.
- **Дальше:** `validator` → гейт → коммит записей. Правки P2/P3 — в пакет той же сервисной сессии (одна строка каждая); P2-права (`auditor.md:11`) — отдельным решением владельца.

## migrator · 02.10.2026 · P2/P3 аудита CCSN — готово

- Сделано (две строки по аудиту, иных изменений нет): (P2-1) `docs/tasks/T-15-mcp-ready-process/wave0b-report.md` §4 — хвост CCSN-пункта приведён к непротиворечивому: «перенос — **исполнено 02.10.2026** (v2.5.1; пресет standard; аудит переноса — `service-mcp-ready-r7`).» (снят повтор условия «после `auditor`»/«после аудита»). (P3) `docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md` — строка guard (P2): CCSN отделён от C10-артефактов: «… — **CC Safety Net включён 02.10.2026 (v2.5.1, standard; `service-mcp-ready-r7`)**; C10 — 4 точных правила глобально + плагин `wave0-guard` (якорные правила)».
- Проверки: `git diff -- <2 файла>` → целевые правки (guard P2 1+/1−; report CCSN-строка 1+/1−); `git diff --check` по обоим пусто; `git diff --numstat -- src tests Cargo.toml` пусто; `cargo` не запускал (D50).
- Нюанс: в `wave0b-own-report.md` в дереве есть **чужой** ханк («откат полигона» `[ ]`→`[x]`, стр. ~87) — не моя правка, существовала до сессии; мой дифф по этому файлу — только строка guard (P2).
- Дальше / риски: P2-права (`.opencode/agents/auditor.md:11`, `read deny **/target/**`) — отдельным решением владельца (вне этого пакета). Коммит не делал (гейт впереди).

## validator · 02.10.2026 · принято (CCSN)

- **Вердикт:** принято, P1/P2/P3 нет. Отчёт `docs/reviews/service-ccsn-2026-10-02.md`;
  квитанция `accepted` (`service-ccsn`, iteration 1) в `receipts.yaml`.
- **База:** `develop` = `origin/develop` = `HEAD` = `38800cb` (ветки нет, прямая
  правка). Снимок пакета: 8 `M` + `?? .opencode/mail/service-mcp-ready-r7.md`.
- **CCSN:** before→after конфига = ровно блок `plugins` (object-форма,
  `options.shell=powershell`); 4 C10-правила дословно целы. Улики: `ccsn-doctor`
  Detected|Configured|Verified / self-test 3/3; `ccsn-status` standard; логи —
  deny `git clean -fdx` (`git.clean-force`, `use_alternative`), allow `echo CSN-OK`,
  allow `git clean -ndx`; след B0-i5 (v2.4.11) сохранён.
- **Записи:** карточка C10, `wave0b-report.md` §4 (без самопротиворечия),
  `wave0b-own-report.md` §3 (CCSN ≠ C10) + §5 (полигон), `wave0b-own.md`
  §«Откат полигона», память `{service,migrator,auditor}` — согласованы.
- **Границы:** `src/tests/Cargo.toml/AGENTS.md/opencode.json/agents|rules` пусто;
  `git diff --check` пусто (CRLF-warning только на r6); `agents-perms.mjs` → `11 из 18`.
- **Известный пункт (вне пакета):** P2-2 `.opencode/agents/auditor.md:11`
  `read deny **/target/**` инертно — решение владельца.
- **`cargo` не запускался** (D50; продуктовый код не затронут).

## git · 02.10.2026 · план пакета (CCSN)

- Подтверждение сверено по ленте — §«гейт пакета CCSN пройден» (:71–87), ответ
  владельца «Коммит + push develop». База: `develop` = `origin/develop` = `HEAD`
  = `38800cb`; ветки нет, master не трогаем.
- Снимок: 9 `M` + 2 `??` = ровно пакет 12 путей (лента r7 и отчёт CCSN — новые)
  + этот чекпойнт `memory/git.md` (F43) = 13. `wave0b-own.md` — в пакете (входит
  в 12).
- К коммиту — 13 путей (`./`-префикс, без `--`): `.opencode/mail/service-mcp-ready-r7.md`,
  `docs/reviews/service-ccsn-2026-10-02.md`, `.opencode/mail/service-mcp-ready-r6.md`,
  `.opencode/memory/{auditor,migrator,service,validator,git}.md`,
  `.opencode/state/current/receipts.yaml`,
  `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own-report.md,wave0b-own.md,wave0b-report.md}`.
- Сообщение: `chore(process): T-15 — CC Safety Net включён (standard) + полигон
  B0-own закрыт (записи); приёмка`. Ожидаемая staged-сверка: **11 `M` + 2 `A`**.
- Осталось: `add` 13 путей → `diff --cached --name-status` → `commit` →
  `push origin develop`. Хеши — в ответе `lead` (не здесь). После push в
  отслеживаемые файлы не писать (F43).

## auditor · 02.10.2026 · готово (аудит P2-2 auditor.md)

- **Аудит:** снятие правила `read deny **/target/**` из `.opencode/agents/auditor.md`
  (решение владельца «Убрать правило»; прецедент — P2-аудит CCSN, `auditor.md:11`).
- **Вердикт: P1 нет. P2 нет. P3 — 1 (вне диффа).** Снятие подтверждено;
  расхождений «инструкция ↔ права» нет.
- **Дифф (подтверждено):** `git diff -- .opencode/agents/auditor.md` — ровно одна
  удалённая строка `- { action: read, resource: "**/target/**", effect: deny }`
  (из блока read после `edit`); иных строк не тронуто, тело/остальной фронтматтер
  целы. `git diff --stat` → `auditor.md | 1 -` (+ чужой ханк ленты).
- **Обоснование (подтверждено):** паттерн `**/target/**` требует сегмент перед
  `target/` → для корневого `target/**` инертен (чтение `target/wave0b-*` этой
  сессией проходит). После снятия открыт только корневой `target/**` (уже был
  доступен, улики аудита `target/wave0b-own-i*`/`target/wave0b-csn/**` читаемы —
  протоколом востребованы); остальные deny сохранены: `.git/**`,
  `**/node_modules/**`, `Cargo.lock`, `.credo/**`, shell-allowlist (`*` deny).
  Ничего сверх прежнего не открыто.
- **«Инструкция ↔ права»:** ссылок на снятое правило в каноне нет — в
  `.opencode/rules/**`, `AGENTS.md` и теле `auditor.md` совпадений
  `**/target/**`/`target` нет (остальные `**/target/**` — фронтматтеры других
  ролей, не затронуты). `node .opencode/scripts/agents-perms.mjs` ×2 → стабильно
  `agents: 11 из 18`; `auditor` shell-allowlist без изменений; список команд
  `review.md:95–96` совпадает с фронтматтером (read-права в него не входят).
- **Границы (независимо):** `git status --porcelain` → только
  `.opencode/agents/auditor.md` и `.opencode/mail/service-mcp-ready-r7.md`;
  `git diff --numstat -- src tests Cargo.toml` пусто; `git diff --check` пусто.
  Правка `auditor.md` — единственная в служебной зоне; `cargo` не запускался (D50).
- **P3 (вне диффа):** в ленте нет явной записи о `opencode reload` для этой
  правки: единственные `reload` — стр. 23 (CCSN-проба) и стр. 120 («reload не
  требуется» — для пакета CCSN **до** этой правки). Фактически правка применена
  (runtime: чтение `target/wave0b-csn/` и `target/wave0b-own-i*` доступно, deny
  снят). Правка одной строки: дописать в запись сервисной сессии (стр. ~103)
  «→ `opencode reload`» либо явно «reload выполнен». Последствие — только
  прослеживаемость; право/поведение корректны.
- **Машинно/технически:** `git diff -- <1 dot-путь>` прошёл; `git diff -- <2
  dot-путей>` и `Get-ChildItem` — отклонены (узкий allowlist, ожидаемо). Бюджет:
  ~12 файлов чтения, 5 rg, ~8 shell.
- **Дальше:** приёмка `validator` → гейт → коммит (P3 — по усмотрению пакета).

## validator · 02.10.2026 · принято (P2-2 auditor.md)

- **Вердикт: принято, P1/P2/P3 нет.** Отчёт
  `docs/reviews/service-permissions-auditor-target-2026-10-02.md`; квитанция
  `accepted` (`service-permissions-auditor-target`, iteration 1) в `receipts.yaml`.
- **Предмет:** `git diff .opencode/agents/auditor.md` → ровно одна удалённая строка
  `- { action: read, resource: "**/target/**", effect: deny }`; тело и остальной
  фронтматтер целы; прочие deny сохранены. Ссылок на снятое правило в
  `.opencode/rules`/`AGENTS.md`/теле нет.
- **Аудит:** `auditor` — P1 нет, P2 нет; P3 (нет явной записи `reload`) **закрыт**
  записью ленты r7 §«P2-2: применение (reload)» (:105–110). Память `auditor` — append.
- **Права:** `agents-perms.mjs` ×2 → `11 из 18`; `review.md:95–96` совпадает.
- **Границы:** только `auditor.md` + лента r7 + память `auditor` (append);
  `src/tests/Cargo.toml/AGENTS.md` пусто; `git diff --check` пусто.
- **DoD (R2):** `cargo fmt --check` pass; `clippy -D warnings` pass; `cargo test --all`
  — **135 passed / 0 failed**. `cargo` запускался для полноты (D50 допускал пропуск:
  продуктовый код не затронут). База `develop` = `origin/develop` = `HEAD` = `606055f`.

## git · 02.10.2026 · план пакета (P2-2)

- Подтверждение сверено по ленте — §«P2-2: гейт пройден» (:112–129), ответ
  владельца «Коммит + push develop». База: `develop` = `origin/develop` = `HEAD`
  = `606055f`; ветки нет, master не трогаем.
- Снимок: 5 `M` + 1 `??` = ровно пакет 6 путей (отчёт приёмки — новый) + этот
  чекпойнт `memory/git.md` (F43) = 7.
- К коммиту — 7 путей (`./`-префикс, без `--`): `.opencode/agents/auditor.md`,
  `.opencode/mail/service-mcp-ready-r7.md`, `.opencode/memory/{auditor,validator}.md`,
  `.opencode/state/current/receipts.yaml`, `.opencode/memory/git.md`,
  `docs/reviews/service-permissions-auditor-target-2026-10-02.md`.
- Сообщение: `chore(agents): auditor — снято инертное правило read deny
  **/target/** (P2-2); приёмка`. Ожидаемая staged-сверка: **6 `M` + 1 `A`**.
- Осталось: `add` 7 путей → `diff --cached --name-status` → `commit` →
  `push origin develop`. Хеши — в ответе `lead` (не здесь). После push в
  отслеживаемые файлы не писать (F43).

## migrator · 02.10.2026 · готово (F63 закрыт)

- Сделано (одна строка): `docs/analysis/findings-registry.md`, строка **F63** —
  статус «открыт (остаток; вход CCSN/расширения правил)» → «закрыт 02.10.2026:
  CCSN standard (secret.*) блокирует shell-чтение `.env`/`.ssh` глобально; в CREDO
  дублируется `wave0-guard`; улика — `target/wave0b-csn/f63-shell-secret-deny.jsonl`».
  Источник/суть/связи строки сохранены. Источник — лента r7 §«F63 — проверка и
  закрытие» (:154–164), решение владельца «закрыть тут», задача не заводится.
- Проверки: `git diff -- docs/analysis/findings-registry.md` → ровно один ханк,
  одна строка (F63 1+/1−), прочие строки реестра не тронуты; `git diff --check --
  docs/analysis/findings-registry.md` пусто; `git diff --numstat -- src tests
  Cargo.toml` пусто; `cargo` не запускал (D50).
- Дальше / риски: коммит не делал (гейт впереди).

## migrator · 02.10.2026 · готово (F65 закрыт)

- Сделано (одна строка): `docs/analysis/findings-registry.md`, строка **F65** —
  статус «открыт (операционная осторожность в служебной зоне)» → «закрыт
  02.10.2026: подтверждено на актуальной 2.0.22 (latest) — churn воспроизводится;
  принято в практику: `reload` только по необходимости, после — проверять
  MCP-каталог, в temp-полигоне без нужды не вызывать; при будущем обновлении —
  ре-проверка». Источник/суть/связи строки сохранены.
- Источник — факт-чек владельца/сервисной сессии: churn MCP после `reload`
  наблюдался на **актуальной** версии `2.0.22 (channel=latest)`, ждать «новой
  версии» нечего; принято в практику. Задача не заводится.
- Проверки: `git diff -- docs/analysis/findings-registry.md` → ровно один ханк,
  одна строка (F65 1+/1−), прочие строки реестра не тронуты; `git diff --check`
  пусто (по всему дереву); `cargo` не запускал (D50).
- Дальше / риски: коммит не делал (гейт впереди).

## git · 02.10.2026 · план пакета (F63)

- Подтверждение сверено по ленте — §«гейт F63 пройден» (:166–176), ответ
  владельца «Коммит + push develop». База: `develop` = `origin/develop` = `HEAD`
  = `e861bba`; ветки нет, master не трогаем.
- Снимок: 4 `M` = ровно пакет 4 путей + этот чекпойнт `memory/git.md` (F43) = 5.
- К коммиту — 5 путей (`./`-префикс, без `--`):
  `docs/analysis/findings-registry.md`, `.opencode/mail/service-mcp-ready-r7.md`,
  `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
  `.opencode/memory/git.md`.
- Сообщение: `docs(T-15): F63 закрыт — CCSN standard покрывает shell-чтение
  секретов глобально`. Ожидаемая staged-сверка: **5 `M`**.
- Осталось: `add` 5 путей → `diff --cached --name-status` → `commit` →
  `push origin develop`. Хеши — в ответе `lead` (не здесь). После push в
  отслеживаемые файлы не писать (F43).

## git · 02.10.2026 · план пакета (F65)

- Подтверждение сверено по ленте — §«F65 закрыт (факт-чек версии)» (:202–220),
  ответ владельца (:212) «Коммит + push develop». База: `develop` =
  `origin/develop` = `HEAD` = `ef39b33`; ветки нет, master не трогаем.
- Снимок: 4 `M` = ровно пакет 4 путей + этот чекпойнт `memory/git.md` (F43) = 5.
- К коммиту — 5 путей (`./`-префикс, без `--`):
  `docs/analysis/findings-registry.md`, `.opencode/mail/service-mcp-ready-r7.md`,
  `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
  `.opencode/memory/git.md`.
- Сообщение: `docs(T-15): F65 закрыт — reload-эффект подтверждён на 2.0.22,
  принято в практику`. Ожидаемая staged-сверка: **5 `M`**.
- Осталось: `add` 5 путей → `diff --cached --name-status` → `commit` →
  `push origin develop`. Хеши — в ответе `lead` (не здесь). После push в
  отслеживаемые файлы не писать (F43).
