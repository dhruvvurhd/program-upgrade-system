use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    bpf_loader_upgradeable,
    program::invoke_signed,
};
use crate::state::*;
use crate::error::ErrorCode;
use crate::events::*;
use crate::constants::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct ExecuteUpgrade<'info> {
    #[account(
        mut,
        constraint = proposal.status == UpgradeStatus::TimelockActive @ ErrorCode::InvalidProposalState,
    )]
    pub proposal: Account<'info, UpgradeProposal>,
    
    #[account(
        seeds = [SEED_MULTISIG],
        bump = multisig_config.bump,
    )]
    pub multisig_config: Account<'info, MultisigConfig>,
    
    /// CHECK: This is the program to upgrade
    #[account(mut)]
    pub program_to_upgrade: UncheckedAccount<'info>,
    
    /// CHECK: This is the program data account
    #[account(mut)]
    pub program_data: UncheckedAccount<'info>,
    
    /// CHECK: This is the buffer account with new program
    #[account(mut)]
    pub buffer: UncheckedAccount<'info>,
    
    /// CHECK: Spill account for rent
    #[account(mut)]
    pub spill_account: UncheckedAccount<'info>,
    
    pub executor: Signer<'info>,
    
    /// CHECK: BPF Loader Upgradeable Program
    pub bpf_loader_upgradeable: UncheckedAccount<'info>,
    
    pub rent: Sysvar<'info, Rent>,
    
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(
    ctx: Context<ExecuteUpgrade>,
    _proposal_id: Pubkey,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    
    // Verify timelock expired
    let timelock_activated = proposal.timelock_activated_at
        .ok_or(ErrorCode::InvalidProposalState)?;
    
    validate_timelock_expired(timelock_activated, proposal.timelock_period)?;
    
    // Verify threshold met
    require!(
        validate_threshold(proposal.approval_count, ctx.accounts.multisig_config.threshold)?,
        ErrorCode::InsufficientApprovals
    );
    
    // Verify buffer matches proposal
    require!(
        ctx.accounts.buffer.key() == proposal.new_program_buffer,
        ErrorCode::InvalidProgramBuffer
    );
    
    let clock = Clock::get()?;
    
    // Execute upgrade via CPI to BPF Loader Upgradeable
    // NOTE: In production, this requires proper authority setup
    // The multisig PDA should be the upgrade authority
    let upgrade_instruction = bpf_loader_upgradeable::upgrade(
        &ctx.accounts.program_to_upgrade.key(),
        &ctx.accounts.buffer.key(),
        &ctx.accounts.multisig_config.key(),
        &ctx.accounts.spill_account.key(),
    );
    
    let multisig_seeds = &[
        SEED_MULTISIG,
        &[ctx.accounts.multisig_config.bump],
    ];
    
    invoke_signed(
        &upgrade_instruction,
        &[
            ctx.accounts.program_data.to_account_info(),
            ctx.accounts.program_to_upgrade.to_account_info(),
            ctx.accounts.buffer.to_account_info(),
            ctx.accounts.spill_account.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.clock.to_account_info(),
            ctx.accounts.multisig_config.to_account_info(),
        ],
        &[multisig_seeds],
    )?;
    
    // Update proposal state
    proposal.status = UpgradeStatus::Executed;
    proposal.executed_at = Some(clock.unix_timestamp);
    
    emit!(UpgradeExecutedEvent {
        proposal_id: proposal.id,
        program_id: ctx.accounts.program_to_upgrade.key(),
        executor: ctx.accounts.executor.key(),
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}
