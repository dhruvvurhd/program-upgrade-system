use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::events::*;
use crate::constants::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct ApproveUpgrade<'info> {
    #[account(
        mut,
        constraint = proposal.status == UpgradeStatus::Proposed 
            || proposal.status == UpgradeStatus::Approved 
            @ ErrorCode::InvalidProposalState,
    )]
    pub proposal: Account<'info, UpgradeProposal>,
    
    #[account(
        seeds = [SEED_MULTISIG],
        bump = multisig_config.bump,
    )]
    pub multisig_config: Account<'info, MultisigConfig>,
    
    pub approver: Signer<'info>,
}

pub fn handler(
    ctx: Context<ApproveUpgrade>,
    _proposal_id: Pubkey,
) -> Result<()> {
    validate_multisig_member(&ctx.accounts.multisig_config.members, &ctx.accounts.approver.key())?;
    
    let proposal = &mut ctx.accounts.proposal;
    
    // Check for duplicate approval
    require!(
        !proposal.approvals.contains(&ctx.accounts.approver.key()),
        ErrorCode::DuplicateApproval
    );
    
    // Add approval
    proposal.approvals.push(ctx.accounts.approver.key());
    proposal.approval_count += 1;
    
    let clock = Clock::get()?;
    let threshold_met = validate_threshold(
        proposal.approval_count, 
        ctx.accounts.multisig_config.threshold
    )?;
    
    let mut timelock_activated = false;
    
    // If threshold met and timelock not yet activated, activate it
    if threshold_met && proposal.timelock_activated_at.is_none() {
        proposal.status = UpgradeStatus::TimelockActive;
        proposal.timelock_activated_at = Some(clock.unix_timestamp);
        timelock_activated = true;
        
        emit!(TimelockActivatedEvent {
            proposal_id: proposal.id,
            activated_at: clock.unix_timestamp,
            expires_at: clock.unix_timestamp + proposal.timelock_period,
        });
    } else if threshold_met {
        proposal.status = UpgradeStatus::Approved;
    }
    
    emit!(ApprovalEvent {
        proposal_id: proposal.id,
        approver: ctx.accounts.approver.key(),
        approval_count: proposal.approval_count,
        threshold: ctx.accounts.multisig_config.threshold,
        timelock_activated,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}
