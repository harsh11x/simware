use async_trait::async_trait;
use sentinelx_core::{AiRiskPrediction, PipelineContext, Result, RiskPredictor};

pub struct HeuristicRiskPredictor;

#[async_trait]
impl RiskPredictor for HeuristicRiskPredictor {
    async fn predict(&self, ctx: &PipelineContext) -> Result<AiRiskPrediction> {
        let mut score: f64 = 0.1;
        let mut reasoning = Vec::new();
        let mut features = Vec::new();

        if let Some(static_result) = &ctx.static_analysis {
            if static_result.is_packed {
                score += 0.25;
                reasoning.push("Binary appears packed — common evasion technique.".into());
                features.push(("is_packed".into(), 0.25));
            }
            if static_result.entropy > 7.0 {
                score += 0.15;
                reasoning.push(format!(
                    "High entropy ({:.2}) suggests encryption or compression.",
                    static_result.entropy
                ));
                features.push(("entropy".into(), static_result.entropy / 10.0));
            }
            if !static_result.urls.is_empty() {
                score += 0.1;
                reasoning.push(format!("Found {} embedded URLs.", static_result.urls.len()));
                features.push(("url_count".into(), static_result.urls.len() as f64 / 10.0));
            }
            if !static_result.imported_apis.is_empty() {
                score += 0.2;
                reasoning.push("Suspicious API references detected.".into());
                features.push(("suspicious_apis".into(), 0.2));
            }
        }

        if let Some(ti) = &ctx.threat_intel {
            if ti.hash_reputation.score > 0.7 {
                score += 0.3;
                reasoning.push("Threat intelligence indicates malicious reputation.".into());
            }
        }

        score = score.min(0.99);
        let confidence = if reasoning.len() >= 3 { 0.85 } else { 0.6 };

        Ok(AiRiskPrediction {
            probability: score,
            confidence,
            reasoning,
            feature_importance: features,
        })
    }
}

pub struct MlRiskPredictor {
    endpoint: String,
    client: reqwest::Client,
}

impl MlRiskPredictor {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl RiskPredictor for MlRiskPredictor {
    async fn predict(&self, ctx: &PipelineContext) -> Result<AiRiskPrediction> {
        let payload = serde_json::json!({
            "session_id": ctx.session_id(),
            "static_analysis": ctx.static_analysis,
            "threat_intel": ctx.threat_intel,
        });

        match self
            .client
            .post(format!("{}/predict", self.endpoint))
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                resp.json().await.map_err(|e| {
                    sentinelx_core::SentinelError::Internal(e.to_string())
                })
            }
            _ => {
                tracing::warn!("ML service unavailable — falling back to heuristic predictor");
                HeuristicRiskPredictor.predict(ctx).await
            }
        }
    }
}
