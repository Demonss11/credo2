**Проверка:** P2-2 — снятие инертного правила `.opencode/agents/auditor.md:11`
`read deny **/target/**` (решение владельца «Убрать правило»; отдельный шаг
канона: правка → аудит `auditor` → приёмка → коммит).

**Версия:** `develop` = `origin/develop` = `HEAD` = `606055f` + рабочее дерево
(прямая правка без ветки), 02.10.2026.

**Вердикт:** принято

**P1:** — критичных проблем нет.
**P2:** — нет.
**P3:** — нет.

## Проверки

- **Дифф предмета:** `git diff .opencode/agents/auditor.md` — ровно одна удалённая
  строка `- { action: read, resource: "**/target/**", effect: deny }` (из блока
  read после `edit`); иных строк не тронуто. `git diff --numstat` →
  `0 1 .opencode/agents/auditor.md`.
- **Целость файла:** тело (`auditor.md:37–202`) и остальной фронтматтер
  (`:1–10`, `:11–35`) — без правок; сохранены deny `.git/**`, `**/node_modules/**`,
  `Cargo.lock`, `.credo/**`, shell-allowlist (`*` deny), `webfetch`/`websearch`/
  `skill`/`subagent` deny, `question allow`, `external_directory ask`.
- **Наличие ссылок на снятое правило:**
  `rg -n "target/\*\*|\*\*/target" .opencode/rules AGENTS.md .opencode/agents/auditor.md`
  → пусто (exit 1). Ссылок в каноне нет; остальные `**/target/**` — фронтматтеры
  других ролей, не затронуты.
- **Машинная сверка прав:** `node .opencode/scripts/agents-perms.mjs` (дважды,
  R2/`auditor.md:152–153`) → стабильно `agents: 11 из 18`; фронтматтер `auditor`
  в списке, shell-allowlist без изменений.
- **Границы пакета:** `git status -sb` → только `M .opencode/agents/auditor.md`,
  `M .opencode/mail/service-mcp-ready-r7.md`, `M .opencode/memory/auditor.md`;
  `git diff --stat -- src tests Cargo.toml AGENTS.md` пусто;
  `git diff --check` пусто. `numstat`: `62 0` лента, `16 0` память (`auditor`) —
  записи append-only.
- **Записи:** лента r7 — §«P2-2: снятие…» (:95–103), §«P2-2: применение (reload)»
  (:105–110) — reload зафиксирован явно; аудит `auditor` (:178–215, `P1/P2 нет,
  P3 — reload`); память `auditor` — append (:221–236).
- **DoD прототипа (R2):** `cargo fmt --check` — pass; `cargo clippy --all-targets
  -- -D warnings` — pass (Finished, 0 warnings); `cargo test --all` —
  **135 passed / 0 failed** (lib 61, docs_journal 14, features_inventory 4,
  mcp_draft 25, mcp_errors 8, publish 12, rest 11).

## Что проверено и ок

- Основание снятия: паттерн `**/target/**` требует сегмент перед `target/` —
  для корневого `target/**` инертен (чтение `target/wave0b-*` сессией аудита
  проходило); протокол аудита требует чтения улик `target/**`. После снятия
  открыт только уже доступный корневой `target/**`; ничего сверх прежнего не
  открыто.
- «Инструкция ↔ права»: список команд `review.md:95–96` совпадает с фронтматтером
  `auditor`; read-права в него не входят; расхождений нет.
- P3 аудита (отсутствие явной записи `opencode reload` для правки) **закрыт**
  записью ленты r7 §«P2-2: применение (reload)» (:105–110).
- База: `git rev-parse HEAD develop origin/develop` → совпадает ×3; ветки нет,
  `master` не трогается; после коммита CCSN `606055f`.

## Технические заметки

- `git diff -- .<dot-путь>` движок отклоняет (узкий allowlist); рабочий вызов —
  `git diff <path>` без `--`. `cargo` запускался, несмотря на D50 (правка канона,
  продуктовый код не затронут) — для полноты DoD R2; результат зелёный.
