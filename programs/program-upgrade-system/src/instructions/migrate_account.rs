use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::events::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(old_account_key: Pubkey)]
pub struct MigrateAccount<'info> {
    #[account(
        init,
        payer = migrator,
        space = AccountVersion::LEN,
        seeds = [SEED_MIGRATION, old_account_key.as_ref()],
        bump
    )]
    pub account_version: Account<'info, AccountVersion>,
    
    /// CHECK: The account to migrate
    #[account(mut)]
    pub old_account: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub migrator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<MigrateAccount>,
    old_account_key: Pubkey,
) -> Result<()> {
    let account_version = &mut ctx.accounts.account_version;
    let old_account = &ctx.accounts.old_account;
    
    // Verify old_account_key matches
    require!(
        old_account.key() == old_account_key,
        ErrorCode::InvalidAccountVersion
    );
    
    let clock = Clock::get()?;
    
    // Read old account data
    let old_data = old_account.try_borrow_data()?;
    
    // Create a simple hash by taking first 32 bytes or padding with zeros
    let mut old_data_hash = [0u8; 32];
    let copy_len = old_data.len().min(32);
    old_data_hash[..copy_len].copy_from_slice(&old_data[..copy_len]);
    
    // In a real implementation, you would:
    // 1. Deserialize old data structure
    // 2. Transform to new data structure
    // 3. Realloc account if needed
    // 4. Serialize new data back
    
    // For this example, we just track the migration
    account_version.account = old_account.key();
    account_version.version = 2; // New version
    account_version.migrated = true;
    account_version.migrated_at = Some(clock.unix_timestamp);
    account_version.old_data_hash = old_data_hash;
    account_version.new_data_hash = old_data_hash; // Would be different after real migration
    
    emit!(AccountMigratedEvent {
        account: old_account.key(),
        old_version: 1,
        new_version: 2,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}
