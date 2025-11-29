use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeMultisig<'info> {
    #[account(
        init,
        payer = authority,
        space = MultisigConfig::LEN,
        seeds = [SEED_MULTISIG],
        bump
    )]
    pub multisig_config: Account<'info, MultisigConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeMultisig>,
    members: Vec<Pubkey>,
    threshold: u8,
) -> Result<()> {
    require!(
        members.len() <= MAX_MULTISIG_MEMBERS,
        ErrorCode::TooManyMembers
    );
    
    require!(
        threshold > 0 && threshold as usize <= members.len(),
        ErrorCode::InvalidThreshold
    );
    
    let multisig = &mut ctx.accounts.multisig_config;
    multisig.authority = ctx.accounts.authority.key();
    multisig.members = members;
    multisig.threshold = threshold;
    multisig.bump = ctx.bumps.multisig_config;
    
    Ok(())
}
