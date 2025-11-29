use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::events::*;
use crate::constants::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct CancelUpgrade<'info> {
    #[account(
        mut,
        constraint = proposal.status != UpgradeStatus::Executed @ ErrorCode::CannotCancelAfterExecution,
        constraint = proposal.status != UpgradeStatus::Cancelled @ ErrorCode::ProposalAlreadyCancelled,
    )]
    pub proposal: Account<'info, UpgradeProposal>,
    
    #[account(
        seeds = [SEED_MULTISIG],
        bump = multisig_config.bump,
    )]
    pub multisig_config: Account<'info, MultisigConfig>,
    
    #[account(mut)]
    pub canceller: Signer<'info>,
    
    /// CHECK: Buffer account to close and refund
    #[account(mut)]
    pub buffer: UncheckedAccount<'info>,
    
    /// CHECK: Rent recipient
    #[account(mut)]
    pub rent_recipient: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<CancelUpgrade>,
    _proposal_id: Pubkey,
) -> Result<()> {
    validate_multisig_member(&ctx.accounts.multisig_config.members, &ctx.accounts.canceller.key())?;
    
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;
    
    // Update proposal state
    proposal.status = UpgradeStatus::Cancelled;
    
    // Close buffer account and refund rent
    // In production, you would invoke close buffer instruction via CPI
    
    emit!(UpgradeCancelledEvent {
        proposal_id: proposal.id,
        canceller: ctx.accounts.canceller.key(),
        reason: "Cancelled by multisig".to_string(),
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}
