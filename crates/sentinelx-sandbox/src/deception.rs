use async_trait::async_trait;
use sentinelx_core::{
    DeceptionEnvironment, DeceptionHit, Result, UserSimulator,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

const FAKE_ARTIFACTS: &[(&str, &str)] = &[
    ("/fake/home/Documents/passwords.txt", "credentials"),
    ("/fake/home/.ssh/id_rsa", "ssh_key"),
    ("/fake/home/.aws/credentials", "cloud_token"),
    ("/fake/home/wallet.dat", "crypto_wallet"),
    ("/fake/home/project/.env", "api_key"),
];

pub struct DefaultDeceptionEnvironment {
    hits: Arc<RwLock<HashMap<String, Vec<DeceptionHit>>>>,
}

impl Default for DefaultDeceptionEnvironment {
    fn default() -> Self {
        Self {
            hits: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DeceptionEnvironment for DefaultDeceptionEnvironment {
    async fn populate(&self, instance_id: &str) -> Result<()> {
        tracing::info!(instance_id, artifacts = FAKE_ARTIFACTS.len(), "deception environment populated");
        Ok(())
    }

    async fn collect_hits(&self, instance_id: &str) -> Result<Vec<DeceptionHit>> {
        let hits = self.hits.read().await;
        Ok(hits.get(instance_id).cloned().unwrap_or_default())
    }
}

pub fn record_deception_access(
    hits: &Arc<RwLock<HashMap<String, Vec<DeceptionHit>>>>,
    instance_id: &str,
    path: &str,
    artifact_type: &str,
) {
    let hit = DeceptionHit {
        artifact_path: path.into(),
        artifact_type: artifact_type.into(),
        access_type: "read".into(),
        timestamp: chrono::Utc::now(),
    };
    tokio::spawn({
        let hits = hits.clone();
        let instance_id = instance_id.to_string();
        async move {
            hits.write().await.entry(instance_id).or_default().push(hit);
        }
    });
}

pub fn fake_artifact_paths() -> Vec<PathBuf> {
    FAKE_ARTIFACTS.iter().map(|(p, _)| PathBuf::from(p)).collect()
}
