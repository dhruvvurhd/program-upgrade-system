use anchor_lang::prelude::*;

#[constant]
pub const SEED_MULTISIG: &[u8] = b"multisig";

#[constant]
pub const SEED_PROPOSAL: &[u8] = b"proposal";

#[constant]
pub const SEED_MIGRATION: &[u8] = b"migration";

#[constant]
pub const TIMELOCK_PERIOD: i64 = 172800; // 48 hours in seconds


pub const MAX_DESCRIPTION_LENGTH: usize = 500;

pub const MAX_MULTISIG_MEMBERS: usize = 10;

pub const MAX_APPROVALS: usize = 10;
