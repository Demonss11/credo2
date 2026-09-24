use crate::core::StoredCheck;
use crate::publish;
use crate::semver::Semver;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

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
            if v.meta.deprecated_at.is_some() {
                deprecated.push(v.version.clone());
            } else {
                supported.push(v.version.clone());
            }
            hash_input.push_str(&format!("{}|{}|{}\n", name, v.version, v.meta.checksum));
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
        publish::ensure_repo(&repo)?;
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
        let head =
            match tokio::task::spawn_blocking(move || publish::rev_parse(&repo, "main")).await? {
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
    publish::ensure_repo(repo)?;
    let head = match publish::rev_parse(repo, "main") {
        Ok(h) => h,
        Err(_) => return Ok((String::new(), Vec::new())),
    };
    let checks = publish::list_from_ref(repo, "main")?;
    Ok((head, checks))
}
