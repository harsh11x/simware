use async_trait::async_trait;
use sentinelx_core::{AnalysisStore, PipelineContext, Result, SentinelError};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use uuid::Uuid;

pub struct SqliteAnalysisStore {
    pool: SqlitePool,
}

impl SqliteAnalysisStore {
    pub async fn new(database_url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)
            .map_err(|e| SentinelError::StorageError(e.to_string()))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| SentinelError::StorageError(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS analysis_sessions (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                file_hash TEXT,
                status TEXT NOT NULL,
                platform TEXT NOT NULL,
                context_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_sessions_created ON analysis_sessions(created_at DESC);
            "#,
        )
        .execute(&pool)
        .await
        .map_err(|e| SentinelError::StorageError(e.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn in_memory() -> Result<Self> {
        Self::new("sqlite::memory:").await
    }
}

#[async_trait]
impl AnalysisStore for SqliteAnalysisStore {
    async fn save_session(&self, ctx: &PipelineContext) -> Result<()> {
        let json = serde_json::to_string(ctx)
            .map_err(|e| SentinelError::StorageError(e.to_string()))?;
        let hash = ctx
            .session
            .hash
            .as_ref()
            .map(|h| h.sha256.as_str())
            .unwrap_or("");

        sqlx::query(
            r#"
            INSERT INTO analysis_sessions (id, file_path, file_hash, status, platform, context_json, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO UPDATE SET
                status = excluded.status,
                context_json = excluded.context_json,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(ctx.session_id().to_string())
        .bind(ctx.session.file.path.to_string_lossy().to_string())
        .bind(hash)
        .bind(format!("{:?}", ctx.session.status))
        .bind(format!("{:?}", ctx.session.platform))
        .bind(json)
        .bind(ctx.session.created_at.to_rfc3339())
        .bind(ctx.session.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| SentinelError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn load_session(&self, id: Uuid) -> Result<PipelineContext> {
        let row: (String,) = sqlx::query_as(
            "SELECT context_json FROM analysis_sessions WHERE id = ?1",
        )
        .bind(id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| SentinelError::StorageError(e.to_string()))?;

        serde_json::from_str(&row.0).map_err(|e| SentinelError::StorageError(e.to_string()))
    }

    async fn list_sessions(&self, limit: u32) -> Result<Vec<PipelineContext>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT context_json FROM analysis_sessions ORDER BY created_at DESC LIMIT ?1",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SentinelError::StorageError(e.to_string()))?;

        rows.into_iter()
            .map(|(json,)| {
                serde_json::from_str(&json)
                    .map_err(|e| SentinelError::StorageError(e.to_string()))
            })
            .collect()
    }
}
