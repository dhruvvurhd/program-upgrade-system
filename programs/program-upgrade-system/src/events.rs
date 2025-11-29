use anchor_lang::prelude::*;

#[event]
pub struct ProposalCreatedEvent {
    pub proposal_id: Pubkey,
    pub proposer: Pubkey,
    pub new_program_buffer: Pubkey,
    pub description: String,
    pub timelock_end: i64,
    pub timestamp: i64,
}

#[event]
pub struct ApprovalEvent {
    pub proposal_id: Pubkey,
    pub approver: Pubkey,
    pub approval_count: u8,
    pub threshold: u8,
    pub timelock_activated: bool,
    pub timestamp: i64,
}

#[event]
pub struct UpgradeExecutedEvent {
    pub proposal_id: Pubkey,
    pub program_id: Pubkey,
    pub executor: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct UpgradeCancelledEvent {
    pub proposal_id: Pubkey,
    pub canceller: Pubkey,
    pub reason: String,
    pub timestamp: i64,
}

#[event]
pub struct AccountMigratedEvent {
    pub account: Pubkey,
    pub old_version: u8,
    pub new_version: u8,
    pub timestamp: i64,
}

#[event]
pub struct TimelockActivatedEvent {
    pub proposal_id: Pubkey,
    pub activated_at: i64,
    pub expires_at: i64,
}
