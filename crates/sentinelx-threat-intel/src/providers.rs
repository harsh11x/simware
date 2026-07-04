use async_trait::async_trait;
use sentinelx_core::{
    IndicatorOfCompromise, PipelineContext, ReputationScore, Result, ThreatIntelProvider,
    ThreatIntelResult,
};

/// Local hash reputation cache — no external network calls.
pub struct LocalReputationProvider;

#[async_trait]
impl ThreatIntelProvider for LocalReputationProvider {
    fn name(&self) -> &str {
        "local-reputation"
    }

    async fn lookup(&self, ctx: &PipelineContext) -> Result<ThreatIntelResult> {
        let hash = ctx
            .session
            .hash
            .as_ref()
            .map(|h| h.sha256.as_str())
            .unwrap_or("unknown");

        Ok(ThreatIntelResult {
            hash_reputation: ReputationScore {
                score: 0.5,
                source: self.name().into(),
                verdict: "unknown".into(),
                last_seen: None,
            },
            certificate_reputation: None,
            domain_reputations: vec![],
            ip_reputations: vec![],
            malware_families: vec![],
            iocs: vec![IndicatorOfCompromise {
                ioc_type: "sha256".into(),
                value: hash.into(),
                confidence: 0.5,
                source: self.name().into(),
            }],
        })
    }
}

/// Stub for external feed integration (VirusTotal, MISP, etc.)
pub struct HttpFeedProvider {
    pub base_url: String,
    pub api_key_env: String,
}

#[async_trait]
impl ThreatIntelProvider for HttpFeedProvider {
    fn name(&self) -> &str {
        "http-feed"
    }

    async fn lookup(&self, ctx: &PipelineContext) -> Result<ThreatIntelResult> {
        let _api_key = std::env::var(&self.api_key_env).ok();
        tracing::debug!(url = %self.base_url, "external TI lookup skipped — no API key");
        LocalReputationProvider.lookup(ctx).await
    }
}
