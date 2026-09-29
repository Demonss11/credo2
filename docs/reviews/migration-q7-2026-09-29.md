# Отчёт приёмки: `service-migration-q7` (перенос Q7 → D52)

**Проверка:** перенос журнала **Q7** «Термины в глоссарии» → `D52-glossary-terms-canon`
(§10 задан новой строкой **№52** по BRIEF §7); согласованность журнала,
`TRACEABILITY`, `questions/README`, §10/§11, обратных ссылок; границы пакета; DoD по
составу (D50). Чек-лист — `validator.md` §«Перенос Q/D», правила — `BRIEF.md` §2/§4/§5.3/§5.7/§7.

**Версия:** `develop`, HEAD `5883a17` + рабочее дерево (8 M + 3 `??`), 2026-09-29.

**Вердикт:** принято с замечаниями

**P1:** — (критичных проблем нет)

**P2:** — (критичных проблем нет)

**P3:** `docs/OPEN_QUESTIONS.md:66` — указатель Q7 записан без темы и даты
(`### Q7. → перенесён`), тогда как соседние перенесённые блоки сохраняют тему и дату:
`Q4:46` `### Q4. ✅ \`Приоритет\` входит в MVP? (решено 2026-09-24) → перенесён`,
`Q5:51`, `Q6:60`. Последствие: читатель архива у указателя Q7 теряет контекст
(тему/дату), которого не требуют от соседей; при этом форма буквально совпадает с
шаблоном BRIEF §7 («`### Qn. → перенесён` + ссылки»), поэтому не блокер. Правка
`migrator` — одна строка: `### Q7. ✅ Термины в глоссарии (решено 2026-09-26) → перенесён`.

**Проверки:**

- `git status --porcelain` → 8 M + 3 `??`; среди изменённых только
  `docs/**` (CHANGELOG, OPEN_QUESTIONS, SPECIFICATION, TRACEABILITY,
  questions/README, features/publish.feature) и 2 памяти (`migrator`, `docs-writer`);
  новые — `mail/service-migration-q7.md`, `questions/Q7.md`, `decisions/D52-…md`.
- `git diff --stat` → +43/−33 по 8 файлам, всё `docs/**`/`memory`; `src/**`,
  `tests/**`, `Cargo.toml`, `AGENTS.md`, `.opencode/agents|rules/**`, `docs/tasks/**`
  в статусе отсутствуют.
- `git diff 22f7683..HEAD -- src tests` → пусто (последнее изменение Rust — `22f7683`,
  реформат W8-config).
- `git log -1 --oneline` → `5883a17 chore(agents): Q56/D51 …` (снимок совпал).
- `git diff -- docs/features/publish.feature` → 3+/1−, только шапка (комментарий
  `Q7 → [D52]`); `rg -c "Сценарий:" docs/features/publish.feature` → 7 (без
  изменений); `docs/features/README.md` не в статусе.
- `git diff -- docs/OPEN_QUESTIONS.md` → −36/+… только блок Q7 заменён указателем
  (`:66-69`), полного текста не осталось.
- `rg` по `docs/**` → `D52` только один файл (`glob docs/decisions/D52*`),
  `^\| 52 \|` в SPEC — ровно одна строка `:873`.
- Доступные команды `validator` (`review.md:86-87`) соблюдены; составных команд и
  `git -C` не использовано.

**Что проверено и ок:**

- **Архив (BRIEF §7):** `OPEN_QUESTIONS.md:66-69` — короткий указатель, полного
  текста Q7 нет; ID `Q7` и дата `2026-09-26` сохранены в `questions/Q7.md:4` и
  `decisions/D52-…md:4`. Историческая фраза `OPEN_QUESTIONS.md:443` «открытый Q7»
  (в блоке Q13, ещё не перенесённом) и ссылки на Q4/Q5/Q6 в неперенесённых блоках —
  тот же сложившийся приём; архив не пополнялся (правится только переносимый блок).
- **D52:** `Статус: accepted`, `Дата: 2026-09-26`, `Resolves: [Q7]`,
  `Спека: §10, решение №52`, `Affects` (SPEC §11, `publish.feature`, T-07),
  `Tasks: —` с пояснением про покрытие T-07; стиль ↔ D15…D51. Слаг
  `D52-glossary-terms-canon` уникален.
- **§10 №52:** строка добавлена после №51 (`SPECIFICATION.md:873`), номер уникален,
  стиль ↔ строки №44-51 (ссылка на D, «Q7; полный контекст — …», разделитель `|`).
- **Сверка с кодом (⚪ + факты):** `src/core.rs:71-81` — `CheckMeta` = `name`,
  `version`, `published_at`, `published_by`, `checksum` (+ `deprecated_at`/
  `deprecation_reason`); `display_name`/`source_hash`/`compiler_version` отсутствуют —
  расхождение подтверждено, покрыто T-07. `src/lib.rs:526-539` — `ManifestEntry`
  = `name`/`active`/`supported`/`deprecated`, `Manifest` = `schema_version`/`generated_at`/
  `service_hash`/`checks` — определения §11 сверены. `SPECIFICATION.md` §11
  (`:884-885`, `:886-901`) содержит все 14 новых статей и 2 уточнённые («Черновик»,
  «Публикация»). Задач не требуется, T-07 (`tasks/README.md:49`, «Источник: Q13, Q7»)
  уже покрывает расхождение `meta.json`.
- **Согласованность:** `Q7` ↔ `D52` ↔ `TRACEABILITY.md:14` ↔ `questions/README.md:31`
  ↔ §10 №52; статус `resolved`; «Задачи: T-07» ↔ `tasks/README.md:49`. Все
  относительные ссылки резолвятся (`Q7.md`, `D52-…md`, SPEC, `publish.feature`,
  `T-07-meta-fields/README.md`, `OPEN_QUESTIONS.md`, `TRACEABILITY.md`).
- **Обратные ссылки:** `features/publish.feature:7-11` — `[Q7] → [D52]` в шапочной
  пометке, дата `2026-09-26` и текст про Q13 сохранены; `CHANGELOG.md:38-46` — запись
  «Перенос Q7 → D52» в «Документация», после записи Q5/Q6.
- **Границы:** `src/**`, `tests/**`, `Cargo.toml`, `AGENTS.md`,
  `.opencode/agents|rules/**`, `opencode.json`, `docs/tasks/**` не тронуты; архив не
  пополнялся; чужие записи не переписаны (правки 2 памятей — их владельцы).

**DoD (`D50` — cargo не запускался):** пакет не меняет `src/**`, `tests/**`,
`Cargo.toml` (`git diff 22f7683..HEAD -- src tests` пусто; в `git status` этих путей
нет), следовательно, `cargo fmt`/`clippy`/`test --all` по D50 п.1 не выполняются.
Исключение D50 (правка счётчиков/состава сценариев `docs/features/**` → адресный
`cargo test --test features_inventory`) **не сработало**: правка `publish.feature`
— только строка шапки-комментария (3+/1−), блоки `Сценарий:` и их число (7) не
менялись, `docs/features/README.md` не изменён. DoD пакета — адресные проверки
документов выше.

**Ограничения/технические заметки:** `git diff --stat` для `.md`-файлов выводит
предупреждения `CRLF will be replaced by LF` (настройка `.gitattributes`, не
дефект пакета). Срез `token-guard` при чтении больших файлов не повлиял на выводы:
решения принимались по узким `rg`/`git diff` и точечным чтениям.
