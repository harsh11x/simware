use async_trait::async_trait;
use sentinelx_core::{
    PipelineContext, Result, SentinelError, StaticAnalysisResult, StaticAnalyzer,
};
use std::fs;
use std::path::Path;
use tracing::instrument;

use crate::entropy::{is_likely_packed, shannon_entropy};
use crate::strings::{extract_printable_strings, extract_suspicious_apis, extract_urls};
use crate::yara::{map_to_mitre, YaraScanner};

pub struct DefaultStaticAnalyzer {
    yara: YaraScanner,
    max_bytes: usize,
}

impl DefaultStaticAnalyzer {
    pub fn new() -> Self {
        Self {
            yara: YaraScanner::new(Some("rules/".into())),
            max_bytes: 50 * 1024 * 1024,
        }
    }
}

impl Default for DefaultStaticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StaticAnalyzer for DefaultStaticAnalyzer {
    #[instrument(skip(self, ctx), fields(session_id = %ctx.session_id()))]
    async fn analyze(&self, ctx: &PipelineContext) -> Result<StaticAnalysisResult> {
        let path = &ctx.session.file.path;
        analyze_file(path, self.max_bytes, &self.yara, ctx).await
    }
}

async fn analyze_file(
    path: &Path,
    max_bytes: usize,
    yara: &YaraScanner,
    _ctx: &PipelineContext,
) -> Result<StaticAnalysisResult> {
    let data = fs::read(path).map_err(|e| SentinelError::FileNotFound(e.to_string()))?;
    let sample = if data.len() > max_bytes {
        &data[..max_bytes]
    } else {
        &data[..]
    };

    let entropy = shannon_entropy(sample);
    let strings = extract_printable_strings(sample, 6);
    let urls = extract_urls(&strings);
    let imported_apis = extract_suspicious_apis(&strings);
    let is_packed = is_likely_packed(entropy);
    let is_obfuscated = entropy > 6.5 && strings.len() < 10;

    let mut partial = StaticAnalysisResult {
        entropy,
        is_packed,
        is_obfuscated,
        strings: strings.iter().take(500).cloned().collect(),
        urls,
        imported_apis: imported_apis.clone(),
        exported_apis: vec![],
        yara_matches: vec![],
        mitre_techniques: vec![],
        certificate: None,
        metadata: serde_json::json!({
            "size_bytes": data.len(),
            "sampled_bytes": sample.len(),
            "string_count": strings.len(),
        }),
    };

    partial.mitre_techniques = map_to_mitre(&imported_apis, &partial.urls);
    partial.yara_matches = yara.scan(sample, entropy);

    Ok(partial)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sentinelx_core::ExecutionSession;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn analyzes_text_file() {
        let mut tmp = NamedTempFile::new().unwrap();
        std::io::Write::write_all(
            &mut tmp,
            b"hello world http://example.com VirtualAlloc test",
        )
        .unwrap();
        let file = sentinelx_core::FileMetadata::from_path(tmp.path().to_path_buf());
        let session = ExecutionSession::new(file, "test");
        let ctx = PipelineContext::new(session);
        let analyzer = DefaultStaticAnalyzer::default();
        let result = analyzer.analyze(&ctx).await.unwrap();
        assert!(result.entropy >= 0.0);
        assert!(!result.urls.is_empty());
    }
}
