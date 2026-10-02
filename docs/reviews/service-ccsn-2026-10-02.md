# Приёмка: service-ccsn — CC Safety Net (перенос, T-15)

**Проверка:** сервисная операция `service-mcp-ready-r7` — перенос CC Safety Net
(кандидат B0-i5) + правки P2/P3 аудита + отложенные записи о закрытии полигона
B0-own.

**Версия:** `develop` @ `38800cb96a6f9c2c9294767e8a52732479a2e262`
(= `origin/develop` = `HEAD`) + рабочее дерево (прямая правка без ветки);
снимок 02.10.2026.

**Вердикт:** принято

**P1:** —
**P2:** —
**P3:** —

**Известный пункт (вне пакета, отдельное решение владельца):** P2-2 аудитора —
`.opencode/agents/auditor.md:11` правило `read deny **/target/**` инертно для
корневого `target/**` (чтение улик `target/wave0b-csn/**` прошло). В приёмку не
входит, зафиксирован как известный; проектную область не задевает.

## Проверки

### 1. База и снимок пакета

- `git status -sb` → `## develop...origin/develop` + 8 `M` + `??` r7-лента.
- `git rev-parse HEAD develop origin/develop` → `38800cb…` ×3 (ветки нет; прямая
  правка в `develop`, как в `service-c10`/`service-b0-own-transfer`).
- Снимок `git status --porcelain` для гейта (состав пакета ниже).

### 2. CCSN включён (улики `target/wave0b-csn/`, вне git — `.gitignore:1`)

- `git diff --no-index global-before.jsonc global-after.jsonc` → ровно `+6`:
  добавлен блок `plugins` object-формы
  (`{package: "cc-safety-net@latest", options: {shell: "powershell"}}`); 4
  C10-правила (`read:*.env`, `read:*/.ssh/*`, `shell:git push *--force*`,
  `shell:git reset --hard*`) дословно целы.
- `ccsn-doctor.txt:18,23,51,75,88` → OpenCode **Detected | Configured |
  Verified**, self-test **3/3**, `cc-safety-net 2.5.1`, OpenCode 2.0.22,
  `Platform win32 x64`.
- `ccsn-status.txt:3-6` → Protection destructive/secrets ok, Level **standard**,
  Policy `~\.cc-safety-net\policy.json`.
- Логи (`target/wave0b-csn/logs/**`):
  - deny `git clean -fdx` — `v:2.5.1`, `ruleId:git.clean-force`,
    `intent:use_alternative`, cwd scratch (`…2026-10-02-ses_f03316…`);
  - allow `echo CSN-OK` и allow `git clean -ndx` (тот же лог);
  - след B0-i5: `2026-09-28` `v:2.4.11` deny `git push --force origin main`
    (`…/wave0b-probe/2026-09/`) — сохранён.
- **Наблюдение:** `rg` валидатора по паттерну с `.env`/`.ssh` в `target/**`
  отклонён плагином `wave0-guard: ssh-read` — контур C10 жив (компенсация:
  конфиг прочитан через `read`/`rg policies|plugins`).

### 3. Правки аудита (P2/P3 закрыты)

- P2-1 `wave0b-report.md:87`: хвост `перенос — после \`auditor\`` заменён на
  `перенос — **исполнено 02.10.2026** (v2.5.1; пресет standard; аудит переноса —
  \`service-mcp-ready-r7\`)` — самопротиворечие снято (`-U0`: 1+/1−).
- P3 `wave0b-own-report.md:58`: CCSN отделён от C10-артефактов —
  «**CC Safety Net включён 02.10.2026 (v2.5.1, standard; `service-mcp-ready-r7`)**;
  C10 — 4 точных правила глобально + плагин `wave0-guard`» (1+/1−).

### 4. Отложенные записи (в целевом пакете)

- `wave0b-own-report.md:87` §5: `[ ]`→`[x]` «откат полигона — прототипы выключены
  (`_off/`, 02.10); удаление полигона — после заморозки».
- `wave0b-own.md:362-367` §«Откат полигона»: закрытие 02.10, прототипы в `_off/`,
  журналы не пишутся, улики `target/wave0b-own-i1…i7/`, удаление — после заморозки.
- `service-mcp-ready-r6.md` — F43-остаток/записи (+31) в пакете.

### 5. Записи

- Карточка `README.md:213` — C10: «CC Safety Net — включён 02.10 (standard,
  `cc-safety-net` 2.5.1)» (1+/1−; остальной текст сохранён).
- Память `service.md:118-124` (CCSN включён), `migrator.md:9-27,29-45` (обе
  правки r7), `auditor.md` (аудит CCSN) — append, согласованы с лентой.
- Лента `service-mcp-ready-r7.md` — §установка/§аудитор/§migrator на месте.

### 6. Границы и гигиена

- `git diff --numstat -- src tests Cargo.toml AGENTS.md opencode.json` → пусто;
  `git status --porcelain .opencode/agents|rules`, `AGENTS.md opencode.json`,
  `docs/decisions|questions|TRACEABILITY.md|features` → пусто.
- `git diff --check` → пусто (единственный вывод — CRLF-warning на
  `.opencode/mail/service-mcp-ready-r6.md`, не правка этого шага).
- `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из 18` (состав
  стабилен; фронтматтеры не затронуты).
- Глобальный конфиг `~/.config/opencode/opencode.jsonc` и `~/.cc-safety-net/**` —
  вне репозитория; in-repo улики-копии в `target/**` (вне git).

### 7. DoD

- `cargo` **не запускался** — продуктовый код (`src/**`, `tests/**`, `Cargo.toml`)
  не затронут (D50; `review.md` «Порог существенности»).

## Состав пакета (снимок `git status --porcelain`)

```
 M .opencode/mail/service-mcp-ready-r6.md
 M .opencode/memory/auditor.md
 M .opencode/memory/migrator.md
 M .opencode/memory/service.md
 M docs/tasks/T-15-mcp-ready-process/README.md
 M docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md
 M docs/tasks/T-15-mcp-ready-process/wave0b-own.md
 M docs/tasks/T-15-mcp-ready-process/wave0b-report.md
?? .opencode/mail/service-mcp-ready-r7.md
```

Сверх целевых правок в пакете — F43-остаток `M .opencode/mail/service-mcp-ready-r6.md`
(+31: хвост прошлой операции) и мои записи приёмки (`docs/reviews/service-ccsn-2026-10-02.md`
— `??`, память `validator`, эта лента, `receipts.yaml`) добавятся к снимку после
записи. Не исключать из пакета (прецедент `service-t15-run-review`/`service-c10`).

## Что проверено и ок

- Конфиг before/after — 4 C10-правила целы, добавлен только `plugins`.
- Диагностика CCSN (doctor/status) и живые логи deny/allow + след B0-i5.
- Правки P2/P3 аудита — обе адресно, самопротиворечие снято.
- Отложенные записи полигона (`_off/`, удаление после заморозки).
- Карточка T-15 (C10) и память `{service,migrator,auditor}`.
- Границы (продуктовый код, канон, репо-`opencode.json`) и `git diff --check`.
- `agents-perms.mjs` — 11 из 18; ветка/база `develop` = `origin/develop` = `38800cb`.
