# Operational Runbook

## Quick Reference

| Task | Command |
|------|---------|
| Deploy program | `anchor build && anchor deploy` |
| Create buffer | `./scripts/deploy_buffer.sh` |
| Propose upgrade | `./scripts/propose_upgrade.sh <BUFFER> <DESC>` |
| Approve proposal | `curl -X POST /proposals/:id/approve` |
| Execute upgrade | `curl -X POST /proposals/:id/execute` |
| Start migration | `./scripts/migrate_accounts.sh <PROPOSAL> <ACCOUNTS>` |
| Rollback | `./scripts/rollback.sh <PROPOSAL> <REASON>` |

## Initial Setup

### 1. Environment Setup

```bash
# Install dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
npm install -g @coral-xyz/anchor-cli

# Clone repository
git clone <repo>
cd program-upgrade-system

# Install Rust dependencies
cargo build

# Install Node dependencies
npm install
```

### 2. Database Setup

```bash
# Install PostgreSQL
brew install postgresql@15

# Start PostgreSQL
brew services start postgresql@15

# Create database
createdb upgrade_manager

# Run schema
psql upgrade_manager < backend/src/db/schema.sql
```

### 3. Configuration

```bash
# Copy environment template
cp backend/.env.example backend/.env

# Edit configuration
vim backend/.env

# Required values:
# - DATABASE_URL
# - RPC_URL
# - PROGRAM_ID
# - PAYER_KEYPAIR_PATH
```

### 4. Deploy Initial Program

```bash
# Generate program keypair
solana-keygen new -o ./target/deploy/program_upgrade_system-keypair.json

# Build program
anchor build

# Deploy (sets you as upgrade authority)
anchor deploy

# Verify deployment
solana program show <PROGRAM_ID>
```

### 5. Initialize Multisig

```bash
# Create initialization transaction
anchor run initialize-multisig

# Or via TypeScript:
const multisigMembers = [
    new PublicKey("Member1..."),
    new PublicKey("Member2..."),
    new PublicKey("Member3..."),
    new PublicKey("Member4..."),
    new PublicKey("Member5..."),
];

await program.methods
    .initializeMultisig(multisigMembers, 3) // 3-of-5 threshold
    .rpc();
```

### 6. Transfer Upgrade Authority

```bash
# Transfer to program's multisig PDA
solana program set-upgrade-authority \
    <PROGRAM_ID> \
    --new-upgrade-authority <MULTISIG_PDA>
```

### 7. Start Backend Services

```bash
# Build backend
cd backend
cargo build --release

# Run backend
cargo run --release

# Or with systemd:
sudo systemctl start upgrade-manager
```

## Standard Upgrade Procedure

### Day 0: Preparation

**1. Develop New Features**
```bash
# Create feature branch
git checkout -b feature/new-liquidation-logic

# Make changes to program
vim programs/program-upgrade-system/src/lib.rs

# Test locally
anchor test
```

**2. Security Audit**
```bash
# Run static analysis
cargo clippy -- -D warnings

# Run security audit
cargo audit

# Manual code review
# - Check for arithmetic overflows
# - Verify account validations
# - Review access controls
```

**3. Build & Test on Devnet**
```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Run integration tests
anchor test --provider.cluster devnet

# Manual testing
# - Create test positions
# - Execute trades
# - Verify liquidations work
```

### Day 1: Create Proposal

**1. Build Production Binary**
```bash
# Clean build
anchor clean
anchor build --verifiable

# Verify build
ls -lh target/deploy/program_upgrade_system.so
```

**2. Create Buffer**
```bash
./scripts/deploy_buffer.sh

# Output:
# Buffer Address: 8x7Hf2...
```

**3. Propose Upgrade**
```bash
./scripts/propose_upgrade.sh \
    "8x7Hf2..." \
    "Add improved liquidation logic with safety checks"

# Output:
# Proposal ID: abc-123-def-456
```

**4. Notify Multisig Members**
```bash
# Send notifications
curl -X POST /admin/notify \
    -d '{"proposal_id": "abc-123", "message": "New upgrade proposal"}'

# Or manually:
# - Email all multisig members
# - Post in governance Discord
# - Create forum post
```

### Day 1-2: Approval Phase

**1. Multisig Member 1 Approves**
```bash
curl -X POST http://api/proposals/abc-123/approve \
    -H "Content-Type: application/json" \
    -d '{"approver_keypair_path": "/path/to/member1.json"}'
```

**2. Multisig Member 2 Approves**
```bash
curl -X POST http://api/proposals/abc-123/approve \
    -H "Content-Type: application/json" \
    -d '{"approver_keypair_path": "/path/to/member2.json"}'
```

**3. Multisig Member 3 Approves (Threshold Met)**
```bash
curl -X POST http://api/proposals/abc-123/approve \
    -H "Content-Type: application/json" \
    -d '{"approver_keypair_path": "/path/to/member3.json"}'

# System automatically activates 48-hour timelock
```

### Day 1-3: Timelock Period

**1. Public Announcement**
```markdown
# Post on all channels:

⚠️ UPGRADE SCHEDULED ⚠️

A program upgrade has been approved and will execute in 48 hours.

**What's changing:**
- Improved liquidation logic
- Enhanced safety checks
- Bug fixes

**Timeline:**
- Approval: 2024-12-01 10:00 UTC
- Execution: 2024-12-03 10:00 UTC

**Actions available:**
- Review code: https://github.com/...
- Exit positions if concerned
- Ask questions in Discord

**Expected downtime:** None (zero-downtime upgrade)
```

**2. Monitor Proposal**
```bash
# Check status
curl http://api/proposals/abc-123

# Check timelock remaining
# timelock_until - current_time = hours remaining
```

**3. User Communication**
- Answer questions in Discord
- Provide code diff for review
- Explain changes in simple terms

### Day 3: Execution

**1. Verify Timelock Expired**
```bash
curl http://api/proposals/abc-123 | jq '.timelock_until'

# Compare with current time
date -u +"%Y-%m-%dT%H:%M:%SZ"
```

**2. Execute Upgrade**
```bash
curl -X POST http://api/proposals/abc-123/execute \
    -H "Content-Type: application/json" \
    -d '{"executor_keypair_path": "/path/to/executor.json"}'

# Monitor transaction
solana confirm <TX_SIGNATURE> -v
```

**3. Verify Program Upgraded**
```bash
# Check program account
solana program show <PROGRAM_ID>

# Verify version in program data
anchor idl fetch <PROGRAM_ID>
```

### Day 3-4: Migration

**1. Discover Accounts**
```bash
# Get all program accounts
solana program show <PROGRAM_ID> --accounts > accounts.txt

# Filter for accounts needing migration
# (depends on your account types)
```

**2. Start Migration**
```bash
./scripts/migrate_accounts.sh abc-123 accounts.txt

# Output:
# Job ID: xyz-789
```

**3. Monitor Migration**
```bash
# Check progress every minute
watch -n 60 'curl http://api/migration/xyz-789/progress'

# Expected output:
# {
#   "total": 1000,
#   "completed": 250,
#   "percentage": 25.0,
#   "status": "in_progress"
# }
```

**4. Validate Migration**
```bash
# Check for failed migrations
psql upgrade_manager -c \
    "SELECT * FROM account_migrations WHERE status = 'failed';"

# Retry failed accounts
# ... retry logic ...
```

### Day 4-5: Monitoring

**1. Watch Metrics**
```bash
# Transaction success rate
# Account deserialization errors
# User complaints
# System performance
```

**2. Check Logs**
```bash
# Backend logs
tail -f /var/log/upgrade-manager/backend.log

# Solana program logs
solana logs <PROGRAM_ID>
```

**3. User Feedback**
- Monitor Discord
- Check support tickets
- Watch error rates

## Emergency Procedures

### Emergency Rollback

**Scenario**: Critical bug discovered after upgrade

**1. Immediate Assessment**
```bash
# Assess severity
# - Are funds at risk?
# - Is trading halted?
# - How many users affected?
```

**2. Decide on Action**
- **Minor bug**: Fix and upgrade again
- **Major bug**: Immediate rollback
- **Critical bug**: Halt system, rollback, investigate

**3. Execute Rollback**
```bash
# Cancel current proposal if not executed
./scripts/rollback.sh abc-123 "Critical bug in liquidation"

# Deploy old version to buffer
solana program write-buffer ./backups/program_v1.so

# Create emergency upgrade proposal
./scripts/propose_upgrade.sh <OLD_BUFFER> "Emergency rollback to v1"

# Fast-track approval (all multisig members approve immediately)
# Execute without waiting full timelock (if governance allows)
```

### System Down

**Scenario**: Backend API not responding

**1. Check Health**
```bash
curl http://api/health

# If no response, check service
systemctl status upgrade-manager
```

**2. Restart Services**
```bash
sudo systemctl restart upgrade-manager

# Check logs
journalctl -u upgrade-manager -f
```

**3. Database Issues**
```bash
# Check PostgreSQL
systemctl status postgresql

# Check connections
psql upgrade_manager -c "SELECT count(*) FROM pg_stat_activity;"

# Restart if needed
sudo systemctl restart postgresql
```

### Multisig Member Key Compromised

**1. Immediate Actions**
- Revoke compromised key
- Update multisig configuration
- Audit recent transactions

**2. Update Multisig**
```rust
// Deploy updated multisig config
let new_members = [
    old_member1,
    old_member2,
    new_member_3, // Replace compromised key
    old_member4,
    old_member5,
];

await program.methods
    .updateMultisig(new_members, 3)
    .rpc();
```

## Monitoring & Alerts

### Key Metrics

```bash
# Proposal metrics
- Proposals pending approval
- Timelocks expiring soon
- Proposals stuck

# Migration metrics
- Accounts migrated
- Migration success rate
- Migration time

# System metrics
- Transaction success rate
- Response time
- Error rate
```

### Alert Configuration

```yaml
alerts:
  - name: Proposal Stuck
    condition: approval_count < threshold AND age > 72h
    action: Notify multisig members
    
  - name: Migration Failed
    condition: failure_rate > 5%
    action: Pause migration, investigate
    
  - name: High Error Rate
    condition: error_rate > 1%
    action: Alert engineers
```

## Backup & Recovery

### Regular Backups

```bash
# Daily program backups
cp target/deploy/program_upgrade_system.so \
    backups/program_$(date +%Y%m%d).so

# Daily database backups
pg_dump upgrade_manager > backups/db_$(date +%Y%m%d).sql

# Keep last 30 days
find backups/ -mtime +30 -delete
```

### Restore from Backup

```bash
# Restore database
psql upgrade_manager < backups/db_20241201.sql

# Rollback program
# (use emergency rollback procedure above)
```

## Maintenance Tasks

### Weekly
- [ ] Review pending proposals
- [ ] Check backup integrity
- [ ] Monitor disk space
- [ ] Review error logs

### Monthly
- [ ] Security audit
- [ ] Performance review
- [ ] Update dependencies
- [ ] Test disaster recovery

### Quarterly
- [ ] Multisig key rotation
- [ ] System architecture review
- [ ] Capacity planning
- [ ] Update documentation

## Troubleshooting

### Common Issues

**Issue**: Transaction fails with "Timelock not expired"
```
Solution: Wait for full 48 hours after approval
```

**Issue**: Account deserialization fails
```
Solution: Check if migration needed, run migrate_account
```

**Issue**: Insufficient approvals
```
Solution: Get more multisig members to approve
```

**Issue**: Buffer account invalid
```
Solution: Verify buffer was created correctly, check ownership
```

## Contact Information

**On-Call Engineers**: 
- Primary: +1-xxx-xxx-xxxx
- Secondary: +1-xxx-xxx-xxxx

**Multisig Members**:
- Member 1: email@example.com
- Member 2: email@example.com
- Member 3: email@example.com

**Emergency Escalation**:
1. On-call engineer (response: 15 min)
2. Tech lead (response: 30 min)
3. CTO (response: 1 hour)
