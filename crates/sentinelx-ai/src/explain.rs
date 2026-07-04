use async_trait::async_trait;
use sentinelx_core::{
    ClassificationResult, ExplainabilityResult, ExplainabilityEngine, PipelineContext,
    PlainEnglishFinding, Result, ThreatCategory, ThreatClassifier,
};

pub struct DefaultExplainabilityEngine;

#[async_trait]
impl ExplainabilityEngine for DefaultExplainabilityEngine {
    async fn explain(&self, ctx: &PipelineContext) -> Result<ExplainabilityResult> {
        let mut findings = Vec::new();

        if let Some(static_result) = &ctx.static_analysis {
            if static_result.is_packed {
                findings.push(PlainEnglishFinding {
                    title: "Packed or encrypted content detected".into(),
                    explanation: "The file's data appears compressed or encrypted, which malware often uses to hide its true purpose from scanners.".into(),
                    why_it_matters: "Legitimate software rarely needs heavy packing; this increases suspicion.".into(),
                    possible_risks: vec!["Evasion".into(), "Hidden payload".into()],
                    confidence: 0.75,
                });
            }
            for url in static_result.urls.iter().take(3) {
                findings.push(PlainEnglishFinding {
                    title: "Network destination embedded in file".into(),
                    explanation: format!("The file contains a reference to {url}."),
                    why_it_matters: "Malware may download additional stages or exfiltrate data.".into(),
                    possible_risks: vec!["Command and control".into(), "Data exfiltration".into()],
                    confidence: 0.6,
                });
            }
        }

        if let Some(behavior) = &ctx.behavior {
            if !behavior.deception_hits.is_empty() {
                findings.push(PlainEnglishFinding {
                    title: "Attempted access to decoy sensitive files".into(),
                    explanation: format!(
                        "The program tried to read {} bait files such as fake passwords or SSH keys.",
                        behavior.deception_hits.len()
                    ),
                    why_it_matters: "Credential stealers hunt for secrets on disk.".into(),
                    possible_risks: vec!["Credential theft".into(), "Spyware".into()],
                    confidence: 0.9,
                });
            }
        }

        let executive_summary = if findings.is_empty() {
            "No significant suspicious behavior was observed during analysis.".into()
        } else {
            format!(
                "Analysis identified {} potentially concerning behaviors requiring review.",
                findings.len()
            )
        };

        let confidence = findings.iter().map(|f| f.confidence).sum::<f64>()
            / findings.len().max(1) as f64;

        Ok(ExplainabilityResult {
            executive_summary,
            plain_english_findings: findings,
            confidence,
        })
    }
}

pub struct DefaultThreatClassifier;

#[async_trait]
impl ThreatClassifier for DefaultThreatClassifier {
    async fn classify(&self, ctx: &PipelineContext) -> Result<ClassificationResult> {
        let mut secondary = Vec::new();
        let mut confidence: f64 = 0.5;
        let mut primary = ThreatCategory::Unknown;

        if let Some(behavior) = &ctx.behavior {
            if !behavior.deception_hits.is_empty() {
                primary = ThreatCategory::CredentialStealer;
                confidence = 0.85;
            }
            if behavior
                .events
                .iter()
                .any(|e| matches!(e.event_type, sentinelx_core::BehaviorEventType::FileModify))
            {
                secondary.push(ThreatCategory::Ransomware);
            }
            if !behavior.network_observations.is_empty() {
                secondary.push(ThreatCategory::Downloader);
            }
        }

        if let Some(static_result) = &ctx.static_analysis {
            if static_result
                .imported_apis
                .iter()
                .any(|a| a.to_lowercase().contains("crypt"))
            {
                secondary.push(ThreatCategory::Cryptominer);
            }
        }

        Ok(ClassificationResult {
            primary,
            secondary,
            confidence,
        })
    }
}
