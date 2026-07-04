use sentinelx_agent::AnalysisPipeline;
use sentinelx_storage::SqliteAnalysisStore;
use std::sync::Arc;

pub struct AppState {
    pub pipeline: AnalysisPipeline,
    pub store: Arc<SqliteAnalysisStore>,
}

impl AppState {
    pub async fn new(database_url: &str) -> sentinelx_core::Result<Self> {
        let store = Arc::new(SqliteAnalysisStore::new(database_url).await?);
        let pipeline = AnalysisPipeline::new().with_store(store.clone());
        Ok(Self { pipeline, store })
    }
}
