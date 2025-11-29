use anchor_lang::prelude::*;
use crate::error::ErrorCode;

pub fn validate_multisig_member(members: &Vec<Pubkey>, signer: &Pubkey) -> Result<()> {
    require!(
        members.contains(signer),
        ErrorCode::UnauthorizedSigner
    );
    Ok(())
}

pub fn validate_timelock_expired(activated_at: i64, period: i64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    let expiry = activated_at.checked_add(period).ok_or(ErrorCode::MathOverflow)?;
    
    require!(
        current_time >= expiry,
        ErrorCode::TimelockNotExpired
    );
    Ok(())
}

pub fn validate_threshold(approval_count: u8, threshold: u8) -> Result<bool> {
    Ok(approval_count >= threshold)
}

pub fn validate_description_length(description: &String, max_len: usize) -> Result<()> {
    require!(
        description.len() <= max_len,
        ErrorCode::DescriptionTooLong
    );
    Ok(())
}
