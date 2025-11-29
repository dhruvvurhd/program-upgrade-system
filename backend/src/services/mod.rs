use sqlx::PgPool;
use crate::clients::AnchorClient;

pub mod multisig_coordinator;
pub mod timelock_manager;
pub mod program_builder;
pub mod migration_manager;
pub mod rollback_handler;

pub use multisig_coordinator::*;
pub use timelock_manager::*;
pub use program_builder::*;
pub use migration_manager::*;
pub use rollback_handler::*;

pub struct Services {
    pub db_pool: PgPool,
    pub anchor_client: AnchorClient,
    pub multisig_coordinator: MultisigCoordinator,
    pub timelock_manager: TimelockManager,
    pub program_builder: ProgramBuilder,
    pub migration_manager: MigrationManager,
    pub rollback_handler: RollbackHandler,
}

impl Services {
    pub fn new(db_pool: PgPool, anchor_client: AnchorClient) -> Self {
        Self {
            multisig_coordinator: MultisigCoordinator::new(db_pool.clone()),
            timelock_manager: TimelockManager::new(db_pool.clone()),
            program_builder: ProgramBuilder::new(),
            migration_manager: MigrationManager::new(db_pool.clone()),
            rollback_handler: RollbackHandler::new(db_pool.clone()),
            db_pool,
            anchor_client,
        }
    }
}
