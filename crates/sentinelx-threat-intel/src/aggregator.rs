use async_trait::async_trait;
use sentinelx_core::{
    PipelineContext, ReputationScore, Result, ThreatIntelProvider, ThreatIntelResult,
};

pub struct ThreatIntelAggregator {
    providers: Vec<Box<dyn ThreatIntelProvider>>,
}

impl ThreatIntelAggregator {
    pub fn new(providers: Vec<Box<dyn ThreatIntelProvider>>) -> Self {
        Self { providers }
    }

    pub fn with_defaults() -> Self {
        Self::new(vec![
            Box::new(super::providers::LocalReputationProvider),
        ])
    }

    pub async fn lookup(&self, ctx: &PipelineContext) -> Result<ThreatIntelResult> {
        if self.providers.is_empty() {
            return Ok(ThreatIntelResult {
                hash_reputation: ReputationScore {
                    score: 0.5,
                    source: "none".into(),
                    verdict: "unknown".into(),
                    last_seen: None,
                },
                certificate_reputation: None,
                domain_reputations: vec![],
                ip_reputations: vec![],
                malware_families: vec![],
                iocs: vec![],
            });
        }

        let mut combined = self.providers[0].lookup(ctx).await?;
        for provider in self.providers.iter().skip(1) {
            let result = provider.lookup(ctx).await?;
            merge_results(&mut combined, result);
        }
        Ok(combined)
    }
}

fn merge_results(base: &mut ThreatIntelResult, other: ThreatIntelResult) {
    if other.hash_reputation.score > base.hash_reputation.score {
        base.hash_reputation = other.hash_reputation;
    }
    base.domain_reputations.extend(other.domain_reputations);
    base.ip_reputations.extend(other.ip_reputations);
    base.malware_families.extend(other.malware_families);
    base.iocs.extend(other.iocs);
}
