use crate::core::{CheckContract, CheckMeta, Rule, StoredCheck, checksum_of};
use crate::semver::Semver;
use anyhow::{Context, Result, anyhow, bail};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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

/// Читает parent tree, применяет изменения (path, blob), пишет новое дерево.
pub fn write_index_with_parent(
    repo: &Path,
    parent_ref: &str,
    changes: &[(String, Option<String>)], // None = удалить
) -> Result<String> {
    let idx = temp_index();
    let _ = std::fs::remove_file(&idx);
    if rev_parse(repo, parent_ref).is_ok() {
        git_with_index(repo, &idx, &["read-tree", parent_ref])?;
    } else {
        git_with_index(repo, &idx, &["read-tree", "--empty"])?;
    }
    for (path, sha) in changes {
        match sha {
            Some(s) => {
                git_with_index(
                    repo,
                    &idx,
                    &[
                        "update-index",
                        "--add",
                        "--cacheinfo",
                        &format!("100644,{s},{path}"),
                    ],
                )?;
            }
            None => {
                let _ = git_with_index(repo, &idx, &["update-index", "--force-remove", path]);
            }
        }
    }
    let tree = git_with_index(repo, &idx, &["write-tree"])?
        .trim()
        .to_string();
    let _ = std::fs::remove_file(&idx);
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
    if repo.join("HEAD").exists() {
        return Ok(());
    } // маркер bare-репо
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
    let vstr = version.to_string();
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
    update_ref(repo, &format!("refs/heads/{branch}"), &commit)?;

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
    let path = format!("checks/{name}/{}/meta.json", v);

    let mut meta: CheckMeta = serde_json::from_slice(
        &show_file(repo, "main", &path).with_context(|| format!("нет {name}@{v} в main"))?,
    )?;
    if meta.deprecated_at.is_some() {
        bail!("{name}@{v} уже помечена deprecated");
    }
    meta.deprecated_at = Some(chrono::Utc::now().to_rfc3339());
    meta.deprecation_reason = Some(reason.into());

    let sha = hash_blob(repo, &serde_json::to_vec_pretty(&meta)?)?;
    let tree = write_index_with_parent(repo, "main", &[(path, Some(sha))])?;
    let parent = rev_parse(repo, "main")?;
    let commit = commit_tree(repo, &tree, &parent, &format!("deprecate {name}@{v}"))?;
    update_ref(repo, "refs/heads/main", &commit)?;
    Ok(())
}

/// Записывает manifest.json в main. Возвращает Some(commit), если был создан.
pub fn write_manifest_to_main(
    repo: &Path,
    manifest: &crate::service::Manifest,
) -> Result<Option<String>> {
    ensure_repo(repo)?;
    let bytes = serde_json::to_vec_pretty(manifest)?;
    let new_sha = hash_blob(repo, &bytes)?;

    // Если в main уже идентичный blob — коммит не создаём
    if let Ok(existing) = show_file(repo, "main", "manifest.json") {
        let existing_sha = hash_blob(repo, &existing)?;
        if existing_sha == new_sha {
            return Ok(None);
        }
    }

    let tree = write_index_with_parent(repo, "main", &[("manifest.json".into(), Some(new_sha))])?;
    let parent = rev_parse(repo, "main")?;
    let msg = format!("manifest: {}", manifest.service_hash);
    let commit = commit_tree(repo, &tree, &parent, &msg)?;
    update_ref(repo, "refs/heads/main", &commit)?;
    Ok(Some(commit))
}
