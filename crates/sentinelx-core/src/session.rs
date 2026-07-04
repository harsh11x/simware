use crate::{FileHash, FileMetadata};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Pending,
    Intercepted,
    StaticAnalysis,
    ThreatIntel,
    RiskPrediction,
    Sandboxing,
    BehaviorMonitoring,
    Correlating,
    Scoring,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSession {
    pub id: Uuid,
    pub file: FileMetadata,
    pub hash: Option<FileHash>,
    pub status: SessionStatus,
    pub initiated_by: String,
    pub platform: Platform,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

impl ExecutionSession {
    pub fn new(file: FileMetadata, initiated_by: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            file,
            hash: None,
            status: SessionStatus::Pending,
            initiated_by: initiated_by.into(),
            platform: detect_platform(),
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    pub fn transition(&mut self, status: SessionStatus) {
        self.status = status;
        self.updated_at = Utc::now();
        if status == SessionStatus::Completed || status == SessionStatus::Failed {
            self.completed_at = Some(Utc::now());
        }
    }
}

pub fn detect_platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::MacOS
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else {
        Platform::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInfo {
    pub is_signed: bool,
    pub publisher: Option<String>,
    pub subject: Option<String>,
    pub is_trusted: bool,
    pub is_revoked: bool,
    pub certificate_chain_valid: bool,
}
