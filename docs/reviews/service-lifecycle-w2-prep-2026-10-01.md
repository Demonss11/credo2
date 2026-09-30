# Приёмка: service-lifecycle-w2-prep (подготовка волны 2 `TRACEABILITY`)

**Проверка:** сервисная операция без `T-XX` (ветка `develop`; пакет
документно-служебный). Состав: `Q79`, `D83`, карточка `T-20`; сопутствующее —
строки Q78/D82 и Q79/D83 в `TRACEABILITY.md`, каталоги, `tasks/README.md`.

**Версия:** `develop` @ `b04a77a8d85d677904c2d99c3daa9c9a1fa27e19`
(= `origin/develop` = `HEAD`) + рабочее дерево (2026-10-01).

**Вердикт:** принято.

**P1:** — критичных проблем нет.
**P2:** — нет.
**P3:** — нет.

## Проверки (команды → результат)

- `git status --porcelain` → ровно целевой состав до моих записей:
  `M` `.opencode/memory/migrator.md`, `docs/TRACEABILITY.md`,
  `docs/decisions/README.md`, `docs/questions/README.md`, `docs/tasks/README.md`;
  `??` `.opencode/mail/service-lifecycle-w2-prep.md`, `docs/questions/Q79.md`,
  `docs/decisions/D83-traceability-wave2.md`, `docs/tasks/T-20-traceability-wave2/`.
  Плюс мои `M .opencode/memory/validator.md` (чекпойнт) — ожидаемо.
- `git status -sb` → `## develop...origin/develop`; `git rev-parse develop
  origin/develop HEAD` → `b04a77a…` / `b04a77a…` / `b04a77a…` — база верна,
  линейное продолжение после w1 (`git branch --contains cb7d159` → `develop`;
  `git log --oneline -3` → `b04a77a` поверх `cb7d159`).
- `git diff --stat -- src tests Cargo.toml AGENTS.md` → пусто (границы чисты).
- `git diff -- docs/TRACEABILITY.md` → Q78/D82 `open` → `in work` +
  `[T-20](…) ⬜`; добавлена строка Q79/D83 `in work` + `T-20 ⬜`; легенда не
  тронута.
- `git diff -- docs/tasks/README.md` → +строка T-20 в сводке (`P3`,
  `[D83](…) (Q79)`, `⬜`) и абзац «Исключение» ([D83]) о v0.2-задачах со ссылкой
  на решение.
- `git diff -- docs/questions/README.md` → +1 строка (Q79/D83).
- `git diff -- docs/decisions/README.md` → +1 строка (D83/Q79, `accepted`).
- `rg -c "\| open \|"` / `"\| in work \|"` / `"\| done \|" docs/TRACEABILITY.md`
  → 26 / 21 / 32; `rg -c "^| \[Q"` → 79 = 26+21+32 (арифметика точна).
- `rg "resolved|dropped" docs/TRACEABILITY.md` → только легенда (:87–88,
  «`resolved` упразднён», «`dropped` в таблице не используется») — словарь
  `{open, in work, done}` не нарушен, ячеек `resolved`/`dropped` нет.
- `rg -n "\| in work \| —" docs/TRACEABILITY.md` → пусто (exit 1): каждое `in work`
  имеет задачу — правило D82 «`in work` ⇒ открытая задача» держится; `done` с
  задачами — только ✅ (T-03, T-11–T-13).
- Формы Q79: Статус `resolved by D83`, Дата, Приоритет, Связано; разделы
  Контекст/Вопрос/Варианты/Рекомендация — по `journal.md` §4.
- Формы D83: Статус `accepted`, Дата, `Resolves: Q79`, `Спека: —`,
  `Affects`, `Tasks: T-20`; «Сверка с кодом» ⚪ «не применимо» (:61),
  «Задач не требуется сверх T-20» (:80) — по `journal.md` §5.3.
- Карточка T-20: Статус ⬜, P3, Источник `D83 (Q79)`, связано `D82 (Q78)`,
  критерий готовности и DoD-поправка D50 на месте; статус совпадает со сводкой
  `tasks/README.md` (⬜).
- Ссылки: `git ls-files` подтверждает существование всех адресованных файлов
  (Q78, D82, D63–D65, D77, D50, T-18, `tasks/README.md`); относительные пути в
  Q79/D83/T-20 разрешаются (глубина `../`/`../../` верна для их папок).
- D65 (время жизни адреса): `git grep -n
  "opencode/mail|opencode/state|reviews/|research/|analysis/"` по трём новым
  файлам → пусто (exit 1); сессионных адресов нет.
- `agents-perms.mjs` ×2 → идентично, `agents: 11 из 18`; фронтматтеры/права не
  менялись.
- `git diff --check` → пусто.
- `git diff` по `D82`, `D63`, `D64`, `T-18`, `./.opencode/rules/journal.md` →
  пусто (тела решений и правила не переписаны; D82 не тронут — корректно, смена
  статуса строки Q78/D82 идёт через TRACEABILITY по D83, а не правкой D82).

## Что проверено и ок

- **Состав и границы пакета:** 4 `M` (docs) + 4 `??` (новые) + память — ровно
  план ленты; `src/tests/Cargo.toml/AGENTS.md` пусто.
- **Счётчики и словарь `TRACEABILITY`:** 26/21/32, 79 строк, ячеек
  `resolved`/`dropped` нет.
- **Q79/D83/T-20:** шаблоны журнала и карточки, ссылки, отсутствие сессионных
  адресов.
- **Каталоги и реестр:** Q79/D83 в `questions/README.md` и `decisions/README.md`;
  T-20 в сводке `tasks/README.md`; абзац «Исключение» согласован с D83 п.4.
- **Права:** `agents-perms.mjs` ×2 → `11 из 18`, расхождений нет.

## DoD (D50)

Пакет документный: `src/**`, `tests/**`, `Cargo.toml`, `AGENTS.md` не изменены
(`git diff --stat` пусто), счётчики/состав `docs/features/**` не затронуты →
`cargo fmt/clippy/test` не выполнялись (`review.md`, «Порог существенности»;
D50). Код прототипа не меняется — правки зоны `migrator`/сервисной сессии.

## Замечания (не блокеры)

- `D82` «Сверка с кодом» (:66–68) и «Следствия» (:46) описывают состояние **на
  момент принятия D82** (54 `resolved`, строка Q78/D82 `open`). Это исторический
  снимок решения, а не живая перепись; D83 — отдельное решение о волне 2, и его
  существование на D82 не влияет. Информационно, не находка.
- Коммит не выполнялся (`migrator`), гейт пакетного подтверждения впереди.
