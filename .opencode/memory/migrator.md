# Память: migrator (журнал Q/D)

- **Канон:** `docs/BRIEF.md` (§2 ID, §4 шаблоны, §5.3 сверка, §7 перенос).
- **Правило:** чекпойнт — состояние записи (Q/D), что проверено, следующее
  действие, ссылки. Кратко.

## Чекпойнты

- Чекпойнтов ещё не было.
- 2026-09-27 · T-12 W1 (готово): созданы `docs/questions/Q44.md` (resolved by D39),
  `docs/decisions/D39-loop-dispatcher.md` (accepted, Affects `.opencode/**` и
  `AGENTS.md`, Tasks T-12), `docs/tasks/T-12-agent-loop/README.md` (⬜, P1,
  Источник D39 (Q44)); изменены `docs/SPECIFICATION.md` (§10 №39),
  `docs/TRACEABILITY.md` (строка Q44→D39→T-12), `docs/tasks/README.md` (строка
  T-12). Вердикт сверки — ⚪ не применимо (процессное; cargo не запускался).
  ID свободны, ссылки живые. Отчёт — лента `.opencode/mail/T-12.md`.
  Следующее действие: ждать W2 (сервисная сессия); при создании
  `docs/analysis/README.md` — вернуть ссылку в `Affects` D39.
- 2026-09-27 · T-12 W1 (уточнение под W2): в D39 п. 3 «Состояние на диске» —
  четыре файла (+`progress.yaml`, append-only, пишет только `lead`; resume читает
  `next_action` + `progress`); `Affects` D39 +`.opencode/rules/git-workflow.md`;
  карточка T-12 W2 +`.opencode/rules/git-workflow.md`, `.opencode/agents/git.md`.
  D38/Q43/T-03 не тронуты. Отчёт — лента.
- 2026-09-27 · T-12 (уточнение D39 по аудиту): права `edit` на `state` в пп. 1–2
  приведены к поэлементным файловым (`lead` → `progress.yaml`; `analyst` →
  `next_action.yaml` + `current_state.yaml`); в п. 3 добавлена строка про
  поэлементные права и «один писатель на файл». `state/**` больше не упоминается.
  Отмечено `lead`: SPEC §10 №39 перечисляет состояние без `progress.yaml`
  (не правил — вне объёма). Отчёт — лента.
- 2026-09-27 · T-12 (SPEC §10 №39): добавлен `progress.yaml` — только `lead` в
  краткое перечисление состояния (все четыре файла + «поэлементные права»),
  смысл строки сохранён, ссылка на D39 — на месте. Замечание закрыто.
  Отчёт — лента.
- 2026-09-27 · T-12 (аудит r2, P3): в D39 п. 1 shell-команды `lead` дополнены
  `git branch --show-current` (фронтматтер `lead.md:18`, `review.md`); п. 2
  `analyst` не изменён. Отчёт — лента.
