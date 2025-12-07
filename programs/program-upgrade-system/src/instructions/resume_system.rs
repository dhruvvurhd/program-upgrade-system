use anchor_lang::prelude::*;
use crate::state::MultisigConfig;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ResumeSystem<'info> {
    #[account(
        mut,
        seeds = [b"multisig"],
        bump = multisig_config.bump,
        constraint = multisig_config.members.contains(&resumer.key()) @ ErrorCode::NotAMember,
        constraint = multisig_config.is_paused @ ErrorCode::SystemNotPaused,
    )]
    pub multisig_config: Account<'info, MultisigConfig>,
    
    #[account(mut)]
    pub resumer: Signer<'info>,
}

pub fn handler(ctx: Context<ResumeSystem>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig_config;
    multisig.is_paused = false;
    
    msg!("System resumed by: {}", ctx.accounts.resumer.key());
    
    Ok(())
}
