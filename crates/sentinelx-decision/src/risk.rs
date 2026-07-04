use async_trait::async_trait;
use sentinelx_core::{
    PipelineContext, Result, RiskAssessment, RiskEngine, RiskLevel, ThreatCategory,
};

pub struct DefaultRiskEngine;

#[async_trait]
impl RiskEngine for DefaultRiskEngine {
    async fn assess(&self, ctx: &PipelineContext) -> Result<RiskAssessment> {
        let mut score: f64 = 0.0;
        let mut reasoning = Vec::new();
        let mut categories = Vec::new();

        if let Some(prediction) = &ctx.ai_prediction {
            score += prediction.probability * 0.35;
            reasoning.extend(prediction.reasoning.clone());
        }

        if let Some(static_result) = &ctx.static_analysis {
            if static_result.is_packed {
                score += 0.15;
            }
            if !static_result.yara_matches.is_empty() {
                score += 0.1;
                reasoning.push(format!(
                    "{} YARA rule matches.",
                    static_result.yara_matches.len()
                ));
            }
        }

        if let Some(ti) = &ctx.threat_intel {
            score += ti.hash_reputation.score * 0.2;
            for _ in &ti.malware_families {
                categories.push(ThreatCategory::Trojan);
            }
        }

        if let Some(behavior) = &ctx.behavior {
            if !behavior.deception_hits.is_empty() {
                score += 0.25;
                categories.push(ThreatCategory::CredentialStealer);
                reasoning.push("Deception artifact access detected.".into());
            }
            if !behavior.persistence_indicators.is_empty() {
                score += 0.15;
                reasoning.push("Persistence mechanisms observed.".into());
            }
        }

        if let Some(classification) = &ctx.classification {
            categories.push(classification.primary.clone());
            categories.extend(classification.secondary.clone());
            score += classification.confidence * 0.1;
        }

        if let Some(correlation) = &ctx.correlation {
            score += correlation.attack_chain_confidence * 0.1;
        }

        score = score.min(1.0);
        let confidence = compute_confidence(ctx);
        categories = dedup_categories(categories);

        Ok(RiskAssessment {
            score,
            level: RiskLevel::from_score(score),
            confidence,
            categories,
            reasoning,
        })
    }
}

fn compute_confidence(ctx: &PipelineContext) -> f64 {
    let mut signals = 0u32;
    let mut total = 0u32;

    if ctx.static_analysis.is_some() {
        signals += 1;
    }
    total += 1;
    if ctx.behavior.is_some() {
        signals += 1;
    }
    total += 1;
    if ctx.ai_prediction.is_some() {
        signals += 1;
    }
    total += 1;
    if ctx.threat_intel.is_some() {
        signals += 1;
    }
    total += 1;

    (signals as f64 / total as f64).max(0.4)
}

fn dedup_categories(categories: Vec<ThreatCategory>) -> Vec<ThreatCategory> {
    let mut seen = std::collections::HashSet::new();
    categories
        .into_iter()
        .filter(|c| seen.insert(format!("{c:?}")))
        .collect()
}
