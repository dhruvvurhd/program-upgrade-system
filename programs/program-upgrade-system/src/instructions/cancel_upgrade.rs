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
pub struct CancelUpgrade<'info> {
    #[account(
        mut,
        constraint = proposal.status != UpgradeStatus::Executed @ ErrorCode::CannotCancelAfterExecution,
        constraint = proposal.status != UpgradeStatus::Cancelled @ ErrorCode::ProposalAlreadyCancelled,
    )]
    pub proposal: Box<Account<'info, UpgradeProposal>>,
    
    #[account(
        seeds = [SEED_MULTISIG],
        bump = multisig_config.bump,
    )]
    pub multisig_config: Box<Account<'info, MultisigConfig>>,
    
    #[account(mut)]
    pub canceller: Signer<'info>,
    
    /// CHECK: Buffer account to close and refund. Must be owned by BPF Loader.
    #[account(mut)]
    pub buffer: UncheckedAccount<'info>,
    
    /// CHECK: Rent recipient for buffer close
    #[account(mut)]
    pub rent_recipient: UncheckedAccount<'info>,
    
    /// CHECK: BPF Loader Upgradeable Program
    pub bpf_loader_upgradeable: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<CancelUpgrade>,
    _proposal_id: Pubkey,
) -> Result<()> {
    validate_multisig_member(&ctx.accounts.multisig_config.members, &ctx.accounts.canceller.key())?;
    
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;
    
    // Verify buffer matches proposal
    require!(
        ctx.accounts.buffer.key() == proposal.new_program_buffer,
        ErrorCode::InvalidProgramBuffer
    );
    
    // Verify buffer is owned by BPF Loader (might already be closed)
    let bpf_loader_id = bpf_loader_upgradeable::id();
    let buffer_owned_by_loader = ctx.accounts.buffer.owner == &bpf_loader_id;
    
    // Close buffer account via CPI if it's still owned by loader
    if buffer_owned_by_loader && !ctx.accounts.buffer.data_is_empty() {
        // Build close instruction
        // The multisig PDA must be the buffer authority for this to work
        let close_instruction = bpf_loader_upgradeable::close_any(
            &ctx.accounts.buffer.key(),
            &ctx.accounts.rent_recipient.key(),
            Some(&ctx.accounts.multisig_config.key()),
            None, // No program to close
        );
        
        let multisig_seeds = &[
            SEED_MULTISIG,
            &[ctx.accounts.multisig_config.bump],
        ];
        
        // Invoke close with multisig PDA as signer
        invoke_signed(
            &close_instruction,
            &[
                ctx.accounts.buffer.to_account_info(),
                ctx.accounts.rent_recipient.to_account_info(),
                ctx.accounts.multisig_config.to_account_info(),
            ],
            &[multisig_seeds],
        )?;
        
        msg!("Buffer account closed, rent refunded to {}", ctx.accounts.rent_recipient.key());
    } else {
        msg!("Buffer already closed or not owned by loader, skipping close");
    }
    
    // Update proposal state
    proposal.status = UpgradeStatus::Cancelled;
    
    emit!(UpgradeCancelledEvent {
        proposal_id: proposal.id,
        canceller: ctx.accounts.canceller.key(),
        reason: "Cancelled by multisig member".to_string(),
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}
