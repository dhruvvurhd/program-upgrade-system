use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Manages account migration batches
pub struct MigrationManager {
    db_pool: PgPool,
}

impl MigrationManager {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    
    /// Start migration job
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
        
        tracing::info!("Started migration job {} for {} accounts", job_id, total);
        
        // Spawn background task for migration
        let pool = self.db_pool.clone();
        tokio::spawn(async move {
            if let Err(e) = Self::run_migration(pool, job_id, account_addresses).await {
                tracing::error!("Migration job {} failed: {}", job_id, e);
            }
        });
        
        Ok(job_id)
    }
    
    /// Run migration in background
    async fn run_migration(
        pool: PgPool,
        job_id: Uuid,
        accounts: Vec<String>,
    ) -> Result<()> {
        for (idx, account_str) in accounts.iter().enumerate() {
            let account = Pubkey::from_str(account_str)?;
            
            // Migrate account (call on-chain instruction)
            match Self::migrate_single_account(account).await {
                Ok(_) => {
                    Self::record_migration_success(&pool, job_id, account_str).await?;
                }
                Err(e) => {
                    Self::record_migration_failure(&pool, job_id, account_str, &e.to_string()).await?;
                }
            }
            
            // Update progress
            sqlx::query!(
                r#"
                UPDATE migration_jobs
                SET migrated_accounts = $1,
                    updated_at = NOW()
                WHERE id = $2
                "#,
                (idx + 1) as i64,
                job_id
            )
            .execute(&pool)
            .await?;
            
            // Rate limiting
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        
        // Mark job as finished
        sqlx::query!(
            r#"
            UPDATE migration_jobs
            SET finished_at = NOW()
            WHERE id = $1
            "#,
            job_id
        )
        .execute(&pool)
        .await?;
        
        tracing::info!("Migration job {} completed", job_id);
        
        Ok(())
    }
    
    /// Migrate single account
    async fn migrate_single_account(account: Pubkey) -> Result<()> {
        // Call on-chain migrate_account instruction
        tracing::debug!("Migrating account {}", account);
        Ok(())
    }
    
    /// Record successful migration
    async fn record_migration_success(
        pool: &PgPool,
        job_id: Uuid,
        account: &str,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO account_migrations (id, migration_job_id, account_address, old_version, new_version, status)
            VALUES ($1, $2, $3, 1, 2, 'success')
            "#,
            Uuid::new_v4(),
            job_id,
            account
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    
    /// Record failed migration
    async fn record_migration_failure(
        pool: &PgPool,
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
        .execute(pool)
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
