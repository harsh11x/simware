use crate::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FileHash {
    pub sha256: String,
    pub sha1: Option<String>,
    pub md5: Option<String>,
    pub blake3: Option<String>,
}

impl FileHash {
    pub fn compute(path: &Path) -> Result<Self> {
        let file = File::open(path).map_err(|e| crate::SentinelError::FileNotFound(e.to_string()))?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; 8192];

        let mut sha256 = Sha256::new();
        let mut blake3_hasher = blake3::Hasher::new();

        loop {
            let n = reader
                .read(&mut buffer)
                .map_err(|e| crate::SentinelError::AnalysisFailed(e.to_string()))?;
            if n == 0 {
                break;
            }
            sha256.update(&buffer[..n]);
            blake3_hasher.update(&buffer[..n]);
        }

        Ok(Self {
            sha256: hex::encode(sha256.finalize()),
            sha1: None,
            md5: None,
            blake3: Some(blake3_hasher.finalize().to_hex().to_string()),
        })
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        let mut sha256 = Sha256::new();
        sha256.update(data);
        Self {
            sha256: hex::encode(sha256.finalize()),
            sha1: None,
            md5: None,
            blake3: Some(blake3::hash(data).to_hex().to_string()),
        }
    }
}
