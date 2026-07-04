use crate::{
    AiRiskPrediction, BehaviorAnalysisResult, ClassificationResult, CorrelationResult,
    Decision, ExplainabilityResult, InterceptResult, PipelineContext, RiskAssessment,
    SandboxResult, StaticAnalysisResult, ThreatIntelResult,
};
use async_trait::async_trait;

/// Intercepts file execution before OS launch.
#[async_trait]
pub trait ExecutionInterceptor: Send + Sync {
    async fn intercept(&self, path: &std::path::Path) -> crate::Result<InterceptResult>;
}

/// Static analysis without execution.
#[async_trait]
pub trait StaticAnalyzer: Send + Sync {
    async fn analyze(&self, ctx: &PipelineContext) -> crate::Result<StaticAnalysisResult>;
}

/// Provider-agnostic threat intelligence.
#[async_trait]
pub trait ThreatIntelProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn lookup(&self, ctx: &PipelineContext) -> crate::Result<ThreatIntelResult>;
}

/// ML-based pre-execution risk prediction.
#[async_trait]
pub trait RiskPredictor: Send + Sync {
    async fn predict(&self, ctx: &PipelineContext) -> crate::Result<AiRiskPrediction>;
}

/// Disposable isolated execution environment.
#[async_trait]
pub trait SandboxBackend: Send + Sync {
    fn backend_name(&self) -> &str;
    async fn create_instance(&self) -> crate::Result<String>;
    async fn execute(
        &self,
        instance_id: &str,
        target: &std::path::Path,
        timeout_secs: u64,
    ) -> crate::Result<SandboxResult>;
    async fn destroy_instance(&self, instance_id: &str) -> crate::Result<()>;
}

/// Simulates benign user activity inside sandbox.
#[async_trait]
pub trait UserSimulator: Send + Sync {
    async fn run_profile(&self, instance_id: &str, profile: &str) -> crate::Result<()>;
}

/// Populates deception artifacts and monitors access.
#[async_trait]
pub trait DeceptionEnvironment: Send + Sync {
    async fn populate(&self, instance_id: &str) -> crate::Result<()>;
    async fn collect_hits(&self, instance_id: &str) -> crate::Result<Vec<crate::DeceptionHit>>;
}

/// Collects behavioral telemetry from sandbox.
#[async_trait]
pub trait BehaviorMonitor: Send + Sync {
    async fn start(&self, instance_id: &str) -> crate::Result<()>;
    async fn stop(&self, instance_id: &str) -> crate::Result<BehaviorAnalysisResult>;
}

/// Correlates events into attack chains.
#[async_trait]
pub trait BehaviorCorrelator: Send + Sync {
    async fn correlate(&self, behavior: &BehaviorAnalysisResult) -> crate::Result<CorrelationResult>;
}

/// Translates telemetry to plain English.
#[async_trait]
pub trait ExplainabilityEngine: Send + Sync {
    async fn explain(&self, ctx: &PipelineContext) -> crate::Result<ExplainabilityResult>;
}

/// Classifies behavior into threat categories.
#[async_trait]
pub trait ThreatClassifier: Send + Sync {
    async fn classify(&self, ctx: &PipelineContext) -> crate::Result<ClassificationResult>;
}

/// Combines all evidence into risk score.
#[async_trait]
pub trait RiskEngine: Send + Sync {
    async fn assess(&self, ctx: &PipelineContext) -> crate::Result<RiskAssessment>;
}

/// Final allow/quarantine/block decision.
#[async_trait]
pub trait DecisionEngine: Send + Sync {
    async fn decide(&self, ctx: &PipelineContext) -> crate::Result<Decision>;
}

/// Report generation and export.
#[async_trait]
pub trait ReportGenerator: Send + Sync {
    async fn generate(&self, ctx: &PipelineContext) -> crate::Result<crate::AnalysisReport>;
    async fn export(
        &self,
        report: &crate::AnalysisReport,
        format: crate::ReportFormat,
    ) -> crate::Result<Vec<u8>>;
}

/// Persists analysis sessions and telemetry.
#[async_trait]
pub trait AnalysisStore: Send + Sync {
    async fn save_session(&self, ctx: &PipelineContext) -> crate::Result<()>;
    async fn load_session(&self, id: uuid::Uuid) -> crate::Result<PipelineContext>;
    async fn list_sessions(&self, limit: u32) -> crate::Result<Vec<PipelineContext>>;
}
