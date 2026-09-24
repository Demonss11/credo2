# Ревью `credo2` — соответствие Gherkin и найденные баги

## 1. Что закрыто (по Gherkin)

| Функция | Статус |
|---|---|
| Версия = путь `checks/{name}/{version}/` | ✅ |
| `meta.json` с name/version/published_at/by/checksum | ✅ |
| Чтение отклоняет несоответствие пути и meta | ✅ (`list_from_ref`, `parts.len() != 3`, `meta.name != parts[1]`) |
| Строгий semver, `v.0.0.1` → ошибка | ✅ (`strip_prefix('v')` даёт 4 компонента) |
| Pre-release < release, MAJOR приоритет | ✅ |
| Иммутабельность (дубликат → ошибка) | ✅ (проверка `existing.iter().any(...)`) |
| Downgrade запрещён | ✅ |
| MAJOR bump при изменении входов | ✅ (`same_inputs` + `major == lv.major`) |
| Публикация = ветка, не main | ✅ |
| Манифест с active/supported/deprecated | ✅ |
| `manifest.json` отдельным коммитом | ✅ |
| `/version`, `/checks`, `/checks/:name/versions`, `/checks/:name/versions/:version`, `/evaluate` | ✅ |
| Deprecated помечается в meta и в ответе evaluate | ✅ |
| OpenAPI с `info.version = service_hash` | ✅ |
| API-key, `401`, `/health` без ключа | ✅ |
| Watcher не дёргает `list` при неизменном HEAD | ✅ |
| MCP: `check.deprecate`, `check.rebuild_manifest` | ✅ |

---

## 2. Критичные баги

### Баг 1. `service_hash` не меняется при deprecation

**Файл:** `service.rs`, `build_manifest`.

```rust
hash_input.push_str(&format!("{}|{}|{}\n", name, v.version, v.meta.checksum));
```

`checksum` считается из `rule + contract` (`core::checksum_of`), в нём **нет** ни `deprecated_at`, ни статуса. Когда `check.deprecate` помечает версию, `checksum` не меняется → `hash_input` побитово тот же → `service_hash` тот же. Нарушено требование:

> После deprecation манифест пересобирается автоматически → service_hash изменился.

Хуже: REST `GET /version` отдаёт старый хэш, хотя `active` уже переключился. Клиенты не увидят изменения.

**Патч:**

```rust
for v in &versions {
    let status = if v.meta.deprecated_at.is_some() { "deprecated" } else { "supported" };
    if v.meta.deprecated_at.is_some() {
        deprecated.push(v.version.clone());
    } else {
        supported.push(v.version.clone());
    }
    hash_input.push_str(&format!(
        "{}|{}|{}|{}\n",
        name, v.version, status, v.meta.checksum
    ));
}
```

Плюс тест:

```rust
#[test]
fn manifest_hash_changes_on_deprecation() {
    let mut checks = vec![/* ... */];
    let h1 = build_manifest(&checks).service_hash;
    checks[0].meta.deprecated_at = Some("2026-09-24T00:00:00Z".into());
    let h2 = build_manifest(&checks).service_hash;
    assert_ne!(h1, h2);
}
```

### Баг 2. `Semver`: `PartialEq` противоречит `Ord`

**Файл:** `semver.rs`.

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Semver { ..., pub build: Option<String> }
```

`Ord` игнорирует `build` (правильно, semver spec), а `PartialEq` — учитывает. Значит:

```
Semver::parse("1.2.3+a") == Semver::parse("1.2.3+b")   → false
Semver::parse("1.2.3+a").cmp(&parse("1.2.3+b"))       → Equal
```

Это нарушает контракт `Ord`: `a == b ⟺ a.cmp(b) == Equal`. Любая `BTreeMap<Semver, _>` или `sort + dedup` даст неожиданные результаты.

**Патч — убрать `PartialEq` из derive, реализовать вручную:**

```rust
#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct Semver { ... }

impl PartialEq for Semver {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.pre == other.pre
        // build игнорируется — семантически
    }
}
```

`Eq` можно оставить (пустой маркер), потому что рефлексивность/симметричность сохраняются.

### Баг 3. `v.0.0.1` отклоняется случайно, а не по правилу

**Файл:** `semver.rs`, `parse`.

Сейчас работает, но **по совпадению**: `strip_prefix('v')` → `".0.0.1"` → `split('.')` даёт 4 части → ошибка «нужны ровно три компонента». Если кто-то решит снять ограничение на 3 компонента (например, добавит поддержку `1.2`), `v.0.0.1` начнёт проходить как `1.2.3` — потому что пустой компонент сейчас упадёт на `parse_num`, но это неявная защита.

Явное правило лучше:

```rust
pub fn parse(input: &str) -> Result<Self> {
    let s = input.strip_prefix('v').unwrap_or(input);
    if s.is_empty() {
        bail!("пустая версия");
    }
    if s.starts_with('.') {
        bail!("версия не может начинаться с точки: {input:?}");
    }
    // ...
}
```

---

## 3. Средние замечания

### 3.1. Race condition при параллельной публикации

`publish` читает `main`, потом `update-ref refs/heads/publish/Name-X`. Если две параллельные сессии публикуют одну версию, вторая **молча перезапишет** ветку первой (`update-ref` без `--create-reflog` и без проверки old-value).

Для прод-сценария правильно использовать атомарный update с проверкой старого значения:

```rust
// update-ref refs/heads/publish/Name-1.0.0 <new> <old>
// old = "0000000000000000000000000000000000000000" если ветка не должна существовать
```

В `git` CLI это:

```rust
pub fn create_ref(repo: &Path, refname: &str, commit: &str) -> Result<()> {
    let zero = "0000000000000000000000000000000000000000";
    let out = Command::new("git").arg("-C").arg(repo)
        .args(["update-ref", refname, commit, zero])
        .output()?;
    if !out.status.success() {
        bail!("ветка {refname} уже существует");
    }
    Ok(())
}
```

`main` через `update-ref` без old-value оставить — там всегда один писатель (watcher/MCP-агент внутри процесса).

### 3.2. Pre-release лексикографически ≠ semver

`"rc10" < "rc2"` в текущем `cmp`. По spec должно быть `rc2 < rc10`. Это отмечено TODO, но **теста, фиксирующего поведение, нет** — а без него кто-то рано или поздно «починит» в неправильную сторону. Добавить:

```rust
#[test]
#[ignore = "TODO: numeric pre-release comparison per semver spec"]
fn prerelease_numeric_ordering() {
    let a = Semver::parse("1.0.0-rc2").unwrap();
    let b = Semver::parse("1.0.0-rc10").unwrap();
    assert!(a < b, "current behavior is lex, spec says numeric");
}
```

`#[ignore]` + TODO = явный долг, а не забвение.

### 3.3. Временный index не удаляется при ошибке

`write_index_with_parent` чистит temp-файл только на success-path. Если `update-index` или `write-tree` упадут — в `/tmp` накопится мусор. Мелочь, но в долгоживущем сервисе накапливается.

Правильно — guard-структура:

```rust
struct TempIndex(PathBuf);
impl Drop for TempIndex { fn drop(&mut self) { let _ = std::fs::remove_file(&self.0); } }
```

### 3.4. `mcp.rs`: `publish` строит contract один раз, потом правит version

```rust
let mut contract = contract_from_rule(&rule, version_raw);
let sem = Semver::parse(version_raw)...;
contract.version = sem.to_string();
```

Мелочь, но лучше сразу нормализовать вход:

```rust
let sem = Semver::parse(version_raw).map_err(|e| format!("невалидная версия: {e}"))?;
let contract = contract_from_rule(&rule, &sem.to_string());
```

Тогда `publish::publish(&repo, &rule, &contract, &sem.to_string(), ...)`.

### 3.5. `publish` не проверяет, что `meta.version == path`

`list_from_ref` проверяет, а `publish` — нет: он сам пишет `meta.version = vstr`, но никакой assert, что путь `checks/{name}/{vstr}` совпадает. Теоретически при рефакторинге кто-то поменяет один из format! и получит непроходимую запись. Один `debug_assert!` решает:

```rust
debug_assert_eq!(path, format!("checks/{}/{}", rule.name, vstr));
```

---

## 4. Что не закрыто из Gherkin

| Требование | Что нужно |
|---|---|
| CI: `cargo fmt --check`, `clippy -D warnings`, `cargo test` | `.github/workflows/ci.yml` — 15 строк |
| Интеграционные тесты publish | `tests/publish.rs` на temp bare repo: `rejects_duplicate_version`, `rejects_downgrade`, `rejects_contract_change_without_major`, `manifest_hash_is_deterministic`, `meta_version_matches_path` |
| Тест на deprecation → service_hash | см. § 2.1 |
| Тест на `rc2 < rc10` (зафиксировать текущее) | см. § 3.2 |
| Явный запрет `v.` в парсере | см. § 2.3 |

### 4.1. `tests/publish.rs` — скелет

```rust
use credo2::core::{contract_from_rule, Rule, Value};
use credo2::{publish, service};
use std::process::Command;
use tempfile::TempDir;

fn rule(name: &str, field: &str) -> Rule {
    Rule {
        name: name.into(),
        condition: credo2::core::Condition {
            field: field.into(), op: "<".into(), value: Value::Number(21.0),
        },
        action: credo2::core::Action {
            decision: "Отказ".into(), reason: "test".into(),
        },
    }
}

fn init_repo() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let repo = dir.path().join("repo.git");
    publish::ensure_repo(&repo).unwrap();
    (dir, repo)
}

#[test]
fn rejects_duplicate_version() {
    let (_d, repo) = init_repo();
    let r = rule("A", "x");
    let c = contract_from_rule(&r, "1.0.0");
    publish::publish(&repo, &r, &c, "1.0.0", "test").unwrap();
    // merge ветки в main руками через update-ref — упрощённо:
    let commit = publish::rev_parse(&repo, "refs/heads/publish/A-1.0.0").unwrap();
    publish::update_ref(&repo, "refs/heads/main", &commit).unwrap();

    let err = publish::publish(&repo, &r, &c, "1.0.0", "test").unwrap_err();
    assert!(err.to_string().contains("уже существует"));
}

#[test]
fn rejects_downgrade() {
    let (_d, repo) = init_repo();
    let r = rule("A", "x");
    let c1 = contract_from_rule(&r, "1.0.0");
    publish::publish(&repo, &r, &c1, "1.0.0", "test").unwrap();
    let commit = publish::rev_parse(&repo, "refs/heads/publish/A-1.0.0").unwrap();
    publish::update_ref(&repo, "refs/heads/main", &commit).unwrap();

    let c2 = contract_from_rule(&r, "0.9.0");
    let err = publish::publish(&repo, &r, &c2, "0.9.0", "test").unwrap_err();
    assert!(err.to_string().contains("downgrade"));
}

#[test]
fn rejects_contract_change_without_major() {
    let (_d, repo) = init_repo();
    let r1 = rule("A", "x");
    let c1 = contract_from_rule(&r1, "1.0.0");
    publish::publish(&repo, &r1, &c1, "1.0.0", "test").unwrap();
    let commit = publish::rev_parse(&repo, "refs/heads/publish/A-1.0.0").unwrap();
    publish::update_ref(&repo, "refs/heads/main", &commit).unwrap();

    let r2 = rule("A", "y"); // другой вход
    let c2 = contract_from_rule(&r2, "1.1.0");
    let err = publish::publish(&repo, &r2, &c2, "1.1.0", "test").unwrap_err();
    assert!(err.to_string().contains("MAJOR"));
}
```

(понадобится `tempfile` в `[dev-dependencies]`.)

### 4.2. CI — `.github/workflows/ci.yml`

```yaml
name: ci
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { components: rustfmt, clippy }
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test --all
```
