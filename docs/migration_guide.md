# Account Migration Guide

## Overview

Account migration is the process of transforming account data from an old program version to a new version when the account structure changes.

## Why Migration is Needed

When you upgrade a Solana program and change account structures, existing accounts won't deserialize correctly with the new schema.

### Example Scenario

```rust
// Version 1 (Old)
#[account]
pub struct UserPosition {
    pub owner: Pubkey,          // 32 bytes
    pub size: i64,              // 8 bytes
    pub entry_price: u64,       // 8 bytes
}
// Total: 48 bytes + 8 byte discriminator = 56 bytes

// Version 2 (New)
#[account]
pub struct UserPosition {
    pub owner: Pubkey,          // 32 bytes
    pub size: i64,              // 8 bytes
    pub entry_price: u64,       // 8 bytes
    pub liquidation_price: u64, // 8 bytes (NEW!)
    pub last_updated: i64,      // 8 bytes (NEW!)
}
// Total: 64 bytes + 8 byte discriminator = 72 bytes
```

Without migration, the program would fail to deserialize old accounts.

## Migration Strategies

### Strategy 1: Account Versioning (Recommended)

Use an enum to support multiple versions:

```rust
#[account]
pub enum UserPosition {
    V1 {
        owner: Pubkey,
        size: i64,
        entry_price: u64,
    },
    V2 {
        owner: Pubkey,
        size: i64,
        entry_price: u64,
        liquidation_price: u64,
        last_updated: i64,
    },
}

impl UserPosition {
    pub fn migrate_to_v2(&self) -> UserPosition {
        match self {
            UserPosition::V1 { owner, size, entry_price } => {
                UserPosition::V2 {
                    owner: *owner,
                    size: *size,
                    entry_price: *entry_price,
                    liquidation_price: calculate_liquidation(*entry_price, *size),
                    last_updated: Clock::get()?.unix_timestamp,
                }
            }
            UserPosition::V2 { .. } => self.clone(),
        }
    }
}
```

### Strategy 2: Lazy Migration

Migrate accounts on-demand when accessed:

```rust
pub fn process_trade(ctx: Context<Trade>) -> Result<()> {
    let position = &mut ctx.accounts.user_position;
    
    // Check version and migrate if needed
    if position.version < 2 {
        migrate_to_v2(position)?;
        position.version = 2;
    }
    
    // Continue with trade logic
    // ...
}
```

### Strategy 3: Batch Migration (Our Approach)

Proactively migrate all accounts:

```rust
pub fn migrate_account(
    ctx: Context<MigrateAccount>,
    old_account: Pubkey,
) -> Result<()> {
    let old_data = ctx.accounts.old_account.try_borrow_data()?;
    
    // 1. Deserialize old structure
    let old_position = UserPositionV1::try_from_slice(&old_data[8..])?;
    
    // 2. Transform to new structure
    let new_position = UserPositionV2 {
        owner: old_position.owner,
        size: old_position.size,
        entry_price: old_position.entry_price,
        liquidation_price: calculate_liquidation(
            old_position.entry_price,
            old_position.size
        ),
        last_updated: Clock::get()?.unix_timestamp,
    };
    
    // 3. Realloc account if size changed
    let new_size = 8 + std::mem::size_of::<UserPositionV2>();
    ctx.accounts.old_account.realloc(new_size, false)?;
    
    // 4. Serialize new data
    let mut data = ctx.accounts.old_account.try_borrow_mut_data()?;
    new_position.try_serialize(&mut &mut data[8..])?;
    
    Ok(())
}
```

## Migration Process

### Phase 1: Preparation

1. **Identify Accounts**
   ```bash
   # Get all accounts owned by program
   solana program show <PROGRAM_ID> --accounts
   ```

2. **Create Account List**
   ```bash
   # Save to file
   echo "Account1..." > accounts.txt
   echo "Account2..." >> accounts.txt
   ```

3. **Test Migration on Devnet**
   ```bash
   # Deploy to devnet first
   anchor test --provider.cluster devnet
   ```

### Phase 2: Execution

1. **Start Migration Job**
   ```bash
   ./scripts/migrate_accounts.sh <PROPOSAL_ID> accounts.txt
   ```

2. **Monitor Progress**
   ```bash
   curl http://localhost:3000/migration/<JOB_ID>/progress
   ```

3. **Verify Results**
   ```bash
   # Check migration status in database
   psql -d upgrade_manager -c "SELECT * FROM account_migrations WHERE migration_job_id = '<JOB_ID>';"
   ```

### Phase 3: Validation

1. **Check Account Data**
   ```bash
   # Verify account deserializes correctly
   solana account <ACCOUNT_ADDRESS>
   ```

2. **Run Integration Tests**
   ```bash
   anchor test
   ```

3. **Monitor Error Rates**
   - Check transaction success rate
   - Monitor deserialization errors
   - Watch for user complaints

## Data Transformation Logic

### Calculating Derived Fields

```rust
fn calculate_liquidation_price(entry_price: u64, size: i64) -> u64 {
    // Liquidation at 80% of entry for longs, 120% for shorts
    if size > 0 {
        entry_price * 80 / 100
    } else {
        entry_price * 120 / 100
    }
}
```

### Handling Missing Data

```rust
// Use sensible defaults for new fields
let new_position = UserPositionV2 {
    // ... existing fields ...
    liquidation_price: old_position.entry_price * 80 / 100, // Default calculation
    last_updated: 0, // Unix epoch, indicates never updated
};
```

## Batching Strategy

### Optimal Batch Size

```rust
const BATCH_SIZE: usize = 100; // Migrate 100 accounts at a time
const DELAY_MS: u64 = 100;     // 100ms delay between batches
```

### Rate Limiting

```rust
for chunk in accounts.chunks(BATCH_SIZE) {
    for account in chunk {
        migrate_account(account).await?;
    }
    
    // Rate limiting to avoid overwhelming network
    tokio::time::sleep(Duration::from_millis(DELAY_MS)).await;
}
```

## Error Handling

### Retry Logic

```rust
async fn migrate_with_retry(account: Pubkey) -> Result<()> {
    const MAX_RETRIES: u32 = 3;
    let mut attempts = 0;
    
    loop {
        match migrate_account(account).await {
            Ok(_) => return Ok(()),
            Err(e) if attempts < MAX_RETRIES => {
                attempts += 1;
                tracing::warn!("Migration failed, retry {}: {}", attempts, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### Failed Migration Handling

```rust
// Record failed migrations for manual review
if let Err(e) = migrate_account(account).await {
    record_failure(account, e).await?;
    // Continue with next account instead of stopping
}
```

## Rollback Migration

If migration causes issues:

```rust
pub fn rollback_migration(ctx: Context<RollbackMigration>) -> Result<()> {
    let account = &mut ctx.accounts.account;
    let version_tracker = &ctx.accounts.version_tracker;
    
    // Restore old data from backup
    let old_data = version_tracker.old_data_backup;
    
    // Revert account
    account.data = old_data;
    account.version = version_tracker.old_version;
    
    Ok(())
}
```

## Testing Migration

### Unit Tests

```typescript
it("migrates account from v1 to v2", async () => {
    // Create v1 account
    await program.methods.createPositionV1()
        .accounts({ ... })
        .rpc();
    
    // Migrate to v2
    await program.methods.migrateAccount()
        .accounts({ ... })
        .rpc();
    
    // Verify v2 fields exist
    const account = await program.account.userPosition.fetch(positionPda);
    assert.ok(account.liquidationPrice > 0);
    assert.ok(account.lastUpdated > 0);
});
```

### Integration Tests

```typescript
it("migrates all accounts in batch", async () => {
    const accounts = await getAllUserPositions();
    
    for (const account of accounts) {
        await migrateAccount(account);
    }
    
    // Verify all migrated
    const migratedCount = await getMigratedCount();
    assert.equal(migratedCount, accounts.length);
});
```

## Best Practices

1. **Always Test on Devnet First**
2. **Create Database Backups Before Migration**
3. **Migrate in Small Batches**
4. **Monitor Progress Continuously**
5. **Have Rollback Plan Ready**
6. **Validate Data After Migration**
7. **Communicate with Users**
8. **Keep Old Program Version Available**

## Monitoring

### Key Metrics

- **Migration Progress**: `migrated_accounts / total_accounts`
- **Success Rate**: `successful / attempted`
- **Average Time**: Per-account migration time
- **Error Rate**: Failed migrations / total

### Alerts

Set up alerts for:
- Migration stalled (no progress for 5 minutes)
- High error rate (>5% failures)
- Performance degradation
- Unexpected account states

## Troubleshooting

### Common Issues

**Issue**: Account size mismatch
```
Error: Account data too small
```
**Solution**: Ensure realloc is called with correct new size

**Issue**: Deserialization fails
```
Error: Failed to deserialize account
```
**Solution**: Check discriminator and field order match

**Issue**: Out of SOL for rent
```
Error: Insufficient funds
```
**Solution**: Fund migration wallet with enough SOL

## Production Checklist

- [ ] Test migration on devnet
- [ ] Backup database
- [ ] Backup all account data
- [ ] Verify rent-exempt balances
- [ ] Set up monitoring
- [ ] Prepare rollback plan
- [ ] Notify users of maintenance window
- [ ] Execute migration
- [ ] Validate results
- [ ] Monitor for 24-48 hours
