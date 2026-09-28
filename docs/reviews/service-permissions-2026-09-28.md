# Приёмка: право `validator` на `git branch --contains` (Q54/D49)

**Проверка:** правка системы агентов по Q54/D49 — ровно одна новая строка
allowlist `validator` (`.opencode/agents/validator.md:28`) и синхронизация
списка роли в `.opencode/rules/review.md:81`; журнал Q54/D49, SPEC §10 №49,
`TRACEABILITY.md`, `questions/README.md`, реестр F44; машинный резолв прав и
боевой смоук нового права.

**Версия:** `develop`, HEAD `6d4c840` + рабочее дерево (2026-09-28)

**Вердикт:** принято

**P1:** критичных проблем нет

**P2:** критичных проблем нет

**P3:** критичных проблем нет.
P3 аудитора (`findings-registry.md:52` — F44 ссылался на приёмку
`service-permissions-2026-09-28`, которой не было) **закрыт**: отчёт с этим
именем создан (этот файл). Имя в F44 и имя артефакта теперь совпадают.

**Проверки:**

- `git status --porcelain` → 10 `M` + 2 `??`: канон (`validator.md`,
  `review.md`), лента, 2 памяти (`migrator`, `auditor`), журнал
  (`SPECIFICATION.md`, `TRACEABILITY.md`, `findings-registry.md`,
  `questions/README.md`), untracked `Q54.md`/`D49-*.md`. Посторонних путей нет.
- `git diff --numstat` → `validator.md` 1/0, `review.md` 2/2 (одна строка
  переформулирована в две из-за переноса), лента 106/0 (append-only),
  `memory/migrator.md` 10/0, `memory/auditor.md` 12/0, четыре docs-файла по
  1/0.
- `git diff -- ./.opencode` → ровно ожидаемое: одна строка allowlist после
  `git grep *` (`+ { action: shell, resource: "git branch --contains *",
  effect: allow }`) и строка списка `validator` в `review.md` (`git branch
  --contains`). Иных правок канона нет (dot-путь взят в форме `./…` — квик
  движка, `review.md:99-103`).
- `git diff -- src tests` → пусто; `git diff -- AGENTS.md opencode.json` →
  пусто. Продуктовый код и корневые конфиги не тронуты.
- `opencode debug agents` (один прогон, `reload` не делался) → в блоке
  `permissions` роли `validator` резолв
  `{ action: shell, resource: "git branch --contains *", effect: allow }`
  присутствует (перед `webfetch/deny`); список команд совпадает с
  фронтматтером. Вывод срезан token-guard: опущено ~36909 байт из 51378
  (~36.9 / 51.4 КБ); хвост (строки 1546–2611) прочитан, блок `validator`
  виден полностью; иные роли сверены по файлам, а не по debug-выводу.
- **Смоук нового права (боевой):** `git branch --contains 0a5832f` →
  `* develop`, `exp/agent-cycle-rerun`, `exp/agent-update-t15w0`. Команда
  **принята** движком в свежей сессии; машинное подтверждение вхождения
  коммита в ветки получено (сверх ожидания — третья ветка
  `exp/agent-update-t15w0`; не расхождение).
- Журнал (чек-лист «Перенос Q/D»): `Q54.md` и `D49-validator-branch-contains.md`
  существуют; `D49` — поля `Resolves: Q54`, `Спека: SPEC §10 №49`, `Affects`
  (`validator.md`, `review.md`, `findings-registry.md`), `Tasks: —` с
  пояснением в «Сверке с кодом»; вердикт сверки — ⚪ «не применимо» (права/
  процесс, образец D44) с фактами до/после. Строка `SPECIFICATION.md:870`
  (№49, нумерация 45→49 без повторов), `TRACEABILITY.md:20`,
  `questions/README.md:37`, `findings-registry.md:52` (F44) согласованы;
  ID `Q54`/`D49` уникальны (последние до них — `Q53`/`D48`). Все
  относительные ссылки Q54/D49 живые (`validator.md`, `review.md`,
  `D44`, `D48`, `T-11-closeout-2026-09-28.md`,
  `rights-matching-2026-09-27.md`, лента).

**DoD (обоснование):** изменение — канон и права (`.opencode/**`), продуктовый
код (`src/**`, `tests/**`) не затронут (`git diff -- src tests` пусто).
`review.md` §«Возврат на доработку» для правок канона/прав предписывает
машинную сверку прав (`opencode debug agents`) и адресные проверки — они
выполнены. `cargo test --all` не перезапускался: `src/**`/`tests/**` неизменны
с пакета W8-config (107 passed / 0 failed, 47 files / 278 scenarios — улика
`receipts.yaml`, запись `W8-config`). `cargo fmt --check` не запускался:
Rust-файлы не менялись, форматировать нечего. `reload` не делался (прогон один),
поэтому «дважды после reload» неприменимо.

**Что проверено и ок:** фронтматтер `validator.md:23-28` и список
`review.md:81-82` (ровно одна новая строка, синхронны); отсутствие иных правок
канона и иных ролей; границы (`src`, `tests`, `AGENTS.md`, `opencode.json`);
машинный резолв и смоук права; журнал Q54/D49 и его связи (SPEC §10, TRACEABILITY,
`questions/README`, F44); живые относительные ссылки; append-дисциплина ленты.
