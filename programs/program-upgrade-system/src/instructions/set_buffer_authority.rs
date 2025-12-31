use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    bpf_loader_upgradeable,
    program::invoke,
};
use crate::state::MultisigConfig;
use crate::constants::SEED_MULTISIG;
use crate::error::ErrorCode;
use crate::utils::validate_multisig_member;

#[derive(Accounts)]
pub struct SetBufferAuthority<'info> {
    #[account(
        seeds = [SEED_MULTISIG],
        bump = multisig_config.bump,
    )]
    pub multisig_config: Box<Account<'info, MultisigConfig>>,
    
    /// CHECK: Buffer account whose authority is being transferred
    #[account(mut)]
    pub buffer: UncheckedAccount<'info>,
    
    /// Current buffer authority - must be a signer
    #[account(mut)]
    pub current_authority: Signer<'info>,
    
    /// CHECK: BPF Loader Upgradeable Program
    pub bpf_loader_upgradeable: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<SetBufferAuthority>) -> Result<()> {
    // Verify the signer is a multisig member
    validate_multisig_member(&ctx.accounts.multisig_config.members, &ctx.accounts.current_authority.key())?;
    
    // Verify buffer is owned by BPF Loader Upgradeable
    let bpf_loader_id = bpf_loader_upgradeable::id();
    require!(
        ctx.accounts.buffer.owner == &bpf_loader_id,
        ErrorCode::InvalidBufferOwner
    );
    
    // Create the set_buffer_authority instruction
    // This transfers the buffer authority from current_authority to multisig_config PDA
    let set_authority_ix = bpf_loader_upgradeable::set_buffer_authority(
        &ctx.accounts.buffer.key(),
        &ctx.accounts.current_authority.key(),
        &ctx.accounts.multisig_config.key(),
    );
    
    // Invoke the instruction (current_authority signs as external signer)
    invoke(
        &set_authority_ix,
        &[
            ctx.accounts.buffer.to_account_info(),
            ctx.accounts.current_authority.to_account_info(),
            ctx.accounts.multisig_config.to_account_info(),
        ],
    )?;
    
    msg!("Buffer authority transferred to multisig PDA: {}", ctx.accounts.multisig_config.key());
    
    Ok(())
}
