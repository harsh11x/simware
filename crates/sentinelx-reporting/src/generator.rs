use async_trait::async_trait;
use sentinelx_core::{
    AnalysisReport, Decision, PipelineContext, ReportFormat, ReportGenerator, Result,
    RiskAssessment, RiskLevel,
};
use uuid::Uuid;

pub struct DefaultReportGenerator;

#[async_trait]
impl ReportGenerator for DefaultReportGenerator {
    async fn generate(&self, ctx: &PipelineContext) -> Result<AnalysisReport> {
        let risk = ctx.risk.clone().unwrap_or_else(|| RiskAssessment {
            score: 0.0,
            level: RiskLevel::Safe,
            confidence: 0.0,
            categories: vec![],
            reasoning: vec![],
        });

        let decision = ctx.decision.clone().unwrap_or_default();
        let executive_summary = ctx
            .explainability
            .as_ref()
            .map(|e| e.executive_summary.clone())
            .unwrap_or_else(|| "Analysis complete.".into());

        let technical_summary = build_technical_summary(ctx);

        Ok(AnalysisReport {
            id: Uuid::new_v4(),
            session_id: ctx.session_id(),
            generated_at: chrono::Utc::now(),
            executive_summary,
            technical_summary,
            risk,
            decision,
            timeline: ctx
                .correlation
                .as_ref()
                .map(|c| c.timeline.clone())
                .unwrap_or_default(),
            mitre_mappings: ctx
                .static_analysis
                .as_ref()
                .map(|s| s.mitre_techniques.clone())
                .unwrap_or_default(),
            iocs: ctx
                .threat_intel
                .as_ref()
                .map(|t| t.iocs.clone())
                .unwrap_or_default(),
            behavior_graph: ctx
                .correlation
                .as_ref()
                .map(|c| c.execution_graph.clone())
                .unwrap_or(serde_json::json!({})),
            static_analysis: ctx.static_analysis.clone(),
            threat_intel: ctx.threat_intel.clone(),
            classification: ctx.classification.clone(),
            explainability: ctx.explainability.clone(),
            correlation: ctx.correlation.clone(),
        })
    }

    async fn export(&self, report: &AnalysisReport, format: ReportFormat) -> Result<Vec<u8>> {
        match format {
            ReportFormat::Json => Ok(serde_json::to_vec_pretty(report).map_err(|e| {
                sentinelx_core::SentinelError::Internal(e.to_string())
            })?),
            ReportFormat::Html => Ok(render_html(report).into_bytes()),
            ReportFormat::Pdf => {
                // PDF generation via printpdf/wkhtmltopdf can be integrated later.
                Ok(format!(
                    "SentinelX Report\nSession: {}\nRisk: {:?}\n\n{}",
                    report.session_id, report.risk.level, report.executive_summary
                )
                .into_bytes())
            }
        }
    }
}

fn build_technical_summary(ctx: &PipelineContext) -> String {
    let mut parts = vec![format!("Session: {}", ctx.session_id())];

    if let Some(s) = &ctx.static_analysis {
        parts.push(format!(
            "Static: entropy={:.2}, packed={}, yara_matches={}",
            s.entropy,
            s.is_packed,
            s.yara_matches.len()
        ));
    }
    if let Some(b) = &ctx.behavior {
        parts.push(format!(
            "Behavior: events={}, deception_hits={}",
            b.events.len(),
            b.deception_hits.len()
        ));
    }
    parts.join("\n")
}

fn render_html(report: &AnalysisReport) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>SentinelX Report — {session_id}</title>
  <style>
    body {{ font-family: system-ui, sans-serif; background: #0f172a; color: #e2e8f0; padding: 2rem; }}
    h1 {{ color: #38bdf8; }}
    .risk {{ font-size: 1.5rem; padding: 1rem; border-radius: 8px; background: #1e293b; }}
    .section {{ margin-top: 2rem; }}
  </style>
</head>
<body>
  <h1>SentinelX Analysis Report</h1>
  <p>Generated: {generated_at}</p>
  <div class="risk">Risk Level: {risk_level} (score: {risk_score:.2})</div>
  <div class="section">
    <h2>Executive Summary</h2>
    <p>{executive_summary}</p>
  </div>
  <div class="section">
    <h2>Decision</h2>
    <p>{decision:?}</p>
  </div>
  <div class="section">
    <h2>Technical Summary</h2>
    <pre>{technical_summary}</pre>
  </div>
</body>
</html>"#,
        session_id = report.session_id,
        generated_at = report.generated_at,
        risk_level = report.risk.level.as_str(),
        risk_score = report.risk.score,
        executive_summary = report.executive_summary,
        decision = report.decision.action,
        technical_summary = report.technical_summary,
    )
}
