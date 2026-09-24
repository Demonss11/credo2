pub mod core;
pub mod mcp;
pub mod rest;

use crate::core::{CheckContract, CheckMeta, Rule, Semver, StoredCheck, checksum_of};
use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

// ---------- низкоуровневые git-операции (bare repo, без рабочего дерева) ----------

fn git_run(repo: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .with_context(|| format!("git {args:?}"))?;
    if !out.status.success() {
        bail!("git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn git_with_index(repo: &Path, idx: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .env("GIT_INDEX_FILE", idx)
        .args(args)
        .output()?;
    if !out.status.success() {
        bail!("git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn hash_blob(repo: &Path, content: &[u8]) -> Result<String> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["hash-object", "-w", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin.as_mut().unwrap().write_all(content)?;
    let out = child.wait_with_output()?;
    if !out.status.success() {
        bail!("hash-object failed");
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}

fn temp_index() -> PathBuf {
    std::env::temp_dir().join(format!("credo-idx-{}", uuid::Uuid::new_v4()))
}

/// Временный git-индекс, который удаляется из `/tmp` даже при ошибке.
struct TempIndex(PathBuf);

impl TempIndex {
    fn new() -> Self {
        let p = temp_index();
        let _ = std::fs::remove_file(&p);
        Self(p)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempIndex {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Читает parent tree, применяет изменения (path, blob), пишет новое дерево.
pub fn write_index_with_parent(
    repo: &Path,
    parent_ref: &str,
    changes: &[(String, Option<String>)], // None = удалить
) -> Result<String> {
    let idx = TempIndex::new();
    if rev_parse(repo, parent_ref).is_ok() {
        git_with_index(repo, idx.path(), &["read-tree", parent_ref])?;
    } else {
        git_with_index(repo, idx.path(), &["read-tree", "--empty"])?;
    }
    for (path, sha) in changes {
        match sha {
            Some(s) => {
                git_with_index(
                    repo,
                    idx.path(),
                    &[
                        "update-index",
                        "--add",
                        "--cacheinfo",
                        &format!("100644,{s},{path}"),
                    ],
                )?;
            }
            None => {
                let _ = git_with_index(repo, idx.path(), &["update-index", "--force-remove", path]);
            }
        }
    }
    let tree = git_with_index(repo, idx.path(), &["write-tree"])?
        .trim()
        .to_string();
    Ok(tree)
}

pub fn rev_parse(repo: &Path, refname: &str) -> Result<String> {
    Ok(git_run(repo, &["rev-parse", refname])?.trim().to_string())
}

pub fn commit_tree(repo: &Path, tree: &str, parent: &str, msg: &str) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["commit-tree", tree, "-p", parent, "-m", msg])
        .output()?;
    if !out.status.success() {
        bail!("commit-tree: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}

fn commit_tree_no_parent(repo: &Path, tree: &str, msg: &str) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["commit-tree", tree, "-m", msg])
        .output()?;
    if !out.status.success() {
        bail!("commit-tree: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}

pub fn update_ref(repo: &Path, refname: &str, commit: &str) -> Result<()> {
    git_run(repo, &["update-ref", refname, commit])?;
    Ok(())
}

/// Атомарно создаёт ref, падая, если он уже существует.
/// `old = zero oid` означает «ветки быть не должно» — защищает от гонки
/// при параллельной публикации одной версии.
pub fn create_ref(repo: &Path, refname: &str, commit: &str) -> Result<()> {
    let zero = "0000000000000000000000000000000000000000";
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["update-ref", refname, commit, zero])
        .output()?;
    if !out.status.success() {
        bail!("ветка {refname} уже существует");
    }
    Ok(())
}

fn show_file(repo: &Path, refname: &str, path: &str) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["cat-file", "-p", &format!("{refname}:{path}")])
        .output()?;
    if !out.status.success() {
        bail!("нет файла {refname}:{path}");
    }
    Ok(out.stdout)
}

fn list_tree(repo: &Path, refname: &str) -> Result<Vec<String>> {
    let out = git_run(repo, &["ls-tree", "-r", "--name-only", refname])?;
    Ok(out.lines().map(String::from).collect())
}

// ---------- репозиторий ----------

pub fn ensure_repo(repo: &Path) -> Result<()> {
    // Маркер bare-репо: есть objects/ и refs/, но нет рабочего .git.
    let is_bare =
        repo.join("objects").is_dir() && repo.join("refs").is_dir() && !repo.join(".git").exists();
    if is_bare {
        return Ok(());
    }
    if repo.join(".git").exists() {
        bail!(
            "{} — рабочий репозиторий, ожидается bare (без .git)",
            repo.display()
        );
    }
    std::fs::create_dir_all(repo)?;
    git_run(repo, &["init", "--bare", "--initial-branch=main"])?;

    // Пустой начальный коммит в main
    let empty_tree = "4b825dc642cb6eb9a060e54bf8d69288fbee4904"; // git empty tree
    let commit = commit_tree_no_parent(repo, empty_tree, "init")?;
    update_ref(repo, "refs/heads/main", &commit)?;
    Ok(())
}

// ---------- чтение ----------

pub fn list_from_ref(repo: &Path, refname: &str) -> Result<Vec<StoredCheck>> {
    ensure_repo(repo)?;
    if rev_parse(repo, refname).is_err() {
        return Ok(vec![]);
    }

    let mut out = Vec::new();
    for path in list_tree(repo, refname)? {
        if !path.starts_with("checks/") || !path.ends_with("/meta.json") {
            continue;
        }
        let dir = &path[..path.len() - "/meta.json".len()];
        let parts: Vec<&str> = dir.split('/').collect();
        if parts.len() != 3 {
            continue;
        }

        let meta_bytes = match show_file(repo, refname, &path) {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!("skip {dir}: {e}");
                continue;
            }
        };
        let meta: CheckMeta = match serde_json::from_slice(&meta_bytes) {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!("skip {dir}: bad meta: {e}");
                continue;
            }
        };

        if meta.name != parts[1] || meta.version != parts[2] {
            tracing::warn!(
                "skip {dir}: meta не совпадает с путём (name={}, version={})",
                meta.name,
                meta.version
            );
            continue;
        }

        let rule: Rule =
            serde_json::from_slice(&show_file(repo, refname, &format!("{dir}/rule.json"))?)?;
        let contract: CheckContract =
            serde_json::from_slice(&show_file(repo, refname, &format!("{dir}/contract.json"))?)?;

        out.push(StoredCheck {
            name: meta.name.clone(),
            version: meta.version.clone(),
            rule,
            contract,
            meta,
        });
    }
    Ok(out)
}

pub fn latest_semver_for<'a>(checks: &'a [StoredCheck], name: &str) -> Option<&'a StoredCheck> {
    checks.iter().filter(|c| c.name == name).max_by(|a, b| {
        let av = Semver::parse(&a.version).ok();
        let bv = Semver::parse(&b.version).ok();
        match (av, bv) {
            (Some(x), Some(y)) => x.cmp(&y),
            _ => a.version.cmp(&b.version),
        }
    })
}

// ---------- публикация ----------

#[derive(Debug)]
pub struct PublishOutcome {
    pub name: String,
    pub version: String, // нормализованная
    pub branch: String,
    pub path: String, // checks/Name/1.0.0
    pub commit_msg: String,
}

pub fn publish(
    repo: &Path,
    rule: &Rule,
    contract: &CheckContract,
    version_raw: &str,
    published_by: &str,
) -> Result<PublishOutcome> {
    ensure_repo(repo)?;
    let version = Semver::parse(version_raw)
        .map_err(|e| anyhow!("невалидная версия {version_raw:?}: {e}"))?;
    // Версия для хранилища: без `+build` (build не влияет на идентичность).
    let vstr = version.as_storage();

    // Контракт — производное от версии; не доверяем вызывающему.
    let contract = {
        let mut c = contract.clone();
        c.version = vstr.clone();
        c
    };
    let contract = &contract;

    let path = format!("checks/{}/{}", rule.name, vstr);

    let existing = list_from_ref(repo, "main")?;

    if existing
        .iter()
        .any(|c| c.name == rule.name && c.version == vstr)
    {
        bail!("версия {vstr} уже существует для {}", rule.name);
    }

    if let Some(latest) = latest_semver_for(&existing, &rule.name) {
        let lv = Semver::parse(&latest.version)?;
        if version <= lv {
            bail!(
                "версия {vstr} не больше максимальной {} — downgrade запрещён",
                latest.version
            );
        }
        if !crate::core::same_inputs(&latest.contract, contract) && version.major == lv.major {
            bail!(
                "контракт изменился (входы), но MAJOR не поднят ({} → {}); \
                 поднимите MAJOR",
                latest.version,
                vstr
            );
        }
    }

    let meta = CheckMeta {
        name: rule.name.clone(),
        version: vstr.clone(),
        published_at: chrono::Utc::now().to_rfc3339(),
        published_by: published_by.into(),
        checksum: checksum_of(rule, contract)?,
        deprecated_at: None,
        deprecation_reason: None,
    };
    // Путь должен совпадать с полями meta — иначе запись станет непроходимой.
    debug_assert_eq!(path, format!("checks/{}/{}", meta.name, meta.version));

    let rule_sha = hash_blob(repo, &serde_json::to_vec_pretty(rule)?)?;
    let contract_sha = hash_blob(repo, &serde_json::to_vec_pretty(contract)?)?;
    let meta_sha = hash_blob(repo, &serde_json::to_vec_pretty(&meta)?)?;

    let changes = vec![
        (format!("{path}/rule.json"), Some(rule_sha)),
        (format!("{path}/contract.json"), Some(contract_sha)),
        (format!("{path}/meta.json"), Some(meta_sha)),
    ];
    let tree = write_index_with_parent(repo, "main", &changes)?;

    let parent = rev_parse(repo, "main")?;
    let branch = format!("publish/{}-{}", rule.name, vstr);
    let commit_msg = format!("publish {}@{}", rule.name, vstr);
    let commit = commit_tree(repo, &tree, &parent, &commit_msg)?;
    create_ref(repo, &format!("refs/heads/{branch}"), &commit)?;

    Ok(PublishOutcome {
        name: rule.name.clone(),
        version: vstr,
        branch,
        path,
        commit_msg,
    })
}

/// Помечает версию deprecated. Пишет в main напрямую.
pub fn deprecate(repo: &Path, name: &str, version: &str, reason: &str) -> Result<()> {
    ensure_repo(repo)?;
    let v = Semver::parse(version)?;
    let vstr = v.as_storage();
    let path = format!("checks/{name}/{vstr}/meta.json");

    let mut meta: CheckMeta = serde_json::from_slice(
        &show_file(repo, "main", &path).with_context(|| format!("нет {name}@{vstr} в main"))?,
    )?;
    if meta.deprecated_at.is_some() {
        bail!("{name}@{vstr} уже помечена deprecated");
    }
    meta.deprecated_at = Some(chrono::Utc::now().to_rfc3339());
    meta.deprecation_reason = Some(reason.into());

    let sha = hash_blob(repo, &serde_json::to_vec_pretty(&meta)?)?;
    let tree = write_index_with_parent(repo, "main", &[(path, Some(sha))])?;
    let parent = rev_parse(repo, "main")?;
    let commit = commit_tree(repo, &tree, &parent, &format!("deprecate {name}@{vstr}"))?;
    update_ref(repo, "refs/heads/main", &commit)?;
    Ok(())
}

/// Записывает manifest.json в main. Возвращает Some(commit), если был создан.
pub fn write_manifest_to_main(repo: &Path, manifest: &Manifest) -> Result<Option<String>> {
    ensure_repo(repo)?;
    let bytes = serde_json::to_vec_pretty(manifest)?;
    let new_sha = hash_blob(repo, &bytes)?;

    // Если в main уже идентичный blob — коммит не создаём.
    // rev-parse даёт SHA blob напрямую, без лишней записи объекта в БД.
    if let Ok(existing_sha) = rev_parse(repo, "main:manifest.json")
        && existing_sha == new_sha
    {
        return Ok(None);
    }

    let tree = write_index_with_parent(repo, "main", &[("manifest.json".into(), Some(new_sha))])?;
    let parent = rev_parse(repo, "main")?;
    let msg = format!("manifest: {}", manifest.service_hash);
    let commit = commit_tree(repo, &tree, &parent, &msg)?;
    update_ref(repo, "refs/heads/main", &commit)?;
    Ok(Some(commit))
}

// ---------- сервис и кэш манифеста ----------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub name: String,
    pub active: String,
    pub supported: Vec<String>,
    pub deprecated: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: u32,
    pub generated_at: String,
    pub service_hash: String,
    pub checks: Vec<ManifestEntry>,
}

pub fn build_manifest(checks: &[StoredCheck]) -> Manifest {
    let mut by_name: BTreeMap<String, Vec<&StoredCheck>> = BTreeMap::new();
    for c in checks {
        by_name.entry(c.name.clone()).or_default().push(c);
    }

    let mut entries = Vec::new();
    let mut hash_input = String::new();

    for (name, mut versions) in by_name {
        versions.sort_by(|a, b| {
            let av = Semver::parse(&a.version).ok();
            let bv = Semver::parse(&b.version).ok();
            match (av, bv) {
                (Some(x), Some(y)) => x.cmp(&y),
                _ => a.version.cmp(&b.version),
            }
        });

        let mut supported = Vec::new();
        let mut deprecated = Vec::new();
        for v in &versions {
            let status = if v.meta.deprecated_at.is_some() {
                "deprecated"
            } else {
                "supported"
            };
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

        let active = supported
            .last()
            .or_else(|| deprecated.last())
            .cloned()
            .unwrap_or_default();

        entries.push(ManifestEntry {
            name,
            active,
            supported,
            deprecated,
        });
    }

    let service_hash = {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(hash_input.as_bytes());
        hex::encode(h.finalize())
    };

    Manifest {
        schema_version: 1,
        generated_at: chrono::Utc::now().to_rfc3339(),
        service_hash,
        checks: entries,
    }
}

pub struct CachedState {
    pub head: String,
    pub manifest: Manifest,
    pub checks: Vec<StoredCheck>,
}

pub struct ServiceCache {
    repo: PathBuf,
    state: RwLock<CachedState>,
}

impl ServiceCache {
    pub fn load(repo: PathBuf) -> Result<Arc<Self>> {
        ensure_repo(&repo)?;
        let (head, checks) = read_state(&repo)?;
        let manifest = build_manifest(&checks);
        Ok(Arc::new(Self {
            repo,
            state: RwLock::new(CachedState {
                head,
                manifest,
                checks,
            }),
        }))
    }

    pub async fn reload(&self) -> Result<()> {
        let repo = self.repo.clone();
        let (head, checks) = tokio::task::spawn_blocking(move || read_state(&repo)).await??;
        let manifest = build_manifest(&checks);
        let mut st = self.state.write().await;
        st.head = head;
        st.manifest = manifest;
        st.checks = checks;
        Ok(())
    }

    pub async fn manifest(&self) -> Manifest {
        self.state.read().await.manifest.clone()
    }

    pub async fn checks(&self) -> Vec<StoredCheck> {
        self.state.read().await.checks.clone()
    }

    pub async fn get(&self, name: &str, version: &str) -> Option<StoredCheck> {
        self.state
            .read()
            .await
            .checks
            .iter()
            .find(|c| c.name == name && c.version == version)
            .cloned()
    }

    pub async fn active_version(&self, name: &str) -> Option<String> {
        self.state
            .read()
            .await
            .manifest
            .checks
            .iter()
            .find(|e| e.name == name)
            .map(|e| e.active.clone())
    }

    pub async fn versions(&self, name: &str) -> Option<ManifestEntry> {
        self.state
            .read()
            .await
            .manifest
            .checks
            .iter()
            .find(|e| e.name == name)
            .cloned()
    }

    /// Один тик watcher: дёргает git rev-parse main. Если HEAD тот же — list не вызывается.
    async fn tick(&self) -> Result<()> {
        let repo = self.repo.clone();
        let head = match tokio::task::spawn_blocking(move || rev_parse(&repo, "main")).await? {
            Ok(h) => h,
            Err(_) => return Ok(()),
        };
        let same = { self.state.read().await.head == head };
        if same {
            return Ok(());
        }
        self.reload().await
    }

    pub fn start_watcher(self: &Arc<Self>, period: Duration) {
        let cache = self.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(period);
            tick.tick().await;
            loop {
                tick.tick().await;
                if let Err(e) = cache.tick().await {
                    tracing::warn!("watcher: {e}");
                }
            }
        });
    }
}

fn read_state(repo: &Path) -> Result<(String, Vec<StoredCheck>)> {
    ensure_repo(repo)?;
    let head = match rev_parse(repo, "main") {
        Ok(h) => h,
        Err(_) => return Ok((String::new(), Vec::new())),
    };
    let checks = list_from_ref(repo, "main")?;
    Ok((head, checks))
}

// ---------- песочница и состояние приложения ----------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Draft {
    pub name: String,
    pub rule: Rule,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Sandbox {
    pub drafts: HashMap<String, Draft>,
}

impl Sandbox {
    fn load(p: &Path) -> Self {
        std::fs::read(p)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }
    fn save(&self, p: &Path) -> Result<()> {
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(p, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }
}

pub struct AppState {
    _workspace: PathBuf,
    sandbox_path: PathBuf,
    published_repo: PathBuf,
    sandbox: RwLock<Sandbox>,
    api_key: Option<String>,
}

impl AppState {
    pub fn new(workspace: PathBuf, api_key: Option<String>) -> Result<Self> {
        let credo_dir = workspace.join(".credo");
        std::fs::create_dir_all(&credo_dir)?;
        let sandbox_path = credo_dir.join("sandbox.json");
        let published_repo = credo_dir.join("published-repo");
        let sandbox = Sandbox::load(&sandbox_path);
        Ok(Self {
            _workspace: workspace,
            sandbox_path,
            published_repo,
            sandbox: RwLock::new(sandbox),
            api_key,
        })
    }

    pub fn sandbox_path(&self) -> &Path {
        &self.sandbox_path
    }
    pub fn published_repo(&self) -> &Path {
        &self.published_repo
    }
    pub fn api_key(&self) -> Option<&String> {
        self.api_key.as_ref()
    }

    pub async fn upsert_draft(&self, d: Draft) -> Result<()> {
        {
            self.sandbox.write().await.drafts.insert(d.name.clone(), d);
        }
        self.persist().await
    }
    pub async fn get_draft(&self, name: &str) -> Option<Draft> {
        self.sandbox.read().await.drafts.get(name).cloned()
    }
    pub async fn list_drafts(&self) -> Vec<Draft> {
        let mut v: Vec<_> = self.sandbox.read().await.drafts.values().cloned().collect();
        v.sort_by(|a, b| a.name.cmp(&b.name));
        v
    }
    pub async fn delete_draft(&self, name: &str) -> Result<bool> {
        let removed = self.sandbox.write().await.drafts.remove(name).is_some();
        if removed {
            self.persist().await?;
        }
        Ok(removed)
    }
    async fn persist(&self) -> Result<()> {
        self.sandbox.read().await.save(&self.sandbox_path)
    }

    pub fn make_draft(&self, rule: Rule, existing: Option<&Draft>) -> Draft {
        let now = chrono::Utc::now().to_rfc3339();
        Draft {
            name: rule.name.clone(),
            rule,
            created_at: existing
                .map(|d| d.created_at.clone())
                .unwrap_or_else(|| now.clone()),
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Action, Condition, Value};

    fn stored(deprecated_at: Option<String>) -> StoredCheck {
        let rule = Rule {
            name: "A".into(),
            condition: Condition {
                field: "x".into(),
                op: "<".into(),
                value: Value::Number(21.0),
            },
            action: Action {
                decision: "Отказ".into(),
                reason: "test".into(),
            },
        };
        let contract = crate::core::contract_from_rule(&rule, "1.0.0");
        StoredCheck {
            name: "A".into(),
            version: "1.0.0".into(),
            rule,
            contract,
            meta: CheckMeta {
                name: "A".into(),
                version: "1.0.0".into(),
                published_at: "2026-09-24T00:00:00Z".into(),
                published_by: "test".into(),
                checksum: "sha256:x".into(),
                deprecated_at,
                deprecation_reason: None,
            },
        }
    }

    #[test]
    fn manifest_hash_changes_on_deprecation() {
        let h1 = build_manifest(&[stored(None)]).service_hash;
        let h2 = build_manifest(&[stored(Some("2026-09-24T00:00:00Z".into()))]).service_hash;
        assert_ne!(h1, h2);
    }
}
