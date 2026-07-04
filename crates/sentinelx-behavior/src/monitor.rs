use async_trait::async_trait;
use sentinelx_core::{
    BehaviorAnalysisResult, BehaviorEvent, BehaviorEventType, BehaviorMonitor, ProcessNode, Result,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct DefaultBehaviorMonitor {
    events: Arc<RwLock<HashMap<String, Vec<BehaviorEvent>>>>,
}

impl Default for DefaultBehaviorMonitor {
    fn default() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl BehaviorMonitor for DefaultBehaviorMonitor {
    async fn start(&self, instance_id: &str) -> Result<()> {
        tracing::info!(instance_id, "behavior monitoring started");
        self.events.write().await.insert(instance_id.to_string(), vec![]);
        Ok(())
    }

    async fn stop(&self, instance_id: &str) -> Result<BehaviorAnalysisResult> {
        let events = self
            .events
            .write()
            .await
            .remove(instance_id)
            .unwrap_or_default();

        let process_tree = build_process_tree(&events);
        let deception_hits = events
            .iter()
            .filter(|e| e.event_type == BehaviorEventType::DeceptionArtifactAccess)
            .map(|e| sentinelx_core::DeceptionHit {
                artifact_path: e.details["path"].as_str().unwrap_or("").into(),
                artifact_type: e.details["type"].as_str().unwrap_or("unknown").into(),
                access_type: e.details["access"].as_str().unwrap_or("read").into(),
                timestamp: e.timestamp,
            })
            .collect();

        let network_observations = events
            .iter()
            .filter(|e| e.event_type == BehaviorEventType::NetworkConnect)
            .map(|e| sentinelx_core::NetworkObservation {
                destination: e.details["destination"].as_str().unwrap_or("").into(),
                port: e.details["port"].as_u64().unwrap_or(0) as u16,
                protocol: e.details["protocol"].as_str().unwrap_or("tcp").into(),
                dns_query: e.details["dns"].as_str().map(String::from),
                blocked: e.details["blocked"].as_bool().unwrap_or(true),
            })
            .collect();

        let persistence_indicators = events
            .iter()
            .filter(|e| e.event_type == BehaviorEventType::PersistenceAttempt)
            .filter_map(|e| e.details["mechanism"].as_str().map(String::from))
            .collect();

        Ok(BehaviorAnalysisResult {
            events,
            process_tree,
            deception_hits,
            network_observations,
            persistence_indicators,
        })
    }
}

impl DefaultBehaviorMonitor {
    pub async fn record_event(&self, instance_id: &str, event: BehaviorEvent) {
        if let Some(events) = self.events.write().await.get_mut(instance_id) {
            events.push(event);
        }
    }
}

fn build_process_tree(events: &[BehaviorEvent]) -> ProcessNode {
    let root_event = events
        .iter()
        .find(|e| e.event_type == BehaviorEventType::ProcessCreate)
        .cloned();

    match root_event {
        Some(e) => ProcessNode {
            pid: e.process_id,
            name: e.process_name,
            command_line: e.details["command_line"].as_str().map(String::from),
            children: vec![],
        },
        None => ProcessNode {
            pid: 0,
            name: "unknown".into(),
            command_line: None,
            children: vec![],
        },
    }
}

pub fn make_event(
    event_type: BehaviorEventType,
    process_id: u32,
    process_name: &str,
    details: serde_json::Value,
) -> BehaviorEvent {
    BehaviorEvent {
        timestamp: chrono::Utc::now(),
        event_type,
        process_id,
        process_name: process_name.into(),
        parent_process_id: None,
        details,
    }
}

pub fn event_id() -> Uuid {
    Uuid::new_v4()
}
