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

/// List all upgrade proposals
pub async fn list_proposals(
    State(services): State<Arc<Services>>,
) -> Result<Json<Value>, StatusCode> {
    let proposals = sqlx::query_as!(
        Proposal,
        r#"
        SELECT id, proposer, program, new_buffer, description, status,
               approval_count, proposed_at, timelock_until, executed_at
        FROM upgrade_proposals
        ORDER BY proposed_at DESC
        "#
    )
    .fetch_all(&services.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch proposals: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(json!({ "proposals": proposals })))
}

/// Get single proposal
pub async fn get_proposal(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Proposal>, StatusCode> {
    let proposal = sqlx::query_as!(
        Proposal,
        r#"
        SELECT id, proposer, program, new_buffer, description, status,
               approval_count, proposed_at, timelock_until, executed_at
        FROM upgrade_proposals
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&services.db_pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(proposal))
}

/// Create new upgrade proposal
pub async fn propose_upgrade(
    State(services): State<Arc<Services>>,
    Json(request): Json<ProposeRequest>,
) -> Result<Json<Value>, StatusCode> {
    let proposal_id = Uuid::new_v4();
    
    // Call on-chain program
    // let tx_sig = services.anchor_client
    //     .propose_upgrade(...)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Store in database
    sqlx::query!(
        r#"
        INSERT INTO upgrade_proposals
        (id, proposer, program, new_buffer, description, status, approval_count)
        VALUES ($1, $2, $3, $4, $5, 'Proposed', 0)
        "#,
        proposal_id,
        "system", // Would be actual proposer
        "program_id",
        request.new_program_buffer,
        request.description
    )
    .execute(&services.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to store proposal: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    Ok(Json(json!({
        "proposal_id": proposal_id,
        "status": "created"
    })))
}

/// Approve upgrade proposal
pub async fn approve_upgrade(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
    Json(request): Json<ApproveRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Call on-chain program
    // Load approver keypair
    // Call approve_upgrade instruction
    
    // Record approval
    services.multisig_coordinator
        .record_approval(id, "approver_pubkey".to_string())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Check if threshold met
    let threshold_met = services.multisig_coordinator
        .check_threshold(id, 3)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if threshold_met {
        // Activate timelock
        services.timelock_manager
            .set_timelock(id, 48)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok(Json(json!({
        "proposal_id": id,
        "status": "approved",
        "threshold_met": threshold_met
    })))
}

/// Execute upgrade after timelock
pub async fn execute_upgrade(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
    Json(request): Json<ExecuteRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Verify timelock expired
    // Call on-chain execute_upgrade instruction
    
    // Update database
    sqlx::query!(
        r#"
        UPDATE upgrade_proposals
        SET status = 'Executed',
            executed_at = NOW()
        WHERE id = $1
        "#,
        id
    )
    .execute(&services.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "proposal_id": id,
        "status": "executed"
    })))
}

/// Cancel upgrade proposal
pub async fn cancel_upgrade(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    // Call on-chain cancel_upgrade instruction
    
    sqlx::query!(
        r#"
        UPDATE upgrade_proposals
        SET status = 'Cancelled'
        WHERE id = $1
        "#,
        id
    )
    .execute(&services.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "proposal_id": id,
        "status": "cancelled"
    })))
}
