use async_trait::async_trait;
use sentinelx_core::{
    ExecutionInterceptor, ExecutionSession, FileHash, FileMetadata, InterceptResult, Result,
    SentinelError, SignatureInfo,
};
use std::path::Path;
use tracing::{info, instrument};

pub struct DefaultInterceptor {
    analyze_unknown_publishers: bool,
}

impl DefaultInterceptor {
    pub fn new(analyze_unknown_publishers: bool) -> Self {
        Self {
            analyze_unknown_publishers,
        }
    }
}

impl Default for DefaultInterceptor {
    fn default() -> Self {
        Self::new(true)
    }
}

#[async_trait]
impl ExecutionInterceptor for DefaultInterceptor {
    #[instrument(skip(self), fields(path = %path.display()))]
    async fn intercept(&self, path: &Path) -> Result<InterceptResult> {
        if !path.exists() {
            return Err(SentinelError::FileNotFound(path.display().to_string()));
        }

        let mut metadata = FileMetadata::from_path(path.to_path_buf());
        if let Ok(meta) = std::fs::metadata(path) {
            metadata.size_bytes = meta.len();
        }

        let hash = FileHash::compute(path)?;
        let signature = verify_signature(path).await?;
        let mut session = ExecutionSession::new(metadata, "system");
        session.hash = Some(hash.clone());
        session.transition(sentinelx_core::SessionStatus::Intercepted);

        let should_analyze = determine_should_analyze(&signature, self.analyze_unknown_publishers);

        info!(
            session_id = %session.id,
            sha256 = %hash.sha256,
            should_analyze,
            "execution intercepted"
        );

        Ok(InterceptResult {
            session,
            hash,
            signature,
            should_analyze,
        })
    }
}

fn determine_should_analyze(signature: &SignatureInfo, analyze_unknown: bool) -> bool {
    if signature.is_signed && signature.is_trusted && signature.certificate_chain_valid {
        return false;
    }
    analyze_unknown || !signature.is_signed
}

async fn verify_signature(_path: &Path) -> Result<SignatureInfo> {
    // Platform-specific signature verification via platform module adapters.
    Ok(SignatureInfo {
        is_signed: false,
        publisher: None,
        subject: None,
        is_trusted: false,
        is_revoked: false,
        certificate_chain_valid: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn intercepts_existing_file() {
        let mut tmp = NamedTempFile::new().unwrap();
        tmp.write_all(b"test payload").await.unwrap();
        let interceptor = DefaultInterceptor::default();
        let result = interceptor.intercept(tmp.path()).await.unwrap();
        assert!(!result.hash.sha256.is_empty());
        assert_eq!(
            result.session.status,
            sentinelx_core::SessionStatus::Intercepted
        );
    }
}
