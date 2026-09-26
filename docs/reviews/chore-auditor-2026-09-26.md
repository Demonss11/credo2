# Приёмка: primary-агент `auditor`

**Проверка:** новый primary-агент `auditor` (аудит системы агентов и
мета-документации) — незакоммиченные изменения: `.opencode/agents/auditor.md`
(untracked), абзац «Вне маршрутов…» в `AGENTS.md`, запись в `docs/CHANGELOG.md`.

**Версия:** рабочее дерево + HEAD `1644cf1` (`chore: роль rust-expert и вычитка
идиоматики в маршруте кода`); изменения не закоммичены.

**Вердикт:** принято с замечаниями

**P1:** —

**P2:** —

**P3:**
- `AGENTS.md:53` — «правит только `AGENTS.md` и `.opencode/**`» шире реальных
  прав: фронтматтер `auditor.md` разрешает `edit` только на `AGENTS.md`,
  `.opencode/agents/**` и `.opencode/rules/**` (не `skills/**` и прочее
  `.opencode/**`). Последствие: читатель может решить, что auditor правит весь
  служебный каталог. → сузить формулировку до «`AGENTS.md`,
  `.opencode/agents/**` и `.opencode/rules/**`» (то же в `docs/CHANGELOG.md:47`).

**Проверки:**
- `git status --short` → `M AGENTS.md`, `M docs/CHANGELOG.md`,
  `?? .opencode/agents/auditor.md` (код не тронут).
- `git log -1 --oneline` → `1644cf1 chore: роль rust-expert …`; HEAD совпадает
  с заявленной версией.
- `rg -n "auditor" .opencode AGENTS.md docs/CHANGELOG.md` → упоминания только в
  `AGENTS.md:51`, `docs/CHANGELOG.md:45`, `auditor.md` (совпадения в
  `rust-skills/**` — английское слово «auditor», не роль).
- `cargo test --all` → 60 passed / 0 failed (33 lib + 4 features_inventory +
  12 publish + 11 rest); DoD зелёный.
- Резолвинг `auditor` как primary — `opencode debug agents` мне недоступен по
  allow-листу; права соответствуют заявленным, резолвинг подтверждён внешней
  проверкой.

**Что проверено и ок:**
- `auditor.md` фронтматтер валиден: `mode: primary`,
  `model: opencode-go/deepseek-v4-pro` (модель в whitelist `opencode.json`).
- Права соответствуют промпту: `edit deny *` → allow `AGENTS.md`,
  `.opencode/agents/**`, `.opencode/rules/**`; `shell deny *` → allow `rg *`,
  `git status|log|diff|show|branch *`, `opencode debug *`, `opencode reload`;
  `question: allow`; `subagent`/`skill`/`webfetch`/`websearch` deny;
  `external_directory: ask`. Read-deny (`target/`, `.git/`, `node_modules/`,
  `Cargo.lock`, `.credo/`) согласован с `.opencode/rules/workspace.md`.
- Промпт не конфликтует с каноном: не правит `docs/**`, `opencode.json`, код
  («Границы» `auditor.md:119-127`); чек-лист не противоречит `AGENTS.md`,
  `docs/BRIEF.md` и `.opencode/rules/*.md`.
- Бюджет задан численно: ≤ 15 файлов, ≤ 10 `rg` (`auditor.md:112-115`).
- Ссылки живые: `git-workflow.md`, `review.md`, `workspace.md`,
  `docs/README.md`, `docs/BRIEF.md`, `docs/tasks/README.md`,
  `docs/features/README.md` существуют.
- Вне маршрутов: `AGENTS.md:51-53` явно говорит, что `auditor` не входит в
  маршруты и запускается владельцем отдельной сессией; `auditor` отсутствует в
  списке `subagent` у `lead` (согласовано); `default_agent` в `opencode.json:7`
  остаётся `lead`.
- Q41: описание роли не дублируется — канон в `AGENTS.md` (абзац «Вне
  маршрутов»), в `docs/README.md` только указатель на §Рабочая группа агентов,
  в `CHANGELOG` — хронологическая запись.
- `docs/CHANGELOG.md:45-48` отражает проход (раздел «Процесс»).

## Результат

P3 закрыт: формулировки в `AGENTS.md` и `docs/CHANGELOG.md` сужены до реальных прав (`AGENTS.md`, `.opencode/agents/**`, `.opencode/rules/**`). Вердикт — принято.
