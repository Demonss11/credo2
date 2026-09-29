# CHANGELOG — credo2

## 0.1.0 (в разработке)

> Прототип до релиза API v0.1: перечисленные изменения — намеренные,
> потребителей в проде нет.

### Документация

- **Документация перенесена в `docs/`**: `SPECIFICATION.md`, `GRAMMAR.md`,
  `CHANGELOG.md`, `OPEN_QUESTIONS.md`, `features/`, `tasks/`; относительные
  ссылки и `tests/features_inventory.rs` обновлены.
- **Введён журнал вопросов и решений**: `docs/questions/Qn.md`,
  `docs/decisions/Dn-<слаг>.md` (Dn = номер решения в SPEC §10), сводка связей —
  `docs/TRACEABILITY.md`, процесс — `docs/BRIEF.md`. Пилот миграции — Q1 → D15.
- **Процесс переноса дополнен сверкой с кодом**: у каждого решения — вердикт
  «Сверка с кодом» и решение о задаче (`docs/BRIEF.md` §5.3); стратегия
  «миграция идёт блоками параллельно кодингу» — §8.
- **Перенос Q2, Q3 → D16** (миграция журнала, 28.09.2026): вопросы о каноне
  языка v0.1 и парсере перенесены в журнал
  ([`questions/Q2.md`](questions/Q2.md), [`questions/Q3.md`](questions/Q3.md));
  решение — [`decisions/D16-dsl-canon-regex-mvp.md`](decisions/D16-dsl-canon-regex-mvp.md);
  архивные блоки заменены указателями; сверка с кодом — соответствует
  (дрейф `GRAMMAR` закрывает [`T-14`](tasks/T-14-grammar-message-sync/README.md)).
- **Перенос Q4 → D17** (миграция журнала, 29.09.2026): `Приоритет` вне MVP
  (v0.2 с конвейерами); вопрос — [`questions/Q4.md`](questions/Q4.md),
  решение — [`decisions/D17-priority-out-of-mvp.md`](decisions/D17-priority-out-of-mvp.md);
  сверка с кодом — соответствует, задач не требуется.
- **Перенос Q5, Q6 → D19** (миграция журнала, 29.09.2026): две независимые
  оси меток — статус реализации × приоритет MVP; канон —
  [`features/README.md`](features/README.md), `SPECIFICATION.md` §6 — ссылки
  вместо дублирующей таблицы; вопросы — [`questions/Q5.md`](questions/Q5.md),
  [`questions/Q6.md`](questions/Q6.md), решение —
  [`decisions/D19-statuses-priorities-canon.md`](decisions/D19-statuses-priorities-canon.md);
  Q6 закрыт попутно (таблица §6.1 удалена; счётчики проверяет
  [`tests/features_inventory.rs`](../tests/features_inventory.rs)); сверка с
  кодом — соответствует, задач не требуется.
- **Перенос Q7 → D52** (миграция журнала, 29.09.2026): глоссарий
  ([`SPECIFICATION.md`](SPECIFICATION.md) §11) — терминологический канон:
  14 новых статей + уточнение статей «Черновик» и «Публикация»; вопрос —
  [`questions/Q7.md`](questions/Q7.md), решение —
  [`decisions/D52-glossary-terms-canon.md`](decisions/D52-glossary-terms-canon.md);
  шапочная пометка [`features/publish.feature`](features/publish.feature)
  обновлена (Q13, [Q7](questions/Q7.md) → [D52](decisions/D52-glossary-terms-canon.md));
  сверка с кодом — документная (⚪), задач не требуется (расхождение
  `meta.json` покрывает [T-07](tasks/T-07-meta-fields/README.md)).
- **Перенос Q8–Q10, Q42 → D21** (миграция журнала, 29.09.2026): семантика ядра
  v0.1 — строгие ошибки исполнения (Q8/Q9), словарь решений задаёт банк и
  «не сработало» — пустые `decision`/`reason` (Q10), канон объяснения —
  `snake_case` + `condition` (Q42); вопросы — [`questions/Q8.md`](questions/Q8.md),
  [`Q9.md`](questions/Q9.md), [`Q10.md`](questions/Q10.md),
  [`Q42.md`](questions/Q42.md), решение —
  [`decisions/D21-core-semantics-v01.md`](decisions/D21-core-semantics-v01.md);
  шапочные ссылки D21 добавлены в
  [`features/errors.feature`](features/errors.feature),
  [`explain.feature`](features/explain.feature),
  [`explain_full.feature`](features/explain_full.feature),
  [`execution.feature`](features/execution.feature),
  [`test_draft.feature`](features/test_draft.feature); сверка с кодом —
  ✅ соответствует, задач не требуется (сценарий типов в `errors.feature` —
  целевое v0.2, статус 🟡; `execution.feature` 🟡 из-за `Приоритет`,
  [Q4](questions/Q4.md)).
- **Перенос Q11 → D53** (миграция журнала, 29.09.2026): язык сообщений об
  ошибках — принцип «машина / человек» (сквозной для Q11, Q13, Q42):
  человекочитаемый `message` — русский, машиночитаемое (коды ошибок, ключи JSON,
  идентификаторы, сегменты путей) — латиница `snake_case`; канонические тексты и
  статусы REST-ошибок закреплены (404/410/409/401, `SPECIFICATION.md` §4.4/§4.5);
  вопрос — [`questions/Q11.md`](questions/Q11.md), решение —
  [`decisions/D53-error-messages-language.md`](decisions/D53-error-messages-language.md);
  шапочная пометка D53 добавлена в
  [`features/evaluate.feature`](features/evaluate.feature),
  [`rest_api.feature`](features/rest_api.feature),
  [`rest_auth.feature`](features/rest_auth.feature),
  [`errors.feature`](features/errors.feature); сверка с кодом — ✅ соответствует,
  задач не требуется (`version_deprecated` в MCP — известный резерв, покрыт
  [T-09](tasks/T-09-check-run/README.md) / Q33).
- **Перенос Q12–Q15 → D54, D14, D55, D56** (миграция журнала, 29.09.2026):
  блок «Хранилище, публикация, git» — единый поток источника истины, канон
  артефакта, имя ветки и двухшаговый цикл публикации. **Q12 → D54** — источник
  истины `.dar`-файл; поток `файл → черновик → публикация`; черновик производный,
  устаревание — метка `stale`; вопрос — [`questions/Q12.md`](questions/Q12.md),
  решение — [`decisions/D54-source-of-truth-flow.md`](decisions/D54-source-of-truth-flow.md).
  **Q13 → D14** — артефакт публикации — неизменяемая JSON-тройка
  (`rule.json`/`contract.json`/`meta.json`) в `checks/{name}/{X}/{Y}/{Z}/`,
  «один check = один каталог версии»; вопрос —
  [`questions/Q13.md`](questions/Q13.md), решение —
  [`decisions/D14-published-artifact-canon.md`](decisions/D14-published-artifact-canon.md).
  **Q14 → D55** — имя ветки доставки `publish/{name}-{version}`; `checks/...` —
  путь артефакта, не имя ветки; вопрос —
  [`questions/Q14.md`](questions/Q14.md), решение —
  [`decisions/D55-publish-branch-name.md`](decisions/D55-publish-branch-name.md).
  **Q15 → D56** — двухшаговый цикл публикации: `check.publish` создаёт ветку,
  отдельный явный шаг `credo merge {name} {version}` сливает в `main`
  (ancestor-проверка, CAS, удаление ветки); вопрос —
  [`questions/Q15.md`](questions/Q15.md), решение —
  [`decisions/D56-merge-step.md`](decisions/D56-merge-step.md). Шапочные
  пометки D54/D14/D55/D56 добавлены в затронутые фичи, канон имени ветки — в
  [`features/README.md`](features/README.md); заведена задача
  [`T-17`](tasks/T-17-merge-command/README.md) (`credo merge`; P2 — на
  подтверждение владельца). Сверка с кодом — 🟡 расхождения, покрытые задачами
  ([`T-16`](tasks/T-16-stale-check-test/README.md),
  [`T-08`](tasks/T-08-materialize-source-file/README.md),
  [`T-06`](tasks/T-06-registry-path-xyz/README.md),
  [`T-07`](tasks/T-07-meta-fields/README.md), T-17).
- **Перенос Q16–Q19 → D32, D57, D35, D58** (миграция журнала, 29.09.2026):
  окончание блока «Хранилище, публикация, git». **Q16 → D32** — тест-гейт
  публикации: готовность к публикации — факт успешного `check.test`, метка
  теста (`last_test_checksum`/`tested_at`) хранится в черновике; публикация
  без теста или при расхождении контрольной суммы отклоняется
  (`publish_failed`); вопрос — [`questions/Q16.md`](questions/Q16.md),
  решение — [`decisions/D32-test-gate-mvp.md`](decisions/D32-test-gate-mvp.md).
  **Q17 → D57** — иммутабельность обеспечивает bare-git в три слоя (каталог
  версии, отказ при повторной публикации, CAS при слиянии), без отдельного
  механизма (`refs/rules/...`); вопрос —
  [`questions/Q17.md`](questions/Q17.md), решение —
  [`decisions/D57-bare-git-immutability.md`](decisions/D57-bare-git-immutability.md).
  **Q18 → D35** — semver v0.1: pre-release сравнивается численно, build не
  входит в идентичность версии и отбрасывается `as_storage`; вопрос —
  [`questions/Q18.md`](questions/Q18.md), решение —
  [`decisions/D35-semver-v01.md`](decisions/D35-semver-v01.md).
  **Q19 → D58** — каталоги данных workspace `.credo/` (server) и
  `.dar-notebook/` (Notebook) — в корне workspace, вне git (`.gitignore`);
  вопрос — [`questions/Q19.md`](questions/Q19.md), решение —
  [`decisions/D58-workspace-data-dirs.md`](decisions/D58-workspace-data-dirs.md).
  Шапочные пометки D32/D57/D35/D58 добавлены в затронутые фичи
  (`publish`/`test_draft`/`immutability`/`semver`/`deferred`/`storage_paths`/`notebook_ui`).
  Задачи: [`T-02`](tasks/T-02-test-gate/README.md) (Q16, тест-гейт),
  [`T-06`](tasks/T-06-registry-path-xyz/README.md) и
  [`T-17`](tasks/T-17-merge-command/README.md) (Q17); Q18/Q19 новых задач не
  порождают (сверка ✅/⚪).
- **Сводка решений `decisions/README.md`** (2026-09-29): рабочий обзор
  решений журнала — 29 записей D14–D58 (тема, решаемые вопросы, даты, задачи,
  статус); не канон: текст решения — в `Dn-<слаг>.md`, формулировка —
  `SPECIFICATION.md` §10, связи — `TRACEABILITY.md`; поддержка — `migrator`
  при заведении нового `Dn` тем же изменением, что и файл
  ([`decisions/README.md`](decisions/README.md)).
- **Перенос Q20–Q23 → D22, D23, D26, D25** (миграция журнала, 29.09.2026):
  блок «REST API», часть 1. **Q20 → D22** — канонический путь REST
  `/checks/{name}/versions/{version}/...` (сегмент `/versions/` обязателен;
  конвейерам v0.2 — отдельный ресурс `/pipelines/...`); вопрос —
  [`questions/Q20.md`](questions/Q20.md), решение —
  [`decisions/D22-rest-paths-canon.md`](decisions/D22-rest-paths-canon.md).
  **Q21 → D23** — схема `GET /checks`: манифест
  (`schema_version`/`count`/`service_hash`/`checks`) с `ManifestEntry`
  `name`/`active`/`supported`/`deprecated`; версии по убыванию, `active`
  пуст, если все версии deprecated, поля `kind` нет; вопрос —
  [`questions/Q21.md`](questions/Q21.md), решение —
  [`decisions/D23-get-checks-manifest.md`](decisions/D23-get-checks-manifest.md).
  **Q22 → D26** — аутентификация REST: заголовок `x-api-key` (ключ
  `CREDO_API_KEY`, флаг `--api-key`), открытые пути
  `/health`/`/docs`/`/openapi.json`, без ключа API открыт (демо); вопрос —
  [`questions/Q22.md`](questions/Q22.md), решение —
  [`decisions/D26-rest-auth-x-api-key.md`](decisions/D26-rest-auth-x-api-key.md).
  **Q23 → D25** — единый конверт ошибок REST
  `{"error": {"code", "message"}}` со стабильными кодами 401/404/409/410/422;
  вопрос — [`questions/Q23.md`](questions/Q23.md), решение —
  [`decisions/D25-rest-error-envelope.md`](decisions/D25-rest-error-envelope.md).
  Шапочные пометки D22/D23/D26/D25 добавлены в затронутые фичи
  (`rest_api`/`evaluate`/`batch`/`import_export`/`manifest`/`dashboard`/`rest_auth`/`errors`).
  Сверка с кодом — ✅ по всем четырём, задач не требуется.
- **Перенос Q24–Q26 → D36, D37, D24** (миграция журнала, 29.09.2026): связка
  «границы MVP» — отложенные сценарии (batch, клиентское объяснение,
  импорт/экспорт) вне MVP/v0.2. **Q24 → D36** — массовый прогон (batch) —
  пост-MVP/v0.2; в v0.1 единица исполнения — атомарный `evaluate`, полный
  прогон оркеструет клиент по манифесту, серверный агрегат не нужен; при
  возврате — отдельный эндпоинт `POST /checks/{name}/versions/{version}/batch`,
  ключи латиницей; вопрос — [`questions/Q24.md`](questions/Q24.md), решение —
  [`decisions/D36-batch-deferred.md`](decisions/D36-batch-deferred.md).
  **Q25 → D37** — клиентское объяснение — пост-MVP/v0.2; MCP-инструмент
  `check.explain_client` в MVP не вводится (источника шаблона и «политики
  кредитования» нет), сырьё MVP — `Explanation`; вопрос —
  [`questions/Q25.md`](questions/Q25.md), решение —
  [`decisions/D37-client-explanation-deferred.md`](decisions/D37-client-explanation-deferred.md).
  **Q26 → D24** — импорт/экспорт `.dar` — вне MVP: при источнике истины
  `rules/*.dar` (Q12) обмен вырождается в файловую копию/`git show` и
  `check.create`, отдельный транспорт YAGNI; вопрос —
  [`questions/Q26.md`](questions/Q26.md), решение —
  [`decisions/D24-import-export-deferred.md`](decisions/D24-import-export-deferred.md).
  Шапочные пометки D36/D37/D24 добавлены в затронутые фичи
  (`batch`/`client_explanation`/`import_export`). Сверка с кодом — ⚪ «не
  применимо» (решения об отсрочке), задач не требуется; статусы ⏸/⏳ и
  счётчики `features/README.md` (47/278) не меняются.
- **Перенос Q28–Q29 → D31, D34** (миграция журнала, 29.09.2026): связка
  «MCP-контракты» — контракт `check.create` и контракты MCP-инструментов.
  **Q28 → D31** — `check.create` принимает `{name, source}` (оба обязательны;
  `name` сверяется с заголовком правила), отвечает `{status, name}`; повторный
  вызов перезаписывает черновик («сохранить = обновить»), `expected_kind` и
  `contract` не вводятся; вопрос — [`questions/Q28.md`](questions/Q28.md),
  решение —
  [`decisions/D31-check-create-contract.md`](decisions/D31-check-create-contract.md).
  **Q29 → D34** — общие правила успеха/ошибки, 10 стабильных кодов ошибок
  (различать ошибки следует по `code`, а не по тексту `message`), инварианты
  черновика (`stale`/`test_valid`; `size`/`format` не вводятся) и полные
  JSON-схемы инструментов; сводка — [`SPECIFICATION.md`](SPECIFICATION.md)
  §4.5, полные схемы — D34 (`check.deprecate` требует непустой `reason`);
  вопрос — [`questions/Q29.md`](questions/Q29.md), решение —
  [`decisions/D34-mcp-tool-contracts.md`](decisions/D34-mcp-tool-contracts.md).
  Шапочные пометки D31/D34 добавлены в затронутые фичи
  (`draft`/`agent_minimal`/`mcp_tools`/`test_draft`/`publish`/`deprecation`).
  Задачи связки: [`T-03`](tasks/T-03-check-create/README.md) (сделана),
  [`T-04`](tasks/T-04-mcp-errors/README.md) (сделана),
  [`T-05`](tasks/T-05-mcp-success-schemas/README.md) (открыта; схемы успеха —
  единственная открытая по связке). Сверка с кодом — ✅ (Q28) и 🟡 (Q29,
  расхождение покрыто T-05); статусы и счётчики `features/README.md` (47/278)
  не меняются.

### Процесс

- **Рабочая группа агентов** (`.opencode/agents/`): `lead` (оркестратор,
  `default_agent`) + `migrator`, `docs-writer`, `coder`, `tester`, `validator`,
  `git`; маршруты и права ролей — `AGENTS.md` §Рабочая группа агентов.
- **Ревизия рабочей группы** (2026-09-26): `migrator` ведёт журнал целиком
  (перенос + новые Q/D); маршрут кода замкнут через `docs-writer` (закрытие
  статусов задачи); исправлен порядок прав `docs-writer` (журнал защищён);
  `lead` запускает только роли команды; merge веток публикаций CREDO — за
  человеком; удалён неиспользуемый комплект `.opencode/rules/doc-*.md`.
- **Усиление ролей по образцу проекта ex1** (2026-09-26): правила
  `.opencode/rules/workspace.md` (гигиена поиска и чтения) и
  `.opencode/rules/review.md` (методика ревью: что искать, блокеры, версия
  артефакта); в промпты добавлены методика приёмки, работа с глоссарием
  (`SPECIFICATION.md` §11), «Полезные вызовы» у `lead` и чтение больших
  файлов по карте заголовков.
- **Хранимые отчёты приёмки и роль `researcher`** (2026-09-26): `validator`
  сохраняет отчёты в `docs/reviews/` (единственная его зона записи; улики,
  не канон); новая роль `researcher` — внешние обзоры в `docs/research/`
  (`webfetch`/`websearch` разрешены только ей); карта — `docs/README.md`.
- **Роль `rust-expert` и разгрузка кода** (2026-09-26): код проходит
  идиоматическую вычитку у `rust-expert` (skill `rust-skills`, бюджет чтения,
  правки без изменения поведения); у `coder` и `tester` skill `rust-skills`
  отключён; маршрут кода — `coder` → `rust-expert` → `tester` → `validator` →
  `docs-writer` → `git`.
- **Primary-агент `auditor`** (2026-09-26): аудит системы агентов и
  мета-документации (дубли, противоречия, пробелы, ясность, экономия токенов);
  правит `AGENTS.md`, `.opencode/agents/**`, `.opencode/rules/**`; вне маршрутов
  команды, запускается владельцем отдельной сессией. *(Позже в тот же день
  пересмотрено: `mode: all`, канон не правит — схема A, см. «Цикл агентов v2».)*
- **Корень проекта** — `prototypes/credo2` (`AGENTS.md`, `opencode.json`);
  MCP `credo` работает с рабочей директорией репозитория, данные — `.credo/` (вне git).
- **Цикл агентов v2** (T-11, 2026-09-26): Agile-петля с размерными маршрутами
  S/M/L (`coder → rust-expert → tester → validator → docs-writer → git`);
  `validator` — единственная роль с `cargo test` (полный DoD-прогон), роли,
  работающие с кодом, ограничиваются компиляцией (`fmt`/`check`/`clippy`).
  Память роли — `.opencode/memory/<роль>.md`, лента задачи —
  `.opencode/mail/T-XX.md` (рабочие данные в git, не канон, Q41); чекпойнт
  до/после тяжёлых операций, лимиты `steps` и продолжение по `sessionID`;
  git — пакетное подтверждение после приёмки и идемпотентность при обрыве;
  канон агентов правит сервисная сессия владельца, приёмка — `auditor`
  (`mode: all`) и `validator`; патч v2.1 (по отзыву на бриф) — порог
  существенности повторной проверки, размерные маршруты с риском и guard,
  периодический чекпойнт `K ≈ steps/3`, право `opencode debug agents`
  у `validator`. Целевые сценарии — `features/agents-*.feature`
  (6 файлов / 30 сценариев; статусы — [`features/README.md`](features/README.md)),
  задача — [T-11](tasks/T-11-agent-cycle/README.md), решение —
  [D38](decisions/D38-agent-cycle.md) (Q43).
- **Цикл агентов v3** (T-12, 2026-09-27, D39): `lead` — loop-диспетчер
  (исполняет `.opencode/state/current/next_action.yaml` буквально, решений не
  принимает), эфемерный `analyst` ведёт досье `docs/analysis/<T-XX>-<дата>.md`
  и план; состояние цикла — `.opencode/state/current/` (`next_action`/
  `current_state` пишет `analyst`, `progress` — `lead`, `receipts` —
  `validator`; вне git); fast path класса S, scope-решения — в журнал через
  `migrator` до исполнения, приёмка закрывается квитанцией; R2/R7 и `review.md`
  (порог H5, отчёты `-rN`) сохраняются. Правило цикла —
  `.opencode/rules/dispatch-loop.md`, канон — `AGENTS.md` §Рабочая группа
  агентов, досье — [`docs/analysis/`](analysis/README.md); сценарии —
  `features/agents-*.feature` (6 файлов / 36 сценариев; статусы —
  [`features/README.md`](features/README.md)), задача —
  [T-12](tasks/T-12-agent-loop/README.md), решение —
  [D39](decisions/D39-loop-dispatcher.md) (Q44).
- **Доработка агентов после Run 3** (T-13, 2026-09-27, D40/D41): по итогам
  пилота Run 3 уточнён контракт диспетчера. Маршрут M допускает исключение роли
  с письменным обоснованием в досье (`route_exclusions`; для L исключений нет);
  scope-порог узкий — решение заводится только при отвергнутой альтернативе или
  частичном покрытии, унаследованные границы фиксируются полем
  `inherited_boundaries` без записи в журнал (D40). `analyst` сверяет
  `package.add_paths` со снимком `git status --porcelain` до гейта, описан
  частичный пакет и override владельца (`cancelled_by_owner`/`deferred`);
  `surface_to_user` — всегда `question` с `channel: question|text`; авторитет
  статуса — `receipts.yaml`, `current_state.yaml` — снимок с `as_of`; действие
  завершается записью в `progress.yaml`; введены порог меморандума, реестр
  находок `docs/analysis/findings-registry.md` и pre-flight чек-лист
  `docs/analysis/run-checklist.md`. Правила команд: одиночные (без `;`, `|`,
  `>`), `git -C` вне прав, точечные пути `./…` (D41). Задача —
  [T-13](tasks/T-13-agent-hardening/README.md), отчёт приёмки —
  [`reviews/T-13-2026-09-27.md`](reviews/T-13-2026-09-27.md), решения —
  [D40](decisions/D40-scope-threshold.md) (Q45),
  [D41](decisions/D41-dispatch-refinements.md) (Q46).
- **Процессный пакет W8/Run 5** (2026-09-28, D42–D48): по итогам Run 5 (T-04,
  класс L) уточнён канон агентов — `auditor` получил право
  `edit .opencode/mail/**` (отчёт в ленту задачи), реестр находок закреплён за
  `migrator` (`docs/analysis/findings-registry.md`), введены правило эскалации
  при затыке и «бриф ↔ канон» — стоп-фактор, R2-формулировки и микропроверка
  «память ↔ канон», протокол срезанного вывода, сплит лент; `steps` `coder`
  52 / `tester` 36, бюджет чтения `analyst` — 12; разделение продуктовых и
  процессных коммитов (`state/**` — в процессном пакете), порядок удаления
  ветки при устаревшем upstream и место хешей closeout.
- **Конфиг-пакет качества** (2026-09-28, D45): `rustfmt.toml` (стиль 80 +
  `edition = "2024"`), `rust-toolchain.toml` (`1.96.0`), `.cargo/config.toml`
  (`-D warnings`), `.gitattributes` (`* text=auto eol=lf`); удалён инертный
  `clippy.toml`; реформат `src`/`tests` (10 файлов, только форматирование).
  Волна 0 закреплена: A (`formatter`), B1 — норма, B2 — отключён, deny
  `execute` (`docs-writer`/`git`); зависимости плагина (`package.json`,
  `package-lock.json`) — в git, шаг `npm ci` — в `AGENTS.md`. Решения —
  [D42](decisions/D42-expect-iteration.md)–[D48](decisions/D48-findings-registry-owner.md);
  отчёты приёмки — [`reviews/W8-config-2026-09-28.md`](reviews/W8-config-2026-09-28.md)
  и [`reviews/W8-canon-2026-09-28.md`](reviews/W8-canon-2026-09-28.md); разбор —
  [меморандум W8, том 2](analysis/memorandum-W8-run5.md).
- **Закрытие T-11 и задача T-16** (2026-09-28): цикл агентов (T-11, D38/Q43)
  закрыт — критерий пилота выполнен в Run 4 (T-03): возврат на доработку P1 и
  повторная приёмка `-r2` (отчёты [`reviews/T-03-2026-09-27.md`](reviews/T-03-2026-09-27.md),
  [`reviews/T-03-2026-09-27-r2.md`](reviews/T-03-2026-09-27-r2.md)); повторные
  аудиты 28.09.2026 (W8) расхождений «инструкция ↔ права» не нашли; ветка
  `exp/agent-cycle-rerun` влита в `develop`. Заведена
  [T-16](tasks/T-16-stale-check-test/README.md) — `check.test` на
  stale-черновике исполняет текст файла `rules/{name}.dar` (Q12; находка H7).
  Хвосты процесса — [T-15](tasks/T-15-mcp-ready-process/README.md).

### Добавлено

- **Черновик хранит исходник (T-01).** `Draft` (`src/lib.rs`) хранит `source`
  и `source_hash` (sha256 текста), а не только разобранное правило;
  `check.get_draft`/`check.list_drafts` отдают рендеренные строки, внутренний
  `Rule` не публикуется. Введены вычисляемые `stale` и `test_valid` (Q29,
  инварианты 2–5 §4.5); `check.test` фиксирует `last_test_checksum`/`tested_at`.
  Карточка задачи — [`tasks/T-01-draft-source-hash/README.md`](tasks/T-01-draft-source-hash/README.md),
  отчёт приёмки — [`reviews/T-01-2026-09-26.md`](reviews/T-01-2026-09-26.md);
  автотесты — [`../tests/mcp_draft.rs`](../tests/mcp_draft.rs) (реальный stdio-MCP).
- **`check.create` принимает `{name, source}` (T-03).** MCP-инструмент требует
  оба параметра; `name` сверяется с заголовком `Правило {name}` (`GRAMMAR.md`),
  ответ — `{status: "ok", name}` (Q28, `SPECIFICATION.md` §4.5); повторный
  вызов перезаписывает черновик (upsert). Карточка задачи —
  [`tasks/T-03-check-create/README.md`](tasks/T-03-check-create/README.md),
  отчёт приёмки — [`reviews/T-03-2026-09-27-r2.md`](reviews/T-03-2026-09-27-r2.md).
- **Единый конверт ошибок MCP и 10 стабильных кодов (T-04).** Ошибочные ответы
  MCP возвращают `isError = true` и `{"error": {"code", "message"}}` (ключи
  ровно `code`/`message`); `message` — русский (Q11), `code` — латиница
  `snake_case`. Коды §4.5 (Q29): `validation_failed`, `draft_not_found`,
  `evaluation_failed`, `publish_failed`, `version_not_found`,
  `version_deprecated`, `deprecation_conflict`, `manifest_error`,
  `unknown_tool`, `internal_error` (`version_deprecated` зарезервирован под
  `check.run` — Q33, вне MVP). Конверт REST (Q23) не менялся. Карточка задачи —
  [`tasks/T-04-mcp-errors/README.md`](tasks/T-04-mcp-errors/README.md),
  отчёт приёмки — [`reviews/T-04-2026-09-28.md`](reviews/T-04-2026-09-28.md)
  (`cargo test --all` → 107 passed / 0 failed).

### Исправлено

- **Кириллические имена проверок не попадали в кэш.** `git ls-tree`
  экранировал не-ASCII пути (`core.quotepath`), и `list_from_ref` их
  пропускал: после merge проверка с русским именем не появлялась в
  манифесте/REST. Дерево читается с `-z` (регрессионный тест
  `cyrillic_check_name_is_listed`).

### Ломающие изменения

- **Ответ `check.create` (T-01).** Формат ответа изменён на `{status: "ok", name}`
  (канон Q28, `SPECIFICATION.md` §4.5) — ломающее для MCP-клиентов, читавших
  прежний ответ. Остаток T-03 (обязательный `name`, сверка с заголовком) не
  входит в это изменение.
- **Тело ошибок REST (Q23).** Все ответы с ненулевым статусом переведены на
  единый конверт `{"error": {"code", "message"}}` (было `{"error": "<строка>"}`).
  Коды: `check_not_found`, `version_not_found`, `version_deprecated`,
  `activation_unavailable`, `evaluation_failed`, `unauthorized`; `message` —
  русский текст Q11.
- **Семантика версий (Q11/Q21).** Исполнение deprecated-версии —
  `410 Gone` (`version_deprecated`); active-эндпоинт при отсутствии
  active-версии — `409 Conflict` (`activation_unavailable`); 404-текст
  проверки — `проверка не найдена: {name}` (без кавычек).
- **`GET /checks` (Q21).** В ответ добавлен `schema_version`; `active` —
  максимальная supported-версия, `""` если все версии deprecated;
  `supported`/`deprecated` — по убыванию.
- **Конверт ошибок MCP (T-04).** Ошибочные ответы переведены на единый конверт
  `{"error": {"code", "message"}}` с `isError = true`; плоский конверт ошибки
  больше не используется. Ломающее для MCP-клиентов, читавших прежний формат
  ошибки.
