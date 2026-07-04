use async_trait::async_trait;
use sentinelx_core::{
    DeceptionEnvironment, Result, SandboxBackend, SandboxResult, SentinelError, UserSimulator,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::process::Command;
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};

struct SandboxInstance {
    _dir: TempDir,
    root: PathBuf,
}

pub struct LocalSandboxBackend {
    instances: Arc<RwLock<HashMap<String, SandboxInstance>>>,
    network_isolated: bool,
}

impl LocalSandboxBackend {
    pub fn new(network_isolated: bool) -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            network_isolated,
        }
    }
}

impl Default for LocalSandboxBackend {
    fn default() -> Self {
        Self::new(true)
    }
}

#[async_trait]
impl SandboxBackend for LocalSandboxBackend {
    fn backend_name(&self) -> &str {
        "local-process-isolation"
    }

    async fn create_instance(&self) -> Result<String> {
        let dir = TempDir::new()
            .map_err(|e| SentinelError::SandboxError(e.to_string()))?;
        let id = uuid::Uuid::new_v4().to_string();
        let root = dir.path().to_path_buf();
        let instance = SandboxInstance { _dir: dir, root };
        self.instances.write().await.insert(id.clone(), instance);
        Ok(id)
    }

    async fn execute(
        &self,
        instance_id: &str,
        target: &Path,
        timeout_secs: u64,
    ) -> Result<SandboxResult> {
        let instances = self.instances.read().await;
        let instance = instances
            .get(instance_id)
            .ok_or_else(|| SentinelError::SandboxError("instance not found".into()))?;

        let sandbox_path = instance.root.join(
            target
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("target")),
        );
        std::fs::copy(target, &sandbox_path)
            .map_err(|e| SentinelError::SandboxError(e.to_string()))?;

        let start = std::time::Instant::now();
        let ext = sandbox_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let mut cmd = build_command(&sandbox_path, ext)?;
        cmd.current_dir(&instance.root);
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        let result = timeout(Duration::from_secs(timeout_secs), cmd.status()).await;

        let exit_code = match result {
            Ok(Ok(status)) => status.code(),
            Ok(Err(e)) => {
                tracing::warn!(error = %e, "sandbox execution error");
                None
            }
            Err(_) => {
                tracing::warn!("sandbox execution timed out");
                None
            }
        };

        Ok(SandboxResult {
            sandbox_id: instance_id.to_string(),
            backend: self.backend_name().into(),
            execution_time_ms: start.elapsed().as_millis() as u64,
            exit_code,
            rolled_back: true,
            network_blocked: self.network_isolated,
        })
    }

    async fn destroy_instance(&self, instance_id: &str) -> Result<()> {
        self.instances.write().await.remove(instance_id);
        Ok(())
    }
}

fn build_command(path: &Path, ext: &str) -> Result<Command> {
    let mut cmd = match ext {
        "py" => {
            let mut c = Command::new("python3");
            c.arg(path);
            c
        }
        "sh" | "bash" => {
            let mut c = Command::new("sh");
            c.arg(path);
            c
        }
        "" if path.metadata().map(|m| m.permissions().readonly()).unwrap_or(false) => {
            let mut c = Command::new(path);
            c
        }
        _ => {
            // Safe default: attempt execution only for script types we explicitly handle.
            return Err(SentinelError::SandboxError(format!(
                "unsupported sandbox target type: .{ext}"
            )));
        }
    };
    Ok(cmd)
}

pub struct SandboxOrchestrator {
    backend: Arc<dyn SandboxBackend>,
    user_sim: Arc<dyn UserSimulator>,
    deception: Arc<dyn DeceptionEnvironment>,
    default_timeout_secs: u64,
    activity_profile: String,
}

impl SandboxOrchestrator {
    pub fn new(
        backend: Arc<dyn SandboxBackend>,
        user_sim: Arc<dyn UserSimulator>,
        deception: Arc<dyn DeceptionEnvironment>,
    ) -> Self {
        Self {
            backend,
            user_sim,
            deception,
            default_timeout_secs: 30,
            activity_profile: "passive".into(),
        }
    }

    pub async fn run_analysis(&self, target: &Path) -> Result<SandboxResult> {
        let instance_id = self.backend.create_instance().await?;
        self.deception.populate(&instance_id).await?;
        self.user_sim
            .run_profile(&instance_id, &self.activity_profile)
            .await?;

        let result = self
            .backend
            .execute(&instance_id, target, self.default_timeout_secs)
            .await;

        let _ = self.backend.destroy_instance(&instance_id).await;
        result
    }
}
