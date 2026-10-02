# service-mcp-ready-r6 — лента операции: T-15 фаза C, C10 (`policies`-страховки)

Открыта: 02.10.2026. **Сервисная операция** (продолжение программы T-15; после
B0-own: BO-i2 дал вердикт по механизму, P2 отложен «до C10»). Предмет C10
(карточка T-15): `experimental.policies` — **ревизия списка, живая проба,
решение «включить/отклонить»**; при включении — служебная зона владельца.

**Входы:** [`wave0-token-hygiene.md`](../../docs/tasks/T-15-mcp-ready-process/wave0-token-hygiene.md)
§3.5 (кандидаты), [`wave0b-own.md`](../../docs/tasks/T-15-mcp-ready-process/wave0b-own.md)
§BO-i2 (пробы: policies/permissions/плагин), отчёт
[`wave0b-own-report.md`](../../docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md) §3.

**Рамка:** канон агентов не правится; правка конфигурации — служебная зона
владельца; `cargo` не запускается (D50); пробы — изолированно (полигон/локально);
улики — `target/wave0b-own-i7/` (или отдельная папка).

## сервисная сессия · 02.10.2026 · открытие и ревизия списка

- Полигон BO-i2 жив (`%TEMP%\opencode\wave0b-own`); среда — OpenCode 2.0.22.
- Проверяемые кандидаты (уточняются по §3.5 и BO-i2): `shell:*--force*`,
  `shell:*reset --hard*`, `read:*.env`, `read:*/.ssh/*`; отдельно —
  `external_directory` (риск ложных блокировок служебных операций с `%TEMP%`).
- Записи проб — ниже (append).

## сервисная сессия · 02.10.2026 · пробы C10 исполнены

- **Полигоны:** `wave0b-c10` (4 правила), `wave0b-c10b` (+`shell:*/.ssh/*`),
  `wave0b-c10c` (+`shell:*.env*`); 10 проб; улики —
  `target/wave0b-own-i7/` (`c10-probes.md`, конфиги).
- **Результаты:** P1 force → blocked; P2 reset --hard → blocked; P3 `.env` →
  blocked, `.env.example` → allowed; **P4 — ключевая находка: `read`-политика
  `.ssh` обходится shell-командой** (`Get-Content` прошёл); P5 allow-поток
  (git/cargo/echo) — без ложных срабатываний; P6 `external_directory` не
  блокируется; P7/P8 `shell:*/.ssh/*` блокирует forward- и backslash-варианты;
  P9 `shell:*.env*` блокирует shell-чтение `.env`; P10 — цена: блокируется и
  `.env.example` через shell (`read`-инструментом он по-прежнему разрешён).
- **Выводы:** базовый набор + `shell:*/.ssh/*` — рабочий статический контур;
  для shell-чтения `.env` — либо грубый `shell:*.env*`, либо контекстный
  P2-плагин (точные правила: `.env` deny, `.env.example` allow, `Remove-Item`
  -Recurse -Force, аудит); `external_directory` не трогаем.
- **Дальше:** решение владельца (состав/scope/плагин) → включение в служебной
  зоне → живая проверка → записи и коммит.

## сервисная сессия · 02.10.2026 · C10 включено — готово

- **Решение владельца:** «6 правил глобально + P2-плагин»; после находок
  P11/P12 (широкие shell-паттерны матчат упоминания: `echo`, поиск, коммит-
  сообщения, промпты) — **сужено до 4 точных правил** (решение владельца).
- **Глобальный конфиг** `~/.config/opencode/opencode.jsonc`:
  `read:*.env`, `read:*/.ssh/*`, `shell:git push *--force*`,
  `shell:git reset --hard*`.
- **Плагин** `.opencode/plugins/wave0-guard.ts` (перенос P2, служебная зона):
  якорные deny (git push force / reset --hard / shell-чтение `.env`/`.ssh` /
  `Remove-Item -Recurse -Force`) + аудит `target/wave0-guard.jsonl` (bounded);
  внешние правки — только аудит (P13: deny ломал служебные записи).
- **Проверки (P14):** `git push --force` → policy; `Get-Content .env` (repo,
  `--agent build`) → `wave0-guard: secrets-env`; `git reset --hard` → policy;
  `echo "check .env mention"` → разрешён (ложное снято); allow-поток — ок.
- **Грабли:** reload общего сервиса → churn MCP-каталога (восстанавливается);
  широкие паттерны → ложные блокировки упоминаний (P11/P12); plugin
  outside-edit → блокировал служебную запись конфига (P13).
- **Улики:** `target/wave0b-own-i7/` (`c10-probes.md`, конфиги, журнал guard).
- **Дальше:** `validator` (приёмка C10/переноса P2) → гейт → коммит
  (плагин + записи; глобальный конфиг — вне репозитория).

## validator · 02.10.2026 · принято (C10) — см. секцию роли ниже

- Вердикт: **принято, P1/P2/P3 нет** (P3 информационно: журнал evaluate растёт —
  закрыт ротацией); отчёт `docs/reviews/service-c10-policies-2026-10-02.md`,
  квитанция `service-c10` (iteration 1, accepted) в `receipts.yaml`.
- Проверки: плагин (якорные deny + аудит + outside-edit аудит), глобальная
  копия (4 правила), улики P1–P14, записи, границы; `cargo` не запускался (D50).

## сервисная сессия · 02.10.2026 · гейт пакета C10 пройден

- Владелец подтвердил (**question**): «Коммит + push develop (Recommended)».
- Пакет — 12 путей + запись роли `git` (F43) = 13: `??` —
  `.opencode/plugins/wave0-guard.ts`, `.opencode/mail/service-mcp-ready-r6.md`,
  `docs/reviews/service-c10-policies-2026-10-02.md`; `M` —
  `.opencode/mail/service-mcp-ready-r5.md` (F43-остаток),
  `.opencode/memory/{migrator,service,validator}.md`,
  `.opencode/state/current/receipts.yaml`,
  `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}`,
  `.opencode/memory/git.md`.
- Сообщение коммита: `chore(process): T-15 C10 — policies (4 точных) + плагин
  wave0-guard (P2); приёмка`.
- База: `develop` = `origin/develop` = `dace3eb`; master не трогаем.
- Следующее действие — `dispatch git`.

## git · 02.10.2026 · готово (пакет C10)

- Коммит **`142458d`** (13 файлов: 9 `M` + 4 `A`; +533/−7) → push
  `dace3eb..142458d` в `origin/develop`; ветки не создавались; master не
  трогался; дерево чистое.

## сервисная сессия · 02.10.2026 · C10 завершён

- **Итог C10:** глобально — 4 точных правила (`read:*.env`, `read:*/.ssh/*`,
  `shell:git push *--force*`, `shell:git reset --hard*`); служебная зона —
  плагин `wave0-guard` (P2 перенесён; якорные deny + аудит
  `target/wave0-guard.jsonl`; внешние правки — аудит). **CC Safety Net** —
  отдельным решением позже.
- **Находки:** P11/P12 (широкие shell-паттерны ловят упоминания — сужено),
  P13 (plugin outside-edit блокировал служебную запись — переведён в аудит);
  reload общего сервиса — churn MCP-каталога (восстанавливается).
- **Проверки:** force/reset → policy; `.env` (repo) → плагин; упоминания —
  разрешены; allow-поток — ок; приёмка `validator` — принято (P1/P2/P3 нет).
- **Коммит:** `142458d` (включая F43-остаток r5); глобальный конфиг — вне
  репозитория (служебная зона владельца).
- **Дальше по T-15:** «чистый» S-прогон (F26/F27) или фаза C (C1 — схема
  состояния); P2-перенос закрыт, полигон — до заморозки.

## сервисная сессия · 02.10.2026 · хвосты BO-i2/C10 — F62–F66

- Решение владельца: внести наблюдения B0-own §4 и находки C10 (P11–P13) в
  реестр находок сейчас, одним пакетом (`migrator`), и закоммитить.
- Состав: F62 (широкие shell-паттерны матчат упоминания — сужено в C10),
  F63 (read-политики обходятся shell; остаток — shell-чтение `.env` вне CREDO),
  F64 (guard deny вне location ломал служебные записи — переведён в аудит),
  F65 (`opencode reload` в локации → churn MCP-каталога общей сессии),
  F66 (эксплуатационные грабли B0-own: watcher-дубли, `stats`/`projectID`,
  npm-шим/DEP0190, отказ модели от деструктивной пробы, `sh_*`).
- Карточка: `B0-own-P2` — дополнить «закрыт (C10)».

## сервисная сессия · 02.10.2026 · пакет F62–F66 — гейт пройден (коммит)

- Подтверждение владельца (question): «Внести F62–F66 сейчас … и коммит/пуш
  develop».
- Пакет — 5 путей + запись роли `git` (F43): `M` —
  `docs/analysis/findings-registry.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
  `.opencode/mail/service-mcp-ready-r6.md`, `.opencode/memory/{migrator,service}.md`;
  + `.opencode/memory/git.md`.
- Сообщение: `docs(T-15): findings F62–F66 (C10/B0-own) + B0-own-P2 закрыт (C10)`.
- База: `develop` = `origin/develop` = `142458d`; master не трогаем.
- Следующее действие — `dispatch git`.

## migrator · 02.10.2026 · готово (C10: карточка)

- **Сделано:** две правки в сводной таблице «Реестр задач T-15»
  (`docs/tasks/T-15-mcp-ready-process/README.md`): C10 `⬜` → `✅ 02.10 — 4 точных
  правила глобально (`read:*.env`, `read:*/.ssh/*`, `shell:git push *--force*`,
  `shell:git reset --hard*`) + плагин `wave0-guard` (якорные deny + аудит);
  CC Safety Net — позже`; B0-own-P2 — в статус дописано `· перенесён 02.10 (C10)`
  (текст BO-i2 сохранён). Иных строк/файлов не трогал.
- **Проверки:** `git diff -- <README>` → ровно 2 строки (C10: 1+/1−; B0-own-P2:
  1+/1−); `git diff --check` пусто; `cargo` не запускал (D50).
- **Дальше / риски:** приёмка `validator` (C10/перенос P2) → гейт → коммит;
  границ не нарушал (только `docs/tasks/**`).

## validator · 02.10.2026 · принято (C10)

- **Вердикт:** принято; P1/P2/P3 нет. Отчёт
  `docs/reviews/service-c10-policies-2026-10-02.md`; квитанция `accepted`
  (task `service-c10`, iteration 1) записана в
  `.opencode/state/current/receipts.yaml`.
- **База/версия:** `develop` = `origin/develop` = `HEAD` = `dace3eb` (ветки
  нет, прямая правка; `git status -sb` → `## develop...origin/develop`).
- **Артефакты = заявленному:** плагин `Plugin.define({id:"wave0-guard"})` —
  якорные shell-deny (push force/`-f`, reset --hard, чтение `.env` кроме
  `.env.example`, `.ssh`, `Remove-Item -Recurse -Force`); **внешние правки —
  аудит `outside-edit`, не deny (P13)**; журнал `target/wave0-guard.jsonl`
  (bounded, ротация `.1` при `>8 МБ` каждые 200 appends). Глобальная копия
  `target/wave0b-own-i7/c10-final-global.jsonc` — ровно 4 правила, совпадают с
  лентой (:50–51), `service.md:99–101`, карточкой. Улики P1–P14 +
  `wave0-guard-journal-sample.jsonl`: force/reset/env — блок; упоминания
  (`echo "check .env mention"`) — allow; P13 виден до/после правки
  (`:19` deny `outside-location` → `:29` аудит `outside-edit`).
- **Записи:** лента §«C10 включено» (:44–64), `memory/{service,migrator}`,
  карточка T-15 (C10 ✅ + B0-own-P2 `· перенесён 02.10 (C10)`), `wave0b-own.md`
  (§Статус), `wave0b-own-report.md` §3 (P2 «исполнено 02.10.2026»).
- **Границы чисты:** `src/tests/Cargo.toml/AGENTS.md` — пусто; канон
  `.opencode/agents|rules`/`AGENTS.md`/`opencode.json`/`docs/decisions|questions`/
  `docs/TRACEABILITY.md` — не в статусе; `git diff --check` пусто; `/target` в
  `.gitignore:1` (журнал вне git). `agents-perms.mjs` → `11 из 18`. `cargo` не
  запускался (D50; продуктовый код не затронут).
- **Не моё (в пакете):** `M .opencode/mail/service-mcp-ready-r5.md` (+20) —
  F43-остаток прошлой операции (`service-b0-own-transfer`, коммит `dace3eb`):
  git по F43 после push не писал в отслеживаемые файлы.
- **Глобальный конфиг `~/.config/opencode/opencode.jsonc` — вне репозитория**
  (служебная зона владельца); in-repo копия — улика `target/**` (вне git).
- **Состав пакета для гейта** (снимок `git status --porcelain`): целевые
  7 путей — `?? .opencode/plugins/wave0-guard.ts`, `?? .opencode/mail/service-mcp-ready-r6.md`,
  `M .opencode/memory/{migrator,service}.md`,
  `M docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}`;
  сверх цели — `M .opencode/mail/service-mcp-ready-r5.md` (F43-остаток).
- **Дальше:** гейт пакета подтверждается владельцем, затем `dispatch git`
  (коммит + push `develop`).

## git � 02.10.2026 � ���� ������ (C10)

- �������: ������������� ��������� � ������ ������ C10 �������� (:74�88), ������� + push develop�; ���� develop = origin/develop = HEAD = dace3eb; ������ git status -sb = 8 M + 3 ?? = ����� ����� 12 ����� (+ �������� memory/git.md, F43).
- � ������� � 13 ����� (.opencode/plugins/wave0-guard.ts, .opencode/mail/service-mcp-ready-r6.md, docs/reviews/service-c10-policies-2026-10-02.md, .opencode/mail/service-mcp-ready-r5.md, .opencode/memory/{migrator,service,validator,git}.md, .opencode/state/current/receipts.yaml, docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}); ��������� chore(process): T-15 C10 � policies (4 ������) + ������ wave0-guard (P2); ������; ��������� staged-������ 9 M + 4 A.
- ��������: dd > ������ staged > commit > push origin develop; ����� ���, master �� �������. ���� � � ������ lead (F43).

## migrator · 02.10.2026 · готово (F62–F66)

- **Сделано:** `docs/analysis/findings-registry.md` — после F61 добавлены 5
  записей F62–F66 (стиль таблицы; ID уникальны, после F61); карточка
  `docs/tasks/T-15-mcp-ready-process/README.md` — строка B0-own-P2: в ячейку
  статуса дописано ` · закрыт (C10)` (текст BO-i2 сохранён). Иных строк/файлов
  не трогал.
- **Проверки:** `git diff -- <2 файла>` → ровно целевые правки (F61-строка без
  изменений, +5 строк; B0-own-P2 1+/1−); `git diff --check` по этим файлам →
  пусто; ID F62–F66 уникальны (grep `^\| F6[2-6] ` → 5); `cargo` не запускал
  (D50).
- **Дальше / риски:** приёмка `validator` (пакет хвостов F62–F66) → гейт →
  коммит; границ не нарушал (только `docs/analysis/**`, `docs/tasks/**`).

## git · 02.10.2026 · план пакета (F62–F66)

- Сделано: подтверждение сверено по §«пакет F62–F66 — гейт пройден (коммит)»
  (:125–135), ответ владельца «Внести F62–F66 сейчас … и коммит/пуш develop».
  `git status -sb` → `## develop...origin/develop` + 5 `M` = ровно пакет;
  `HEAD` = `origin/develop` = develop = `142458d`.
- Проверки: `rev-parse HEAD origin/develop develop` → `142458d` (трижды);
  `git log -1 --oneline` → `142458d chore(process): T-15 C10 — policies
  (4 точных) + плагин wave0-guard (P2); приёмка`.
- Дальше / риски: `add` 6 путей (`./`-префикс, без `--`) → `diff --cached
  --name-status` (ожидается 6 `M`) → `commit -m "docs(T-15): findings F62–F66
  (C10/B0-own) + B0-own-P2 закрыт (C10)"` → `push origin develop`. Хеши — в
  ответе `lead`; после push в отслеживаемые файлы не пишу (F43).
