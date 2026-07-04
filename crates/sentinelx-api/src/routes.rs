use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sentinelx_core::{AnalysisReport, PipelineContext, ReportFormat};
use sentinelx_reporting::DefaultReportGenerator;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct AnalyzeRequest {
    pub file_path: String,
    #[serde(default)]
    pub static_only: bool,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct AnalyzeResponse {
    pub session_id: Uuid,
    pub decision: String,
    pub risk_score: f64,
    pub risk_level: String,
    pub report_id: Uuid,
}

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        .route("/analyze", post(analyze))
        .route("/sessions", get(list_sessions))
        .route("/sessions/{id}", get(get_session))
        .route("/sessions/{id}/report", get(get_report))
        .route("/sessions/{id}/report/export", get(export_report))
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "sentinelx-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn metrics() -> impl IntoResponse {
    Json(serde_json::json!({
        "analyses_total": "placeholder",
        "uptime_seconds": "placeholder",
    }))
}

#[utoipa::path(
    post,
    path = "/analyze",
    request_body = AnalyzeRequest,
    responses((status = 200, description = "Analysis started", body = AnalyzeResponse))
)]
async fn analyze(
    State(state): State<AppState>,
    Json(req): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, ApiError> {
    let path = PathBuf::from(&req.file_path);
    if !path.exists() {
        return Err(ApiError::NotFound(format!("file not found: {}", req.file_path)));
    }

    let pipeline = if req.static_only {
        sentinelx_agent::AnalysisPipeline::new()
            .with_store(state.store.clone())
            .skip_sandbox(true)
    } else {
        state.pipeline.clone_for_request()
    };

    let (ctx, report) = pipeline
        .analyze_file(&path)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let decision = ctx
        .decision
        .as_ref()
        .map(|d| format!("{:?}", d.action))
        .unwrap_or_else(|| "Unknown".into());

    Ok(Json(AnalyzeResponse {
        session_id: ctx.session_id(),
        decision,
        risk_score: ctx.risk.as_ref().map(|r| r.score).unwrap_or(0.0),
        risk_level: ctx
            .risk
            .as_ref()
            .map(|r| r.level.as_str().into())
            .unwrap_or_else(|| "unknown".into()),
        report_id: report.id,
    }))
}

async fn list_sessions(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<SessionSummary>>, ApiError> {
    let sessions = state
        .store
        .list_sessions(q.limit)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(
        sessions
            .into_iter()
            .map(|s| SessionSummary {
                id: s.session_id(),
                file_path: s.session.file.path.to_string_lossy().to_string(),
                status: format!("{:?}", s.session.status),
                risk_score: s.risk.as_ref().map(|r| r.score),
            })
            .collect(),
    ))
}

async fn get_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PipelineContext>, ApiError> {
    state
        .store
        .load_session(id)
        .await
        .map(Json)
        .map_err(|_| ApiError::NotFound(id.to_string()))
}

async fn get_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisReport>, ApiError> {
    let ctx = state
        .store
        .load_session(id)
        .await
        .map_err(|_| ApiError::NotFound(id.to_string()))?;
    let report = DefaultReportGenerator
        .generate(&ctx)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(report))
}

#[derive(Deserialize)]
pub struct ExportQuery {
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_format() -> String {
    "json".into()
}

async fn export_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<ExportQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let ctx = state
        .store
        .load_session(id)
        .await
        .map_err(|_| ApiError::NotFound(id.to_string()))?;
    let report = DefaultReportGenerator
        .generate(&ctx)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let format = match q.format.as_str() {
        "html" => ReportFormat::Html,
        "pdf" => ReportFormat::Pdf,
        _ => ReportFormat::Json,
    };

    let bytes = DefaultReportGenerator
        .export(&report, format)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(bytes)
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct SessionSummary {
    pub id: Uuid,
    pub file_path: String,
    pub status: String,
    pub risk_score: Option<f64>,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            ApiError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}
