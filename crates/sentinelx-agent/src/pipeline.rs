use async_trait::async_trait;
use sentinelx_ai::{
    DefaultExplainabilityEngine, DefaultThreatClassifier, HeuristicRiskPredictor,
};
use sentinelx_behavior::{DefaultBehaviorCorrelator, DefaultBehaviorMonitor};
use sentinelx_core::{
    AnalysisReport, AnalysisStore, BehaviorMonitor, DecisionEngine, ExplainabilityEngine,
    PipelineContext, ReportGenerator, Result, RiskEngine, RiskPredictor, SandboxBackend,
    SessionStatus, StaticAnalyzer, ThreatClassifier,
};
use sentinelx_decision::{DefaultDecisionEngine, DefaultRiskEngine};
use sentinelx_interceptor::DefaultInterceptor;
use sentinelx_reporting::DefaultReportGenerator;
use sentinelx_sandbox::{
    DefaultDeceptionEnvironment, DefaultUserSimulator, LocalSandboxBackend, SandboxOrchestrator,
};
use sentinelx_static_analysis::DefaultStaticAnalyzer;
use sentinelx_threat_intel::ThreatIntelAggregator;
use std::path::Path;
use std::sync::Arc;
use tracing::{info, instrument};

pub struct AnalysisPipeline {
    interceptor: DefaultInterceptor,
    static_analyzer: DefaultStaticAnalyzer,
    threat_intel: ThreatIntelAggregator,
    risk_predictor: HeuristicRiskPredictor,
    sandbox: SandboxOrchestrator,
    behavior_monitor: DefaultBehaviorMonitor,
    correlator: DefaultBehaviorCorrelator,
    explainability: DefaultExplainabilityEngine,
    classifier: DefaultThreatClassifier,
    risk_engine: DefaultRiskEngine,
    decision_engine: DefaultDecisionEngine,
    report_generator: DefaultReportGenerator,
    store: Option<Arc<dyn AnalysisStore>>,
    skip_sandbox: bool,
}

impl AnalysisPipeline {
    pub fn new() -> Self {
        let backend: Arc<dyn SandboxBackend> = Arc::new(LocalSandboxBackend::default());
        let sandbox = SandboxOrchestrator::new(
            backend,
            Arc::new(DefaultUserSimulator),
            Arc::new(DefaultDeceptionEnvironment::default()),
        );

        Self {
            interceptor: DefaultInterceptor::default(),
            static_analyzer: DefaultStaticAnalyzer::default(),
            threat_intel: ThreatIntelAggregator::with_defaults(),
            risk_predictor: HeuristicRiskPredictor,
            sandbox,
            behavior_monitor: DefaultBehaviorMonitor::default(),
            correlator: DefaultBehaviorCorrelator,
            explainability: DefaultExplainabilityEngine,
            classifier: DefaultThreatClassifier,
            risk_engine: DefaultRiskEngine,
            decision_engine: DefaultDecisionEngine::default(),
            report_generator: DefaultReportGenerator,
            store: None,
            skip_sandbox: false,
        }
    }

    pub fn with_store(mut self, store: Arc<dyn AnalysisStore>) -> Self {
        self.store = Some(store);
        self
    }

    pub fn skip_sandbox(mut self, skip: bool) -> Self {
        self.skip_sandbox = skip;
        self
    }

    #[instrument(skip(self), fields(path = %path.display()))]
    pub async fn analyze_file(&self, path: &Path) -> Result<(PipelineContext, AnalysisReport)> {
        info!("starting analysis pipeline");

        // 1. Intercept
        let intercept = self.interceptor.intercept(path).await?;
        let mut ctx = PipelineContext::new(intercept.session);
        ctx.session.hash = Some(intercept.hash);

        if !intercept.should_analyze {
            info!("trusted signed binary — fast-path allow");
            ctx.risk = Some(sentinelx_core::RiskAssessment {
                score: 0.05,
                level: sentinelx_core::RiskLevel::Safe,
                confidence: 0.9,
                categories: vec![],
                reasoning: vec!["Trusted publisher signature.".into()],
            });
            ctx.decision = Some(sentinelx_core::Decision {
                action: sentinelx_core::DecisionAction::Allow,
                risk: ctx.risk.clone().unwrap(),
                rationale: "Trusted signed binary.".into(),
                requires_user_confirmation: false,
            });
            let report = self.report_generator.generate(&ctx).await?;
            self.persist(&ctx).await;
            return Ok((ctx, report));
        }

        // 2. Static Analysis
        ctx.set_status(SessionStatus::StaticAnalysis);
        ctx.static_analysis = Some(self.static_analyzer.analyze(&ctx).await?);

        // 3. Threat Intelligence
        ctx.set_status(SessionStatus::ThreatIntel);
        ctx.threat_intel = Some(self.threat_intel.lookup(&ctx).await?);

        // 4. AI Risk Prediction
        ctx.set_status(SessionStatus::RiskPrediction);
        ctx.ai_prediction = Some(self.risk_predictor.predict(&ctx).await?);

        // 5. Sandbox + Behavior (optional)
        if !self.skip_sandbox {
            ctx.set_status(SessionStatus::Sandboxing);
            let instance_id = self.sandbox.backend.create_instance().await?;
            self.behavior_monitor.start(&instance_id).await?;
            ctx.sandbox_result = Some(
                self.sandbox
                    .backend
                    .execute(&instance_id, path, 30)
                    .await?,
            );
            ctx.set_status(SessionStatus::BehaviorMonitoring);
            ctx.behavior = Some(self.behavior_monitor.stop(&instance_id).await?);
            let _ = self.sandbox.backend.destroy_instance(&instance_id).await;
        }

        // 6. Correlation
        if let Some(behavior) = &ctx.behavior {
            ctx.set_status(SessionStatus::Correlating);
            ctx.correlation = Some(self.correlator.correlate(behavior).await?);
        }

        // 7. Classification & Explainability
        ctx.classification = Some(self.classifier.classify(&ctx).await?);
        ctx.explainability = Some(self.explainability.explain(&ctx).await?);

        // 8. Risk & Decision
        ctx.set_status(SessionStatus::Scoring);
        ctx.risk = Some(self.risk_engine.assess(&ctx).await?);
        ctx.decision = Some(self.decision_engine.decide(&ctx).await?);

        ctx.set_status(SessionStatus::Completed);
        let report = self.report_generator.generate(&ctx).await?;
        self.persist(&ctx).await;

        info!(
            session_id = %ctx.session_id(),
            decision = ?ctx.decision.as_ref().map(|d| &d.action),
            risk = ctx.risk.as_ref().map(|r| r.score),
            "analysis complete"
        );

        Ok((ctx, report))
    }

    async fn persist(&self, ctx: &PipelineContext) {
        if let Some(store) = &self.store {
            if let Err(e) = store.save_session(ctx).await {
                tracing::warn!(error = %e, "failed to persist session");
            }
        }
    }
}

impl Default for AnalysisPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisPipeline {
    /// Creates a new pipeline instance sharing the same store reference pattern.
    pub fn clone_for_request(&self) -> Self {
        Self {
            interceptor: DefaultInterceptor::default(),
            static_analyzer: DefaultStaticAnalyzer::default(),
            threat_intel: ThreatIntelAggregator::with_defaults(),
            risk_predictor: HeuristicRiskPredictor,
            sandbox: SandboxOrchestrator::new(
                Arc::new(LocalSandboxBackend::default()),
                Arc::new(DefaultUserSimulator),
                Arc::new(DefaultDeceptionEnvironment::default()),
            ),
            behavior_monitor: DefaultBehaviorMonitor::default(),
            correlator: DefaultBehaviorCorrelator,
            explainability: DefaultExplainabilityEngine,
            classifier: DefaultThreatClassifier,
            risk_engine: DefaultRiskEngine,
            decision_engine: DefaultDecisionEngine::default(),
            report_generator: DefaultReportGenerator,
            store: self.store.clone(),
            skip_sandbox: self.skip_sandbox,
        }
    }
}
