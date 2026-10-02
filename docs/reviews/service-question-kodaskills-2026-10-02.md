# Приёмка: service-question-kodaskills (документный пакет, Q81)

**Проверка:** служебная операция `service-question-kodaskills` (класс S) — заведён
ТОЛЬКО вопрос Q81 (изучение внешнего материала KodaSkills); решение `Dn` и задача
`T-XX` не создавались. Проверка формы §4 `journal.md`, согласованности строк в
`docs/questions/README.md` и `docs/TRACEABILITY.md`, границ пакета и D65.
`cargo` не запускался (D50 — кода/тестов/фич не касается).

**Версия:** база `develop` @ `c2f905f` (= `origin/develop` = `HEAD`) + рабочее
дерево (2026-10-02).

**Вердикт:** принято

**P1:** — (критичных проблем нет)

**P2:** —

**P3:** —

## Проверки

- `git status -sb` → `## develop...origin/develop`; изменений вне рабочего дерева нет.
- `git rev-parse HEAD develop origin/develop` → `c2f905f` / `c2f905f` / `c2f905f`
  (база совпадает, ветка — `develop`, штатная).
- `git status --porcelain` → ровно целевой состав: `M docs/TRACEABILITY.md`,
  `M docs/questions/README.md`, `?? docs/questions/Q81.md`,
  `?? .opencode/mail/service-question-kodaskills.md`, `M .opencode/memory/migrator.md`,
  `M .opencode/state/current/{current_state,next_action,progress}.yaml` (аналитик/lead),
  плюс артефакты приёмки. `src/`, `tests/`, `Cargo.toml` не тронуты.
- `git diff --stat` → `docs/TRACEABILITY.md` +1, `docs/questions/README.md` +1;
  прочие — memory/state (штатные, не код).
- `git diff -- docs/TRACEABILITY.md docs/questions/README.md` → ровно по одной
  добавленной строке Q81, без иных правок.
- `git diff --check` → пусто (пробельные дефекты отсутствуют).
- **Q81 — номер:** `git ls-files docs/questions` → максимум `Q80.md` (Q81 ещё
  untracked как новый); файлов `Q82`/дублей `Q81` нет → номер свободен.
- **Форма §4:** `docs/questions/Q81.md` — заголовок `# Q81: …`; поля
  Статус/Дата/Приоритет/Связано; разделы Контекст · Вопрос · Варианты ·
  Рекомендация. Статус `open`; Дата `2026-10-02`; Приоритет 🟡; Связано `—`.
- **Отсутствие D-решения/T-XX:** поля `Resolves` нет; `rg "Resolves|T-" Q81.md`
  пусто; `rg "Q81" docs/decisions docs/tasks docs/features` пусто (решение и
  задача не заведены).
- **Строки каталогов согласованы:** `TRACEABILITY.md:85` →
  `| [Q81](questions/Q81.md) | — | open | — | — |`;
  `questions/README.md:103` →
  `| [Q81](Q81.md) | применимость внешнего материала KodaSkills (\`skills\`) к CREDO | — |`.
  D = `—`, жизненный цикл `open`, задач нет, реализации нет — совпадают между
  собой и с телом Q81. Колонки Q/D — только ссылки (D76).
- **§8/`open`:** обоснование «решение не принято» согласуется с
  легендой `TRACEABILITY.md:87–88` и §3 `journal.md`; трактовка `open` —
  `D82` (Решение п.1: «`open` — есть вопрос (решения нет …)») и `Q79`.
  Свежая запись `open` без `Dn` — легитимное состояние жизненного цикла,
  не антипаттерн §8.
- **D65 (несессионные адреса):** `rg "\.opencode/(mail|state)|docs/(reviews|research|analysis)" docs/questions/Q81.md`
  → пусто (exit 1). Новые строки README/TRACEABILITY сессионных адресов не несут.
- **Номера строк не цитируются:** `rg ":\d+" docs/questions/Q81.md` → пусто.
- **Ссылки живые:** `../decisions/D82-traceability-lifecycle-waves.md` и `Q79.md`
  существуют (`git ls-files` подтверждает). URL материала дословно:
  `https://github.com/XCode-NLP/KodaSkills/tree/main/skills`.
- **Обновлять нечего:** счётчиков `open/in work/done` в `TRACEABILITY.md` и
  `questions/README.md` нет (только легенда); `decisions/README.md` не
  затрагивается (D не заводился). Правка одного факта — один канон.

## Что проверено и ок

- Границы пакета: `src/**`, `tests/**`, `Cargo.toml`, `docs/features/**`,
  `docs/decisions/**`, `docs/tasks/**`, `.opencode/rules/**`, `AGENTS.md` — не
  тронуты.
- Состав Q81, значения `resolved`/`dropped` (в новых строках отсутствуют),
  легитимность `open`, D65, живые ссылки, отсутствие T-XX/Dn-связи — без замечаний.
- Лента `service-question-kodaskills.md`: назначение, отчёты `migrator`/`lead`,
  `expect` совпал — целостна.

## Точный состав пакета для гейта (коммит + push `develop`)

Целевые документы:
- `docs/questions/Q81.md` (новый)
- `docs/questions/README.md` (M, +1)
- `docs/TRACEABILITY.md` (M, +1)

Операционные (штатные, входят в пакет по плану `surface_to_user`):
- `.opencode/mail/service-question-kodaskills.md` (новый, лента)
- `.opencode/memory/migrator.md` (M, append F35)

Состояние/приёмка:
- `.opencode/state/current/current_state.yaml`, `.opencode/state/current/next_action.yaml`,
  `.opencode/state/current/progress.yaml` (M — analyst/lead)
- `.opencode/state/current/receipts.yaml` (M, квитанция `validator`)
- `.opencode/memory/validator.md` (M, чекпойнт `validator`)
- `docs/reviews/service-question-kodaskills-2026-10-02.md` (новый, этот отчёт)

Перед гейтом состав сверить с `git status --porcelain` на момент вызова `git`.
