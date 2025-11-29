pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use events::*;

declare_id!("HVv23Lp21eugaPym4MS8acrMEoNPjNy27yaqRG63w9p2");

#[program]
pub mod program_upgrade_system {
    use super::*;

    pub fn initialize_multisig(
        ctx: Context<InitializeMultisig>,
        members: Vec<Pubkey>,
        threshold: u8,
    ) -> Result<()> {
        instructions::initialize_multisig::handler(ctx, members, threshold)
    }

    pub fn propose_upgrade(
        ctx: Context<ProposeUpgrade>,
        new_program_buffer: Pubkey,
        description: String,
    ) -> Result<()> {
        instructions::propose_upgrade::handler(ctx, new_program_buffer, description)
    }

    pub fn approve_upgrade(
        ctx: Context<ApproveUpgrade>,
        proposal_id: Pubkey,
    ) -> Result<()> {
        instructions::approve_upgrade::handler(ctx, proposal_id)
    }

    pub fn execute_upgrade(
        ctx: Context<ExecuteUpgrade>,
        proposal_id: Pubkey,
    ) -> Result<()> {
        instructions::execute_upgrade::handler(ctx, proposal_id)
    }

    pub fn cancel_upgrade(
        ctx: Context<CancelUpgrade>,
        proposal_id: Pubkey,
    ) -> Result<()> {
        instructions::cancel_upgrade::handler(ctx, proposal_id)
    }

    pub fn migrate_account(
        ctx: Context<MigrateAccount>,
        old_account: Pubkey,
    ) -> Result<()> {
        instructions::migrate_account::handler(ctx, old_account)
    }
}
