use crate::{
    ClassificationResult, CorrelationResult, Decision, ExplainabilityResult, InterceptResult,
    PipelineContext, RiskAssessment, StaticAnalysisResult, ThreatIntelResult,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub id: Uuid,
    pub session_id: Uuid,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub executive_summary: String,
    pub technical_summary: String,
    pub risk: RiskAssessment,
    pub decision: Decision,
    pub timeline: Vec<crate::TimelineEntry>,
    pub mitre_mappings: Vec<crate::MitreMapping>,
    pub iocs: Vec<crate::IndicatorOfCompromise>,
    pub behavior_graph: serde_json::Value,
    pub static_analysis: Option<StaticAnalysisResult>,
    pub threat_intel: Option<ThreatIntelResult>,
    pub classification: Option<ClassificationResult>,
    pub explainability: Option<ExplainabilityResult>,
    pub correlation: Option<CorrelationResult>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFormat {
    Json,
    Html,
    Pdf,
}
