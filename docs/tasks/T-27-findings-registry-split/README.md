# T-27. Реестр находок: переезд в `docs/registry/` и раскол открытые/закрытые

- **Статус:** ⬜
- **Приоритет:** P2 (реестр и артефакты — до демо, если успеем; продукт не
  меняет; п. процесса журнала/политики ссылок)
- **Зависит от:** —
- **Класс:** **L** — правит канон (`AGENTS.md`, `.opencode/rules/journal.md`) и
  права (`migrator.md`) «даже одной строкой»; фич-сценариев нет
- **Источник:** [D98](../../decisions/D98-findings-registry-relocation.md)
  ([Q95](../../questions/Q95.md)); основания — [D48](../../decisions/D48-findings-registry-owner.md)
  (владелец/зона), [D65](../../decisions/D65-reference-policy.md) (политика
  ссылок)

## Контекст

Реестр находок прогонов — `docs/analysis/findings-registry.md` — вырос (94
строки, 84 находки: открытые `F26`/`F27`/`F74`–`F84` — 14; закрытые — 70) и
живёт в зоне разборов (`docs/analysis/`), держась там особым
исключением-whitelist (D48/D65). Решением владельца 03.10.2026
([D98](../../decisions/D98-findings-registry-relocation.md)) реестр переезжает в
новую зону **`docs/registry/`** и раскалывается на **открытые** (`registry.md`) и
**закрытые** (`archive.md`); оба файла — живые канон-артефакты, ссылки (в т.ч. на
архив) разрешены. Переезд+раскол — **одной операцией**. Класс — **L** (правка
канона `AGENTS.md`/`.opencode/rules/journal.md` и прав `migrator.md` «даже одной
строкой»; фич-сценариев нет).

## Что сделать

1. **Создать `docs/registry/registry.md`** — открытые находки (`F26`, `F27`,
   `F74`–`F84` — 14), и **`docs/registry/archive.md`** — закрытые (70, история).
   Перенести все 84 записи реестра с расколом, **не переписывая** формулировки
   (ID и суть сохранить; шапка — «улика, не канон», правило идемпотентности/append,
   владелец `migrator`).
2. **Обновить `tests/docs_journal.rs`:** путь `journal_files()` и whitelist
   `removable_addresses` — вместо `docs/analysis/findings-registry.md` новый
   адрес(а) зоны `docs/registry/` (снять старое исключение-whitelist).
3. **Обновить канон/ссылки:**
   [`.opencode/rules/journal.md`](../../../.opencode/rules/journal.md) §8
   (исключение-реестр — новый адрес); [`AGENTS.md`](../../../AGENTS.md) (таблица
   ролей, «Пишет в» `migrator`);
   [D48](../../decisions/D48-findings-registry-owner.md) (обратная пометка
   «уточнено D98»); [D65](../../decisions/D65-reference-policy.md) (уточнение:
   исключение-реестр переезжает, ссылки на архив разрешены);
   [`T-18`](../T-18-docs-journal-test/README.md) (whitelist теста);
   [`TRACEABILITY.md`](../../TRACEABILITY.md) (строка
   [Q53](../../questions/Q53.md)/D48 — новый адрес);
   [`.opencode/agents/migrator.md`](../../../.opencode/agents/migrator.md) — тело
   «Реестр находок» + право `edit docs/registry/**` (вместо
   `docs/analysis/findings-registry.md`).
4. **Массово заменить ссылки** `../analysis/findings-registry.md` (и
   `docs/analysis/findings-registry.md`) → `docs/registry/…` по всем файлам
   журнала/канона/документации.
5. **Полный DoD** ([D50](../../decisions/D50-dod-by-package-scope.md); прогон —
   `validator`): `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test --all`.

## Критерий готовности

- `docs_journal` **зелёный** (14/0) + полный DoD (`validator`).
- `docs/registry/registry.md` и `docs/registry/archive.md` созданы; 84 записи
  перенесены с расколом; формулировки не переписаны.
- `docs/analysis/` **не тронут** (остаётся зоной разборов прогонов).
- Ссылки живые: нет адресов на старый `docs/analysis/findings-registry.md`;
  ссылки на `docs/registry/{registry,archive}.md` разрешены и резолвятся.

## Примечания

- **Работу в операции оформления не выполнять** — карточка только фиксирует
  объём; маршрут — **L** (полный + `auditor` до коммита): `git` ветка →
  код-часть (`coder`/`tester` — `tests/docs_journal.rs`) → **канон-часть
  (сервисная сессия — `AGENTS.md`, `.opencode/**`, `migrator.md`+право)** →
  `migrator` (перенос реестра/ссылки) → `validator` (полный DoD) → `auditor`.
- **Решение** — [D98](../../decisions/D98-findings-registry-relocation.md)
  ([Q95](../../questions/Q95.md)).
- **Механика — одной операцией:** переезд и раскол не разрывать (иначе тест и
  канон на переходном состоянии).
- `docs/analysis/**` — вне правок: реестр покидает зону, но сама зона остаётся.
- Класс — **L**; маршрут/исключения фиксирует `analyst` при взятии; канон правит
  только сервисная сессия (`coder`/`docs-writer` прав на `AGENTS.md`/`.opencode/**`
  не имеют).
- Точные строки/символы — снимок на 03.10.2026; ссылки — по символам, не по
  номерам строк (антипаттерн [§8](../../../.opencode/rules/journal.md)).
