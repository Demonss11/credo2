use crate::core::{CheckContract, Rule, contract_from_rule};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Draft {
    pub name: String,
    pub rule: Rule,
    pub contract: CheckContract,
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
            contract: contract_from_rule(&rule, "0.0.0"),
            rule,
            created_at: existing
                .map(|d| d.created_at.clone())
                .unwrap_or_else(|| now.clone()),
            updated_at: now,
        }
    }
}
