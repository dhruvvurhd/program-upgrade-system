use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeStatus {
    Proposed,
    Approved,
    TimelockActive,
    Executed,
    Cancelled,
}

#[account]
pub struct MultisigConfig {
    pub authority: Pubkey,
    pub members: Vec<Pubkey>,
    pub threshold: u8,
    pub is_paused: bool,
    pub bump: u8,
}

impl MultisigConfig {
    pub const LEN: usize = 8 + 32 + 4 + (32 * 10) + 1 + 1 + 1; // discriminator + authority + vec_len + members + threshold + is_paused + bump
}

#[account]
pub struct UpgradeProposal {
    pub id: Pubkey,
    pub proposer: Pubkey,
    pub new_program_buffer: Pubkey,
    pub target_program: Pubkey,
    pub description: String,
    pub status: UpgradeStatus,
    pub approvals: Vec<Pubkey>,
    pub approval_count: u8,
    pub created_at: i64,
    pub timelock_activated_at: Option<i64>,
    pub timelock_period: i64,
    pub executed_at: Option<i64>,
    pub bump: u8,
}

impl UpgradeProposal {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 4 + 500 + 1 + 4 + (32 * 10) + 1 + 8 + 9 + 8 + 9 + 1;
}

#[account]
pub struct AccountVersion {
    pub account: Pubkey,
    pub version: u8,
    pub migrated: bool,
    pub migrated_at: Option<i64>,
    pub old_data_hash: [u8; 32],
    pub new_data_hash: [u8; 32],
}

impl AccountVersion {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 9 + 32 + 32;
}

#[account]
pub struct MigrationTracker {
    pub proposal_id: Pubkey,
    pub total_accounts: u64,
    pub migrated_accounts: u64,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub bump: u8,
}

impl MigrationTracker {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8 + 9 + 1;
}
