# Migration Guide

## Overview
Account migration is required when the data layout changes between program versions. This guide covers how to plan and execute migrations safely.

## Migration Flow

```
1. Deploy new program buffer
2. Propose upgrade (includes migration plan)
3. Collect approvals
4. Wait for timelock
5. Execute upgrade
6. Start account migration
7. Verify migration complete
```

## Account Versioning

Each migrated account gets an `AccountVersion` PDA:

```rust
#[account]
pub struct AccountVersion {
    pub account: Pubkey,
    pub version: u8,
    pub migrated: bool,
    pub migrated_at: Option<i64>,
}
```

## Migration Process

### 1. Identify Accounts
```rust
// Find all accounts of old type
let accounts = get_program_accounts(&old_program_id)?;
```

### 2. Call Migrate
```rust
pub fn migrate_account(
    ctx: Context<MigrateAccount>,
    old_account: Pubkey,
) -> Result<()>
```

### 3. Track Progress
```sql
SELECT 
    (migrated_accounts::float / total_accounts) * 100 as progress
FROM migration_jobs
WHERE id = $1;
```

## Best Practices

1. **Batch Processing** - Migrate in batches of 10-50 accounts
2. **Verification** - Verify data integrity after each batch
3. **Rollback Plan** - Keep old program ready for emergency
4. **Monitoring** - Track progress via API

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| `AccountAlreadyMigrated` | Duplicate call | Skip account |
| `InvalidAccountVersion` | Wrong version | Check program version |
| `MigrationFailed` | Data transform error | Retry or manual fix |
