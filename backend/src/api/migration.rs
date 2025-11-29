use axum::{
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use uuid::Uuid;
use std::sync::Arc;
use crate::services::Services;
use crate::models::*;

/// Start migration for a proposal
pub async fn start_migration(
    State(services): State<Arc<Services>>,
    Json(request): Json<StartMigrationRequest>,
) -> Result<Json<Value>, StatusCode> {
    let proposal_id = Uuid::parse_str(&request.proposal_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let job_id = services.migration_manager
        .start_migration(proposal_id, request.account_addresses)
        .await
        .map_err(|e| {
            tracing::error!("Failed to start migration: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(json!({
        "job_id": job_id,
        "status": "started"
    })))
}

/// Get migration progress
pub async fn get_progress(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
) -> Result<Json<MigrationProgress>, StatusCode> {
    let (total, completed) = services.migration_manager
        .get_progress(id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let percentage = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    let status = if completed >= total {
        "completed"
    } else {
        "in_progress"
    };
    
    Ok(Json(MigrationProgress {
        job_id: id,
        total,
        completed,
        percentage,
        status: status.to_string(),
    }))
}
