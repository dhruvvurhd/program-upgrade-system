use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationJob {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub total_accounts: i64,
    pub migrated_accounts: i64,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartMigrationRequest {
    pub proposal_id: String,
    pub account_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationProgress {
    pub job_id: Uuid,
    pub total: i64,
    pub completed: i64,
    pub percentage: f64,
    pub status: String,
}
