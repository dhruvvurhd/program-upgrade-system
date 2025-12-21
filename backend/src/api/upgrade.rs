use axum::{
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use uuid::Uuid;
use std::sync::Arc;
use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;
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
/// NOTE: This endpoint creates a database record only.
/// The on-chain proposal must be created separately using the Anchor program.
pub async fn propose_upgrade(
    State(services): State<Arc<Services>>,
    Json(request): Json<ProposeRequest>,
) -> Result<Json<Value>, StatusCode> {
    let proposal_id = Uuid::new_v4();
    
    // Store in database (on-chain proposal is handled separately)
    sqlx::query!(
        r#"
        INSERT INTO upgrade_proposals
        (id, proposer, program, new_buffer, description, status, approval_count)
        VALUES ($1, $2, $3, $4, $5, 'Proposed', 0)
        "#,
        proposal_id,
        "system",
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
        "status": "created",
        "note": "Database record created. Submit on-chain proposal via Anchor CLI."
    })))
}

/// Approve upgrade proposal
/// NOTE: Records approval in database. On-chain approval handled separately.
pub async fn approve_upgrade(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
    Json(_request): Json<ApproveRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Record approval in database
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
        // Activate timelock in database
        services.timelock_manager
            .set_timelock(id, 48)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    Ok(Json(json!({
        "proposal_id": id,
        "status": "approval_recorded",
        "threshold_met": threshold_met,
        "note": "Database updated. Confirm on-chain approval separately."
    })))
}

/// Execute upgrade after timelock
/// THIS IS THE PRODUCTION-REALISTIC PATH - Makes real on-chain transaction
pub async fn execute_upgrade(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
    Json(request): Json<ExecuteRequest>,
) -> Result<Json<Value>, StatusCode> {
    tracing::info!("Executing upgrade for proposal {}", id);
    
    // Fetch proposal from database
    let proposal = sqlx::query!(
        r#"
        SELECT id, new_buffer, program, status, timelock_until
        FROM upgrade_proposals
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&services.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Proposal not found: {}", e);
        StatusCode::NOT_FOUND
    })?;
    
    // Verify status
    if proposal.status != "TimelockActive" {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Verify timelock expired (database check - on-chain also enforces)
    if let Some(timelock_until) = proposal.timelock_until {
        if timelock_until > chrono::Utc::now() {
            tracing::warn!("Timelock not yet expired");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
    
    // Parse pubkeys
    let buffer = Pubkey::from_str(&proposal.new_buffer).map_err(|e| {
        tracing::error!("Invalid buffer pubkey: {}", e);
        StatusCode::BAD_REQUEST
    })?;
    
    let program_to_upgrade = Pubkey::from_str(&request.program_id).map_err(|e| {
        tracing::error!("Invalid program pubkey: {}", e);
        StatusCode::BAD_REQUEST
    })?;
    
    // Derive proposal PDA
    let (proposal_pda, _) = Pubkey::find_program_address(
        &[b"proposal", buffer.as_ref()],
        &services.anchor_client.program_id,
    );
    
    // Execute real on-chain transaction
    let tx_result = services.anchor_client
        .execute_upgrade(proposal_pda, buffer, program_to_upgrade)
        .await
        .map_err(|e| {
            tracing::error!("On-chain execute_upgrade failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // If transaction failed, return error
    if !tx_result.success {
        tracing::error!("Transaction failed: {:?}", tx_result.error_message);
        
        // Record failure in database
        sqlx::query!(
            r#"
            UPDATE upgrade_proposals
            SET status = 'Failed',
                tx_signature = $2,
                tx_slot = $3,
                error_message = $4,
                updated_at = NOW()
            WHERE id = $1
            "#,
            id,
            tx_result.signature,
            tx_result.slot as i64,
            tx_result.error_message
        )
        .execute(&services.db_pool)
        .await
        .ok(); // Best effort logging
        
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    // Success - update database
    sqlx::query!(
        r#"
        UPDATE upgrade_proposals
        SET status = 'Executed',
            executed_at = NOW(),
            tx_signature = $2,
            tx_slot = $3,
            updated_at = NOW()
        WHERE id = $1
        "#,
        id,
        tx_result.signature,
        tx_result.slot as i64
    )
    .execute(&services.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update proposal status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    tracing::info!("Upgrade executed successfully: {}", tx_result.signature);
    
    Ok(Json(json!({
        "proposal_id": id,
        "status": "executed",
        "tx_signature": tx_result.signature,
        "tx_slot": tx_result.slot,
        "success": true
    })))
}

/// Cancel upgrade proposal
/// NOTE: Does NOT close the buffer account. Buffer management is external.
pub async fn cancel_upgrade(
    State(services): State<Arc<Services>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    // Update database
    sqlx::query!(
        r#"
        UPDATE upgrade_proposals
        SET status = 'Cancelled',
            updated_at = NOW()
        WHERE id = $1
        "#,
        id
    )
    .execute(&services.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "proposal_id": id,
        "status": "cancelled",
        "note": "Database updated. Close buffer account using Solana CLI: solana program close <buffer>"
    })))
}
