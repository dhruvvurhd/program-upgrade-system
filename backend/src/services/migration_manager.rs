use sqlx::PgPool;
use anyhow::{Result, bail};
use uuid::Uuid;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Manages account migration tracking.
/// 
/// IMPORTANT: This manager tracks migration progress in the database.
/// Actual account data transformation is PROTOCOL-SPECIFIC and intentionally
/// not implemented here. Each protocol must implement its own migration logic
/// based on its account schema changes.
/// 
/// The on-chain `migrate_account` instruction creates a version tracking record
/// but does NOT perform data transformation. Data transformation must be handled
/// by protocol-specific migration scripts.
pub struct MigrationManager {
    db_pool: PgPool,
}

impl MigrationManager {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    
    /// Start migration tracking job
    /// 
    /// This creates database records to track migration progress.
    /// Actual on-chain migration must be performed separately.
    pub async fn start_migration(
        &self,
        proposal_id: Uuid,
        account_addresses: Vec<String>,
    ) -> Result<Uuid> {
        let job_id = Uuid::new_v4();
        let total = account_addresses.len() as i64;
        
        sqlx::query!(
            r#"
            INSERT INTO migration_jobs (id, proposal_id, total_accounts, migrated_accounts)
            VALUES ($1, $2, $3, 0)
            "#,
            job_id,
            proposal_id,
            total
        )
        .execute(&self.db_pool)
        .await?;
        
        tracing::info!("Created migration tracking job {} for {} accounts", job_id, total);
        tracing::warn!("NOTE: Actual on-chain migration must be performed separately using protocol-specific scripts");
        
        Ok(job_id)
    }
    
    /// Record that an account was migrated on-chain
    /// 
    /// Call this AFTER successfully executing the on-chain migrate_account instruction.
    /// This does NOT perform the migration itself.
    pub async fn record_migration_complete(
        &self,
        job_id: Uuid,
        account: &str,
        tx_signature: &str,
    ) -> Result<()> {
        // Verify the account address is valid
        Pubkey::from_str(account)?;
        
        sqlx::query!(
            r#"
            INSERT INTO account_migrations (id, migration_job_id, account_address, old_version, new_version, status, tx_signature)
            VALUES ($1, $2, $3, 1, 2, 'success', $4)
            "#,
            Uuid::new_v4(),
            job_id,
            account,
            tx_signature
        )
        .execute(&self.db_pool)
        .await?;
        
        // Update job progress
        sqlx::query!(
            r#"
            UPDATE migration_jobs
            SET migrated_accounts = migrated_accounts + 1,
                updated_at = NOW()
            WHERE id = $1
            "#,
            job_id
        )
        .execute(&self.db_pool)
        .await?;
        
        Ok(())
    }
    
    /// Record migration failure
    pub async fn record_migration_failure(
        &self,
        job_id: Uuid,
        account: &str,
        error: &str,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO account_migrations (id, migration_job_id, account_address, old_version, new_version, status, error_message)
            VALUES ($1, $2, $3, 1, 2, 'failed', $4)
            "#,
            Uuid::new_v4(),
            job_id,
            account,
            error
        )
        .execute(&self.db_pool)
        .await?;
        Ok(())
    }
    
    /// Get migration progress
    pub async fn get_progress(&self, job_id: Uuid) -> Result<(i64, i64)> {
        let result = sqlx::query!(
            r#"
            SELECT total_accounts, migrated_accounts
            FROM migration_jobs
            WHERE id = $1
            "#,
            job_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        Ok((result.total_accounts, result.migrated_accounts))
    }
}

// NOTE: The previous implementation had a `run_migration` method that returned
// Ok(()) without actually calling any on-chain instructions. This has been removed.
// 
// Migration is intentionally NOT automated because:
// 1. Account data transformation is protocol-specific
// 2. Each protocol has different account schemas
// 3. Migration may require human review of each account
// 4. Batch migration needs rate limiting and error handling specific to the protocol
//
// For production use:
// 1. Call `start_migration` to create a tracking job
// 2. Use protocol-specific scripts to migrate accounts on-chain
// 3. Call `record_migration_complete` after each successful on-chain migration
