use async_trait::async_trait;
use sentinelx_core::{
    BehaviorAnalysisResult, BehaviorCorrelator, CorrelationResult, Result, TimelineEntry,
};
use uuid::Uuid;

pub struct DefaultBehaviorCorrelator;

#[async_trait]
impl BehaviorCorrelator for DefaultBehaviorCorrelator {
    async fn correlate(&self, behavior: &BehaviorAnalysisResult) -> Result<CorrelationResult> {
        let mut timeline: Vec<TimelineEntry> = behavior
            .events
            .iter()
            .map(|e| TimelineEntry {
                timestamp: e.timestamp,
                summary: format!("{:?}: {}", e.event_type, e.process_name),
                severity: severity_for_event(&e.event_type),
                event_ids: vec![Uuid::new_v4()],
            })
            .collect();

        timeline.sort_by_key(|t| t.timestamp);

        let mut confidence: f64 = 0.3;
        if !behavior.deception_hits.is_empty() {
            confidence += 0.3;
        }
        if !behavior.persistence_indicators.is_empty() {
            confidence += 0.2;
        }
        if behavior
            .network_observations
            .iter()
            .any(|n| !n.blocked)
        {
            confidence += 0.15;
        }
        confidence = confidence.min(1.0);

        let execution_graph = serde_json::json!({
            "root": behavior.process_tree,
            "event_count": behavior.events.len(),
            "deception_hits": behavior.deception_hits.len(),
        });

        Ok(CorrelationResult {
            timeline,
            execution_graph,
            attack_chain_confidence: confidence,
            related_events: vec![],
        })
    }
}

fn severity_for_event(event_type: &sentinelx_core::BehaviorEventType) -> String {
    use sentinelx_core::BehaviorEventType::*;
    match event_type {
        DeceptionArtifactAccess | PersistenceAttempt | PrivilegeChange => "high".into(),
        NetworkConnect | RegistryWrite | ServiceCreate => "medium".into(),
        _ => "low".into(),
    }
}
