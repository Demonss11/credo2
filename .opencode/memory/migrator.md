# Память: migrator (журнал Q/D)

- **Канон:** `docs/BRIEF.md` (§2 ID, §4 шаблоны, §5.3 сверка, §7 перенос).
- **Правило:** чекпойнт — состояние записи (Q/D), что проверено, следующее
  действие, ссылки. Кратко.

## Чекпойнты

> D41: оперативная хроника задач — в `state/` и ленте; здесь — знание роли и
> аварийные чекпойнты. Записи ниже — история.

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
- 2026-09-27 · T-13 W1 (готово): созданы `docs/questions/Q45.md` (resolved by
  D40), `docs/questions/Q46.md` (resolved by D41),
  `docs/decisions/D40-scope-threshold.md` (accepted; scope-порог),
  `docs/decisions/D41-dispatch-refinements.md` (accepted; уточнения цикла),
  `docs/tasks/T-13-agent-hardening/README.md` (⬜, P1, Источник D40/D41 (Q45,
  Q46)), `docs/tasks/T-14-grammar-message-sync/README.md` (⬜, P2, Источник D41
  (Q46), зависит от T-03). Изменены `docs/SPECIFICATION.md` (§10 №40/№41),
  `docs/TRACEABILITY.md` (Q45→D40→T-13; Q46→D41→T-13, T-14),
  `docs/tasks/README.md` (сводка T-13/T-14). Вердикт сверки — ⚪ не применимо
  (процессное; `cargo` не запускался). ID свободны, ссылки живые. Отчёт — лента
  `.opencode/mail/T-13.md`. Следующее действие: W2 — research F4 и правки
  канона (сервисная сессия), затем W3 (аудит + приёмка) → W4 (пакет/коммиты).
  Замечание: в D41 добавлен п. 13 (GRAMMAR §7, F11) — опора T-14.
- 2026-09-28 · W8 т. 2 (Run 5, новые записи, готово): созданы Q47–Q53/D42–D48
  (`docs/questions/Q47..Q53.md`; `docs/decisions/D42-expect-iteration.md`,
  `D43-auditor-mail.md`, `D44-run5-refinements.md`, `D45-wave0-quality-config.md`,
  `D46-product-process-commits.md`, `D47-git-refinements-run5.md`,
  `D48-findings-registry-owner.md`). Реестр `docs/analysis/findings-registry.md` —
  внесены F15–F39 (F39 — блокировка прав, снята правкой `migrator.md`/`AGENTS.md`
  сервисной сессией, D48). Изменены `docs/TRACEABILITY.md` (Q47–Q53),
  `docs/SPECIFICATION.md` §10 (№42–48). Сверка у всех D — ⚪ (процесс/права/
  конфиг; продукт не менялся); подтверждения — чтение канона и
  `rights-probe-2026-09-28.md`. Все Tasks: — («канон внесён сервисной сессией;
  карточка T-15 (F26) — docs-writer»). ID без коллизий (были Q46/D41).
  Отчёт — `.opencode/mail/service-mcp-ready-r2.md`. Следующее: аудит `auditor` +
  приёмка `validator` канон-пакета, коммит `git`; открытые находки — F15/F26/F27.
- **Грабли (F39):** обновление реестра находок — в правах `migrator`
  (`edit docs/analysis/findings-registry.md`, с 28.09.2026); при `permission.rejected`
  в `docs/analysis/**` — не обходить, а докладывать (право внесено D48).
- 2026-09-28 · реестр: по аудиту канон-пакета (P3, закрыты правками сервисной
  сессии в тот же день) добавлены F40–F42 (наследие `opencode run` у `auditor`;
  дубли «эскалации» сверх `dispatch-loop`; косвенная запись `analyst` в реестр —
  точечный deny, F39/D48). Больше ничего не трогал; лента — без изменений;
  коммит — за `git`.
