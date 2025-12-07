use anchor_lang::prelude::*;
use crate::state::MultisigConfig;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct PauseSystem<'info> {
    #[account(
        mut,
        seeds = [b"multisig"],
        bump = multisig_config.bump,
        constraint = multisig_config.members.contains(&pauser.key()) @ ErrorCode::NotAMember,
        constraint = !multisig_config.is_paused @ ErrorCode::SystemAlreadyPaused,
    )]
    pub multisig_config: Account<'info, MultisigConfig>,
    
    #[account(mut)]
    pub pauser: Signer<'info>,
}

pub fn handler(ctx: Context<PauseSystem>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig_config;
    multisig.is_paused = true;
    
    msg!("System paused by: {}", ctx.accounts.pauser.key());
    
    Ok(())
}
