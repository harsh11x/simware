use async_trait::async_trait;
use sentinelx_core::{
    Decision, DecisionAction, DecisionEngine, PipelineContext, Result, RiskAssessment, RiskLevel,
};

pub struct DefaultDecisionEngine {
    quarantine_threshold: f64,
    block_threshold: f64,
}

impl Default for DefaultDecisionEngine {
    fn default() -> Self {
        Self {
            quarantine_threshold: 0.45,
            block_threshold: 0.80,
        }
    }
}

#[async_trait]
impl DecisionEngine for DefaultDecisionEngine {
    async fn decide(&self, ctx: &PipelineContext) -> Result<Decision> {
        let risk = ctx.risk.clone().unwrap_or_else(|| RiskAssessment {
            score: 0.5,
            level: RiskLevel::Suspicious,
            confidence: 0.5,
            categories: vec![],
            reasoning: vec![],
        });

        let (action, rationale, requires_confirmation) = match risk.level {
            RiskLevel::Safe => (
                DecisionAction::Allow,
                "File appears safe based on available evidence.".into(),
                false,
            ),
            RiskLevel::LowRisk => (
                DecisionAction::Allow,
                "Low risk detected — allowing with logging.".into(),
                false,
            ),
            RiskLevel::Suspicious => (
                DecisionAction::Quarantine,
                "Suspicious indicators present — quarantining pending review.".into(),
                true,
            ),
            RiskLevel::Dangerous if risk.score >= self.block_threshold => (
                DecisionAction::Block,
                "High-confidence malicious behavior — blocking execution.".into(),
                false,
            ),
            RiskLevel::Dangerous => (
                DecisionAction::Quarantine,
                "Dangerous indicators with moderate confidence — quarantining.".into(),
                true,
            ),
            RiskLevel::Critical => (
                DecisionAction::Block,
                "Critical threat detected — blocking immediately.".into(),
                false,
            ),
        };

        let action = if risk.score >= self.quarantine_threshold
            && risk.score < self.block_threshold
            && risk.confidence >= 0.7
            && matches!(action, DecisionAction::Allow)
        {
            DecisionAction::Quarantine
        } else {
            action
        };

        Ok(Decision {
            action,
            risk,
            rationale,
            requires_user_confirmation: requires_confirmation,
        })
    }
}
