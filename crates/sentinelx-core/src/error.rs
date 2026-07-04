use thiserror::Error;

#[derive(Debug, Error)]
pub enum SentinelError {
    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("sandbox error: {0}")]
    SandboxError(String),

    #[error("threat intelligence error: {0}")]
    ThreatIntelError(String),

    #[error("storage error: {0}")]
    StorageError(String),

    #[error("authorization denied: {0}")]
    AuthorizationDenied(String),

    #[error("configuration error: {0}")]
    ConfigurationError(String),

    #[error("internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, SentinelError>;
