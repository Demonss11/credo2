# Приёмка (повторная, r2): операция `service-statuses-review`

**Проверка:** повторная независимая приёмка после возврата (iteration 1,
`rework`): закрытие P1 (база/ветка) и P2 (второй блок «Цикл задачи»), проверка
согласованного объёма пакета при базе `master`, записи (лента, память, реестр).

**Версия:** ветка `master` @ `9173fc4` (= `origin/master`) + рабочее дерево;
`develop` @ `3d572f3` (= `origin/develop`). Снимок 30.09.2026.

**Вердикт:** **принято**.

**P1:** — **закрыт.** Лента больше не противоречит дереву: решение владельца
(база/ветка пакета — `master`, режим — коммит прямо в `master` по гейту,
`develop` не трогаем) зафиксировано в ленте
(`.opencode/mail/service-statuses-review.md:222–242`). Проверка базы:
`git status -sb` → `## master...origin/master`; `git rev-parse HEAD develop
origin/master origin/develop` → `9173fc4` / `3d572f3` / `9173fc4` / `3d572f3`
(master и develop в синхроне с `origin`). Возвратный риск («на `develop` приехал
бы лишний блок `Цикл задачи`») снят вторым фиксом: `git diff develop --
./.opencode/agents/validator.md` → **только** §10-ханк, второго блока в диффе
против `develop` нет.

**P2:** — **закрыт.** Второй раздел `## Цикл задачи` снят: `rg -n
"^## Цикл задачи" .opencode/agents/validator.md` → одно вхождение (`:52`);
`git diff -- ./.opencode/agents/validator.md` → удалён блок `−13` строк
(`:69–80`) плюс снятый пункт §10 (`−1`, с пунктуацией `;`→`.`), итого `+1/−15`;
ханки только в теле (`@@ -66`, `@@ -87`), фронтматтер `permissions`/`steps`
не тронут (права — `agents-perms.mjs`, `steps=36`). Оставленная версия `:52–67`
— новее (содержит пункт о квитанции в `receipts.yaml`), её внутренняя ссылка
«чек-лист «Код» ниже» (`:60`) резолвится (**Код (`T-XX`):** — `:79`).

**P3:** — нет.

**Проверки:**
- `git status --porcelain` → 8 `M` + 2 `??`, ровно ожидаемый состав: `M`
  `.opencode/agents/migrator.md`, `.opencode/agents/validator.md`,
  `.opencode/memory/{migrator,validator,auditor}.md`,
  `.opencode/state/current/receipts.yaml`, `docs/decisions/{D38-agent-cycle,D70-spec-reduction}.md`;
  `??` `.opencode/mail/service-statuses-review.md`,
  `docs/reviews/service-statuses-review-2026-09-30.md`.
- `git diff --numstat` → migrator.md `+2/−7`, validator.md `+1/−15`, D38 `+7/−0`,
  D70 `+8/−0`, memory/auditor `+43/−0`, memory/migrator `+39/−0`,
  memory/validator `+19/−1`, receipts `+16/−0`.
- `git diff -- ./.opencode/agents/migrator.md` → без изменений с прошлой приёмки:
  снят абзац-дубль «Память и почта» (−3), снят пункт §10 (−2), перенумерация
  `6→5`, `7→6` («Порядок работы» `1–6`).
- `git diff -- ./docs/decisions/D38-agent-cycle.md` → один пункт
  «**Обновление 30.09.2026**» (+7) в конце «Следствий», перед «Сверка с кодом»;
  описывает **оба** дубля: абзац `migrator.md` и второй раздел `validator.md`,
  обе ревизии канона T-11; тело решения и прочие пункты не переписаны, только
  стабильные пути (D65).
- `git diff -- ./docs/decisions/D70-spec-reduction.md` → один пункт
  «**Обновление 30.09.2026**» (+8), остатки §10; ссылка
  `../tasks/T-18-docs-journal-test/README.md` жива; сессионных адресов нет.
- `git diff --stat -- src tests Cargo.toml AGENTS.md` → пусто.
- `docs/TRACEABILITY.md` — не в `git status` (статусы не тронуты).
- `node .opencode/scripts/agents-perms.mjs` ×2 → идентично, `agents: 11 из 18`,
  расхождений нет; права `validator` совпадают с `review.md:88–90`.
- DoD (D50): `cargo fmt/clippy/test` **не запускались** — пакет документно-
  служебный (`.opencode/agents+memory`, `.opencode/state`, `docs/decisions`,
  `docs/reviews`, лента), в нём нет изменений `src/**`, `tests/**`, `Cargo.toml`;
  исключение для счётчиков `docs/features/**` не сработало
  (`.opencode/rules/review.md:36–39`).
- Технические проблемы: отклонённых команд в этом прогоне не было; срезов
  `token-guard` не было.

**Что проверено и ок:** закрытие P1/P2 подтверждено машинно (`develop`-дифф
`validator.md` — только §10; единственное «Цикл задачи»); объём пакета
соответствует согласованному с владельцем; фронтматтеры обоих канонов не
тронуты; `docs/TRACEABILITY.md` и `AGENTS.md` вне диффа; записи памяти —
append-only (`migrator +39`, `auditor +43`, `validator +19/−1`), канону ролей не
противоречат (F35); лента — в порядке, хронология цела (возврат → решение
владельца → `migrator` D38 → δ-аудит), факты сходятся (`D38 +7`, `D70 +8`,
migrator `+2/−7`); права (`agents-perms` ×2) чисты; δ-аудит `auditor` —
P1/P2/P3 нет.

**Дальше / риски (наблюдение, не находка):** режим «коммит прямо в `master`» —
формальное исключение из `.opencode/rules/git-workflow.md:26–28`; санкция
владельца зафиксирована только в ленте операции (запись входит в тот же коммит,
т. е. след остаётся в истории git). Отдельной записи в каноне нет — при будущем
аудите «история ↔ git-workflow» это может читаться как расхождение; решение —
за владельцем. `develop` сохраняет дубль-абзац и §10-пункт (выравнивание —
отдельным решением владельца).
