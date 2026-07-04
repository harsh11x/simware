use sentinelx_core::{MitreMapping, YaraMatch};

pub struct YaraScanner {
    rules_path: Option<String>,
}

impl YaraScanner {
    pub fn new(rules_path: Option<String>) -> Self {
        Self { rules_path }
    }

    pub fn scan(&self, _data: &[u8], entropy: f64) -> Vec<YaraMatch> {
        let mut matches = Vec::new();

        // Heuristic rules when YARA-X native integration is not compiled in.
        if entropy > 7.5 {
            matches.push(YaraMatch {
                rule_name: "HEURISTIC_HIGH_ENTROPY".into(),
                namespace: "sentinelx",
                tags: vec!["entropy".into(), "suspicious".into()],
                meta: serde_json::json!({"entropy": entropy}),
            });
        }

        if self.rules_path.is_some() {
            tracing::debug!("YARA rules path configured: {:?}", self.rules_path);
        }

        matches
    }
}

pub fn map_to_mitre(imported_apis: &[String], urls: &[String]) -> Vec<MitreMapping> {
    let mut mappings = Vec::new();

    for api in imported_apis {
        let lower = api.to_lowercase();
        if lower.contains("writeprocessmemory") || lower.contains("createremotethread") {
            mappings.push(MitreMapping {
                technique_id: "T1055".into(),
                technique_name: "Process Injection".into(),
                tactic: "Defense Evasion".into(),
                evidence: format!("Suspicious API reference: {api}"),
            });
        }
        if lower.contains("regsetvalue") {
            mappings.push(MitreMapping {
                technique_id: "T1112".into(),
                technique_name: "Modify Registry".into(),
                tactic: "Defense Evasion".into(),
                evidence: format!("Registry API reference: {api}"),
            });
        }
    }

    if !urls.is_empty() {
        mappings.push(MitreMapping {
            technique_id: "T1071".into(),
            technique_name: "Application Layer Protocol".into(),
            tactic: "Command and Control".into(),
            evidence: format!("Embedded URLs detected: {}", urls.len()),
        });
    }

    mappings
}
