use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RiskLevel {
    Safe,
    LowRisk,
    Suspicious,
    Dangerous,
    Critical,
}

impl RiskLevel {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s < 0.15 => RiskLevel::Safe,
            s if s < 0.35 => RiskLevel::LowRisk,
            s if s < 0.60 => RiskLevel::Suspicious,
            s if s < 0.85 => RiskLevel::Dangerous,
            _ => RiskLevel::Critical,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RiskLevel::Safe => "safe",
            RiskLevel::LowRisk => "low_risk",
            RiskLevel::Suspicious => "suspicious",
            RiskLevel::Dangerous => "dangerous",
            RiskLevel::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatCategory {
    Ransomware,
    Spyware,
    Trojan,
    Downloader,
    CredentialStealer,
    Cryptominer,
    Adware,
    Worm,
    Backdoor,
    RootkitIndicators,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionAction {
    Allow,
    Quarantine,
    Block,
    RequestAdminApproval,
    ExportReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub score: f64,
    pub level: RiskLevel,
    pub confidence: f64,
    pub categories: Vec<ThreatCategory>,
    pub reasoning: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub action: DecisionAction,
    pub risk: RiskAssessment,
    pub rationale: String,
    pub requires_user_confirmation: bool,
}

impl Default for Decision {
    fn default() -> Self {
        Self {
            action: DecisionAction::Quarantine,
            risk: RiskAssessment {
                score: 0.5,
                level: RiskLevel::Suspicious,
                confidence: 0.5,
                categories: vec![],
                reasoning: vec!["Insufficient evidence — defaulting to quarantine.".into()],
            },
            rationale: "Default secure policy: quarantine when confidence is high but not absolute."
                .into(),
            requires_user_confirmation: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRiskPrediction {
    pub probability: f64,
    pub confidence: f64,
    pub reasoning: Vec<String>,
    pub feature_importance: Vec<(String, f64)>,
}
