use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::thread;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResponse {
    pub id: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisStatus {
    pub id: String,
    pub status: String,
    pub aiRiskScore: Option<f64>,
    pub finalDecision: Option<String>,
}

pub struct BackendClient {
    client: Client,
    base_url: String,
}

impl BackendClient {
    pub fn new(base_url: &str) -> Self {
        BackendClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    fn calculate_hash(path: &str) -> String {
        // Fast dummy hash for now, replace with actual sha256 later
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn submit_for_analysis(&self, file_path: &str) -> Result<String, String> {
        let hash = Self::calculate_hash(file_path);
        
        // Since we aren't actually reading the binary for the dummy setup, 
        // we'll just send the hash to the backend.
        let response = self.client.post(&format!("{}/api/v1/analyze", self.base_url))
            .json(&serde_json::json!({
                "file_path": file_path,
                "file_hash": hash
            }))
            .send()
            .map_err(|e| e.to_string())?;

        let data: AnalysisResponse = response.json().map_err(|e| e.to_string())?;
        Ok(data.id)
    }

    pub fn wait_for_verdict(&self, analysis_id: &str) -> Result<String, String> {
        let url = format!("{}/api/v1/analysis/{}", self.base_url, analysis_id);
        
        loop {
            let response = self.client.get(&url).send().map_err(|e| e.to_string())?;
            let status: AnalysisStatus = response.json().map_err(|e| e.to_string())?;

            if status.status == "completed" {
                return Ok(status.finalDecision.unwrap_or_else(|| "ALLOW".to_string()));
            }

            thread::sleep(Duration::from_secs(2));
        }
    }
}
