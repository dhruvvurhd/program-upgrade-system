use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::events::*;
use crate::constants::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(new_program_buffer: Pubkey, description: String)]
pub struct ProposeUpgrade<'info> {
    #[account(
        init,
        payer = proposer,
        space = UpgradeProposal::LEN,
        seeds = [SEED_PROPOSAL, new_program_buffer.as_ref()],
        bump
    )]
    pub proposal: Box<Account<'info, UpgradeProposal>>,
    
    #[account(
        seeds = [SEED_MULTISIG],
        bump = multisig_config.bump,
    )]
    pub multisig_config: Box<Account<'info, MultisigConfig>>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ProposeUpgrade>,
    new_program_buffer: Pubkey,
    description: String,
) -> Result<()> {
    validate_description_length(&description, MAX_DESCRIPTION_LENGTH)?;
    validate_multisig_member(&ctx.accounts.multisig_config.members, &ctx.accounts.proposer.key())?;
    
    let clock = Clock::get()?;
    let proposal = &mut ctx.accounts.proposal;
    
    proposal.id = proposal.key();
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.new_program_buffer = new_program_buffer;
    proposal.target_program = crate::ID;
    proposal.description = description.clone();
    proposal.status = UpgradeStatus::Proposed;
    proposal.approvals = vec![];
    proposal.approval_count = 0;
    proposal.created_at = clock.unix_timestamp;
    proposal.timelock_activated_at = None;
    proposal.timelock_period = TIMELOCK_PERIOD;
    proposal.executed_at = None;
    proposal.bump = ctx.bumps.proposal;
    
    emit!(ProposalCreatedEvent {
        proposal_id: proposal.id,
        proposer: proposal.proposer,
        new_program_buffer,
        description,
        timelock_end: clock.unix_timestamp + TIMELOCK_PERIOD,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}
