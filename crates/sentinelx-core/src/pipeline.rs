use crate::{
    AiRiskPrediction, Decision, ExecutionSession, FileHash, RiskAssessment, SessionStatus,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineContext {
    pub session: ExecutionSession,
    pub static_analysis: Option<StaticAnalysisResult>,
    pub threat_intel: Option<ThreatIntelResult>,
    pub ai_prediction: Option<AiRiskPrediction>,
    pub sandbox_result: Option<SandboxResult>,
    pub behavior: Option<BehaviorAnalysisResult>,
    pub correlation: Option<CorrelationResult>,
    pub explainability: Option<ExplainabilityResult>,
    pub classification: Option<ClassificationResult>,
    pub risk: Option<RiskAssessment>,
    pub decision: Option<Decision>,
}

impl PipelineContext {
    pub fn new(session: ExecutionSession) -> Self {
        Self {
            session,
            static_analysis: None,
            threat_intel: None,
            ai_prediction: None,
            sandbox_result: None,
            behavior: None,
            correlation: None,
            explainability: None,
            classification: None,
            risk: None,
            decision: None,
        }
    }

    pub fn session_id(&self) -> Uuid {
        self.session.id
    }

    pub fn set_status(&mut self, status: SessionStatus) {
        self.session.transition(status);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticAnalysisResult {
    pub entropy: f64,
    pub is_packed: bool,
    pub is_obfuscated: bool,
    pub strings: Vec<String>,
    pub urls: Vec<String>,
    pub imported_apis: Vec<String>,
    pub exported_apis: Vec<String>,
    pub yara_matches: Vec<YaraMatch>,
    pub mitre_techniques: Vec<MitreMapping>,
    pub certificate: Option<CertificateAnalysis>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YaraMatch {
    pub rule_name: String,
    pub namespace: String,
    pub tags: Vec<String>,
    pub meta: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitreMapping {
    pub technique_id: String,
    pub technique_name: String,
    pub tactic: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateAnalysis {
    pub subject: String,
    pub issuer: String,
    pub valid_from: String,
    pub valid_to: String,
    pub is_self_signed: bool,
    pub is_expired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelResult {
    pub hash_reputation: ReputationScore,
    pub certificate_reputation: Option<ReputationScore>,
    pub domain_reputations: Vec<(String, ReputationScore)>,
    pub ip_reputations: Vec<(String, ReputationScore)>,
    pub malware_families: Vec<String>,
    pub iocs: Vec<IndicatorOfCompromise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationScore {
    pub score: f64,
    pub source: String,
    pub verdict: String,
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorOfCompromise {
    pub ioc_type: String,
    pub value: String,
    pub confidence: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub sandbox_id: String,
    pub backend: String,
    pub execution_time_ms: u64,
    pub exit_code: Option<i32>,
    pub rolled_back: bool,
    pub network_blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: BehaviorEventType,
    pub process_id: u32,
    pub process_name: String,
    pub parent_process_id: Option<u32>,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BehaviorEventType {
    ProcessCreate,
    ProcessTerminate,
    FileCreate,
    FileModify,
    FileDelete,
    FileRead,
    RegistryWrite,
    ServiceCreate,
    ScheduledTask,
    NetworkConnect,
    DnsLookup,
    LibraryLoad,
    MemoryAllocate,
    PrivilegeChange,
    PersistenceAttempt,
    DeceptionArtifactAccess,
    ApiCall,
    Syscall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnalysisResult {
    pub events: Vec<BehaviorEvent>,
    pub process_tree: ProcessNode,
    pub deception_hits: Vec<DeceptionHit>,
    pub network_observations: Vec<NetworkObservation>,
    pub persistence_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessNode {
    pub pid: u32,
    pub name: String,
    pub command_line: Option<String>,
    pub children: Vec<ProcessNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeceptionHit {
    pub artifact_path: String,
    pub artifact_type: String,
    pub access_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkObservation {
    pub destination: String,
    pub port: u16,
    pub protocol: String,
    pub dns_query: Option<String>,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationResult {
    pub timeline: Vec<TimelineEntry>,
    pub execution_graph: serde_json::Value,
    pub attack_chain_confidence: f64,
    pub related_events: Vec<(Uuid, Uuid, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub summary: String,
    pub severity: String,
    pub event_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityResult {
    pub executive_summary: String,
    pub plain_english_findings: Vec<PlainEnglishFinding>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainEnglishFinding {
    pub title: String,
    pub explanation: String,
    pub why_it_matters: String,
    pub possible_risks: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub primary: crate::ThreatCategory,
    pub secondary: Vec<crate::ThreatCategory>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterceptResult {
    pub session: ExecutionSession,
    pub hash: FileHash,
    pub signature: crate::SignatureInfo,
    pub should_analyze: bool,
}
