use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized signer - not a multisig member")]
    UnauthorizedSigner,
    
    #[msg("Insufficient approvals - threshold not met")]
    InsufficientApprovals,
    
    #[msg("Timelock not expired - must wait 48 hours")]
    TimelockNotExpired,
    
    #[msg("Invalid proposal state")]
    InvalidProposalState,
    
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    
    #[msg("Proposal already cancelled")]
    ProposalAlreadyCancelled,
    
    #[msg("Invalid program buffer")]
    InvalidProgramBuffer,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Description too long")]
    DescriptionTooLong,
    
    #[msg("Invalid multisig threshold")]
    InvalidThreshold,
    
    #[msg("Too many members")]
    TooManyMembers,
    
    #[msg("Duplicate approval")]
    DuplicateApproval,
    
    #[msg("Account already migrated")]
    AccountAlreadyMigrated,
    
    #[msg("Invalid account version")]
    InvalidAccountVersion,
    
    #[msg("Migration failed")]
    MigrationFailed,
    
    #[msg("Cannot cancel after execution")]
    CannotCancelAfterExecution,
    
    #[msg("Timelock already activated")]
    TimelockAlreadyActivated,
}
