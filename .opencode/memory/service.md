# Память: сервисная сессия (контур владельца)

- **Назначение:** сервисные операции без задачи (T-15 B0 «wave 0 фазы B», вне
  канона; MCP-ready, процесс агентов).
- **Канон:** `AGENTS.md` §«Память и почта», §«Сервисные операции»;
  `.opencode/rules/dispatch-loop.md` (hard rules).
- **Правило:** чекпойнт — волна/итерация, выполненные шаги, улики, что осталось;
  кратко, со ссылками.

## Чекпойнты

- 02.10.2026, C1 (схема состояния T-15) — открытие: срез `develop` =
  `origin/develop` = `c754c97`; входы — записка §5, D42, F15, отчёты прогона
  02.10; план — owner-gate → `migrator` (`Dn`) → правки канона → `auditor` →
  `validator` → гейт → `git`. В дереве — хвост clean-logs (17 лент + память):
  отдельный коммит либо подхват (решение владельца). Лента —
  `.opencode/mail/service-mcp-ready-r8.md`.
- 02.10.2026, C1 — приёмка accepted (`T-15-c1`, DoD 135/0), **коммит отклонён
  владельцем** («не делаем коммит») — пакет `deferred_by_owner`. Замечания
  владельца: (1) читатели `state-schema.md` — только `analyst` + `validator`
  (lead/git не читают; отдельная роль-хранитель не нужна — машинный контроль
  `validate-state.mjs`); (2) cargo на пакетах без кода не запускать (D50) —
  бриф приёмки ошибочно требовал полный DoD (отход зафиксирован в ленте r8).
  Правки внесены; переприёмка — адресная, без cargo.
- 02.10.2026, C1 закрыта: коммит `d414998`, push `2f5544c..d414998`
  (origin/develop); схема `state-schema.md` в каноне (D86/Q83), F15 закрыт,
  приёмка `T-15-c1` (r2 — адресная, без cargo). Замечания владельца учтены
  (читатели схемы: analyst+validator; D50 — тесты только по составу пакета).
  Дальше по T-15 — C2/C3/C5/C6 или чистый S-прогон (выбор владельца).
- 02.10.2026, T-20 закрыта: коммит `70f690e`, merge `caac10d` в develop,
  ветка удалена (origin — вручную: CCSN блокирует `git push --delete`;
  локально — роль `git`). Итог: 9 → done, Q22 → in work + T-24, 16 → open
  (v0.2), Q81 open, Q78/Q79 → done; приёмка `-r3` (два rework: битая ссылка,
  синк ячеек), финальный аудит L — чисто; cargo на документном пакете не
  запускался (D50). Дальше — S-пилот на T-24 (F26/F27).
- 02.10.2026, r9 «дешёвые правки» по T-24: Q84→D87; права (lead + rev-parse;
  git + branch --list/ls-remote; tester + clippy; analyst — заметка), CCSN-пункт
  в git-workflow, stale-детекция в dispatch-loop, модель lead `#default`;
  reload выполнен, права подтверждены; MCP churn/restore (F65). Дальше —
  auditor → validator → гейт → git.
- 02.10.2026, r9 закрыта: коммит `af53e23`, push `6c28e51..af53e23` (develop);
  права/CCSN/state/модель (D87), F69–F71 закрыты, C14 ✅. Остаток — C9/C12
  (F67/F68). Пост-пакетные записи — подхват.
- 02.10.2026, r10 закрыта (C9/C12): коммит `2958bf5`, push `af53e23..2958bf5`
  (develop). Лимит `lead` 24, записи 1/действие + сегментный итог, глубокий
  план, resume без `analyst`, дробление L, сплит лент (D88); F58/F67/F68/F72
  закрыты; C9/C12 ✅. Проверка — следующим прогоном (F26/F27).
- 03.10.2026, r11 закрыта (P1/P2/P3): коммит `6cd75cf`, push (develop). D89:
  дочерние задачи (ветка от родителя/общий пакет), заморозка решений,
  session-commit (инкремент 1). C15/C16 ✅, C3 🚧, F80–F82; F74 — первое
  применение (docs_journal 14/0). Остаток: C3 process/runN/теги/фасад; F74
  правило в D50; F26/F27 — прогон.
- 03.10.2026, r12 закрыта (C5/C7): коммит `7e458fd`, push (develop). D90:
  re-raise (объект/4 категории/зеркало) + шаблон сегментного итога + фичи
  (re-raise ✅, state-schema/session-checkpoint 🟡). C5/C7 ✅; F61 — пометка.
  docs_journal на приёмке поймал P1 (вилдкард `agents-*` в TRACEABILITY) →
  фикс → `-r2` 14/0. Остаток T-15: C2, C4+C3-остаток, C6, C8.
- 03.10.2026, r13 открыта (C2 — `validate-state.mjs`): срез `develop` =
  `origin/develop` = `7e458fd` (r12). План: `migrator` (Q88 → D91 + карточка),
  скрипт (служебная зона), правки канона (ссылки в `state-schema.md`/
  `dispatch-loop.md`/`validator.md`/`review.md`/`AGENTS.md`), `auditor`,
  `validator` (адресная + схемный прогон), гейт → `git`. Лента —
  `.opencode/mail/service-mcp-ready-r13.md`. `C2` — только процесс-слой
  (продуктовый код не затрагивается). Заметка: чужая лента r11 имеет смешанную
  кодировку (хвост — битые байты) — не трогается.
- 03.10.2026, r13 (C2): скрипт `.opencode/scripts/validate-state.mjs` (D91) —
  валидатор схемы состояния; точки применения — pre-flight (`lead`) и приёмка
  (`validator`); права добавлены обеим ролям. Аудит `auditor` — P2 ×5 + P3
  (карточка «✅» без приёмки; нет права `lead`; единое `iteration` по 2 из
  4 артефактов; `question`; граница мягкого режима; форма `review.md`) —
  исправлены сервисной сессией. Приёмка `validator` — **принято** (`service-c2`,
  iteration 1); схемный прогон 0 ошибок; `docs_journal` 14/0; `C2` ✅; Q88
  остаётся `in work` (T-15 🚧). Грабля: `validator` упёрлась в лимит шагов до
  записи артефактов — отчёт/квитанцию оформила сервисная сессия; фикстуры
  приёмки убраны до пакета.
- 03.10.2026, r13 закрыта (C2): коммит `eeac481`, push `7e458fd..eeac481`
  (`origin/develop`). D91: валидатор схемы `.opencode/scripts/validate-state.mjs`;
  точки применения — pre-flight (`lead`) и приёмка (`validator`); права обеим
  ролям; канон `state-schema.md`/`dispatch-loop.md`/`review.md`/`AGENTS.md`
  обновлён. Приёмка `service-c2` (iteration 1, accepted): схемный прогон
  0 ошибок, `docs_journal` 14/0; аудит — P2×5+P3 исправлены. `C2` ✅, Q88
  `in work` (T-15 🚧), фича `agents-state-schema` ✅. Остаток T-15: C4+C3-остаток,
  C6, C8.
- 03.10.2026, r14 закрыта (предикат зачёта): коммит `42e1296`, push
  `eeac481..42e1296` (develop). D92: раздел «Зачётный прогон» в `state-schema.md`
  (ядро: ведущий `lead`, нет упоров `steps`, нет незапланированных прерываний
  владельца, нет ручных восстановлений; не-дефекты — плановые гейты,
  `owner_override`, rework, CCSN, session-commit; машинная проверка;
  неретроактивно). Приёмка `service-credited-run` (iteration 1, accepted),
  `docs_journal` 14/0; аудит — P3 ×2 (путь `metrics-report.mjs`, тавтология
  условия 3) исправлены. `C17` ✅; заведены F83 (`.credo/**` в git), F84 (дрейф
  полей live state).
- 03.10.2026, r15 закрыта (C3-остаток + C4): D93 — session-commit: ветка сессии
  `process/<прогон>-s<M>` от текущего HEAD, тег `session/<прогон>-s<M>`,
  сообщение `chore(process): <прогон> s<M>`, снапшот
  `state/snapshots/<прогон>-s<M>/` (режим `--snapshot` в
  `session-checkpoint.mjs`; право у роли `git`), push ветки и тега, без merge в
  `develop`; сервисные операции — без process-ветки. Фасад
  `.opencode/commands/git/checkpoint.md` (+ `status.md` — теги сессий);
  `.opencode/commands/**` — в области аудита `auditor`. Приёмка
  `service-process-branch` (iteration 1, accepted), `docs_journal` 14/0; аудит —
  P2 ×2 (`review.md` — список команд `git`; карта фич — имя прогона), свежий
  аудит чист. `C3`/`C4` ✅, фича `agents-session-checkpoint` ✅. Грабля:
  накопительная ветка `process/<прогон>` конфликтует с M-файлами на общем
  дереве — принята ветка сессии (`switch -c` от текущего HEAD).
- 03.10.2026, r16 (C11 — B2 профиль-флаг): плагин `token-guard.ts` —
  `B2_PROFILE ∈ "off"|"codemode"|"mcp"` (по умолчанию `off`; таблицы обоих
  профилей, `activeB2Prefixes()`, `off` — no-op; B1 не изменён); тест
  `.opencode/scripts/token-guard-test.mjs` (синтетика профилей + зеркало
  констант + B1-регрессия; PASS, 24 ok). `AGENTS.md` — сноска о профиле; `D45`
  п.3 — пометка «уточнено D94». Приёмка `service-b2-profile` (iteration 1,
  accepted), `docs_journal` 14/0; аудит — P2 (тест не сверял зеркало — закрыт
  парсером+assert; P3 — дубль пометки D45) исправлены. `C11` 🟢 «подготовлен и
  принят» (не ✅ — живой проверки в MCP-канале не было), closeout не держит;
  Q91 `in work` (T-15 🚧). Грабля: прогон `token-guard-test.mjs` недоступен
  ролям без права (by design) — R2 возможен только владельцем/сервисной сессией.
  Дальше: открытие фазы E (отдельная задача).
- 03.10.2026, r17 (открытие фазы E): D95 — фаза E как отдельная задача
  `T-26-mcp-server-design` (⬜, P1; после заморозки — фаза D T-15); предмет —
  только process-MCP (6–10 инструментов: storage + validation + query, решений не
  принимает); git-MCP (Прил. A) вне предмета; класс априори L; вход — B2 (D94).
  Создано: Q92, D95, `docs/tasks/T-26-mcp-server-design/README.md`, реестр,
  `TRACEABILITY:96`, карточка T-15 (фаза E → T-26); фича `agents-mcp-readiness`
  не тронута (поведение не менялось). Приёмка `service-phase-e` (iteration 1,
  accepted): `docs_journal` 14/0, `features_inventory` 4/0. Аудит — P2 (нет
  чекпойнта migrator за r17 — лимит шагов; дозаписан сервисной сессией) закрыт.
  Грабля: `migrator` (18 шагов) не успел отчёт/чекпойнт — сервисная дозапись;
  правки `docs/features/**` недоступны `migrator` (зона `docs-writer`).
  Дальше: C6/C8 + прогон + заморозка (фаза D) → старт T-26.
- 03.10.2026, r18 (C6 — метрики из состояния): D96/Q93 — скрипт
  `.opencode/scripts/state-metrics.mjs` (метрики/очереди из `progress`/
  `receipts`/`current_state`/`next_action`; CLI + `--json`/`--task`/`--session`/
  `--out`/`--dir`; мини-YAML-парсер со вложенными мапами/списками). Канон:
  `dispatch-loop.md` §«Состояние и записи» — ссылка (без таблиц); `AGENTS.md` —
  карта. Права: `state-metrics.mjs` добавлен `validator`+`lead` (+`review.md`).
  Фича `agents-metrics` ✅. Приёмка `service-state-metrics` (-r2, accepted):
  `docs_journal` 14/0, `features_inventory` 4/4, прогон скрипта R2 (полный/JSON/
  фильтры/`--out`). Аудит — P2 ×5 + P3 (мёртвая `verdicts`) закрыты. Грабля:
  права на новый скрипт забыты при первом заходе (P2-1 rework) — добавлять сразу
  в пакет с скриптом. Хвост данных: `replan_reason` в `progress` не пишется →
  категории re-plan «(не указана)», конвергенция = total (не дефект скрипта;
  отдельное решение). `C6` ✅; Q93 `in work` (T-15 🚧). Остаток T-15: C8
  (кандидаты B0). Дальше: фаза D (чек-лист §8, зачётный прогон, заморозка).
