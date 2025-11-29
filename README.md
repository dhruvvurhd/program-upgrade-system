# Program Upgrade & Migration System

A production-ready governance system for safely upgrading Solana programs with multisig approvals, timelock delays, and account state migration.

## 🎯 Features

- ✅ **Multisig Governance** - Requires 3-of-5 approvals for upgrades
- ✅ **48-Hour Timelock** - Mandatory delay after approval
- ✅ **Safe Program Upgrades** - Via BPF Upgradeable Loader
- ✅ **Account Migration** - Batch migration with progress tracking
- ✅ **Emergency Rollback** - Cancel proposals and revert upgrades
- ✅ **Audit Trail** - Complete history of all actions
- ✅ **Zero Downtime** - Upgrades without halting the system
- ✅ **REST API** - HTTP interface for all operations

## 📋 Table of Contents

- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Testing](#testing)
- [Deployment](#deployment)
- [Contributing](#contributing)

## 🏗 Architecture

```
┌─────────────┐
│   Clients   │ (CLI, Dashboard, Scripts)
└──────┬──────┘
       │
┌──────▼──────┐
│  REST API   │ (Axum Backend)
└──────┬──────┘
       │
┌──────▼──────┐
│   Solana    │ (Anchor Program)
│  Blockchain │
└──────┬──────┘
       │
┌──────▼──────┐
│  PostgreSQL │ (Audit & History)
└─────────────┘
```

**Components:**
- **Anchor Program**: On-chain governance logic
- **Rust Backend**: Off-chain orchestration
- **PostgreSQL**: Historical data & audit logs
- **REST API**: HTTP interface

See [docs/architecture.md](docs/architecture.md) for detailed architecture.

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+
- Solana CLI 1.17+
- Anchor 0.29+
- PostgreSQL 15+
- Node.js 18+

### 1. Clone & Build

```bash
git clone <repository>
cd program-upgrade-system

# Build Anchor program
anchor build

# Build backend
cd backend && cargo build --release
```

### 2. Setup Database

```bash
createdb upgrade_manager
psql upgrade_manager < backend/src/db/schema.sql
```

### 3. Configure

```bash
cp backend/.env.example backend/.env
# Edit backend/.env with your configuration
```

### 4. Deploy Program

```bash
anchor deploy
```

### 5. Start Backend

```bash
cd backend
cargo run --release
```

### 6. Initialize Multisig

```bash
anchor run initialize-multisig
```

## 📦 Installation

### System Dependencies

```bash
# macOS
brew install rust solana anchor postgresql

# Linux (Ubuntu/Debian)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
npm install -g @coral-xyz/anchor-cli
sudo apt install postgresql-15
```

### Project Dependencies

```bash
# Rust dependencies
cargo build

# Node dependencies (for tests)
npm install

# Backend dependencies
cd backend && cargo build
```

## 📖 Usage

### Create an Upgrade Proposal

```bash
# 1. Build new program version
anchor build

# 2. Create buffer
./scripts/deploy_buffer.sh
# Output: Buffer Address: 8x7Hf...

# 3. Propose upgrade
./scripts/propose_upgrade.sh 8x7Hf... "Add new feature"
# Output: Proposal ID: abc-123
```

### Approve Proposal

```bash
# Multisig member 1
curl -X POST http://localhost:3000/proposals/abc-123/approve \
    -H "Content-Type: application/json" \
    -d '{"approver_keypair_path": "/path/to/member1.json"}'

# Multisig member 2
curl -X POST http://localhost:3000/proposals/abc-123/approve \
    -H "Content-Type: application/json" \
    -d '{"approver_keypair_path": "/path/to/member2.json"}'

# Multisig member 3 (threshold met, timelock activated)
curl -X POST http://localhost:3000/proposals/abc-123/approve \
    -H "Content-Type: application/json" \
    -d '{"approver_keypair_path": "/path/to/member3.json"}'
```

### Execute After Timelock

```bash
# Wait 48 hours, then execute
curl -X POST http://localhost:3000/proposals/abc-123/execute \
    -H "Content-Type: application/json" \
    -d '{"executor_keypair_path": "/path/to/executor.json"}'
```

### Migrate Accounts

```bash
# Create list of accounts
echo "Account1..." > accounts.txt
echo "Account2..." >> accounts.txt

# Start migration
./scripts/migrate_accounts.sh abc-123 accounts.txt

# Monitor progress
curl http://localhost:3000/migration/xyz-789/progress
```

### Emergency Rollback

```bash
./scripts/rollback.sh abc-123 "Critical bug discovered"
```

## 🔌 API Reference

### Proposals

```http
GET    /proposals           # List all proposals
GET    /proposals/:id       # Get proposal details
POST   /proposals/propose   # Create new proposal
POST   /proposals/:id/approve   # Approve proposal
POST   /proposals/:id/execute   # Execute upgrade
POST   /proposals/:id/cancel    # Cancel proposal
```

### Migration

```http
POST   /migration/start         # Start migration job
GET    /migration/:id/progress  # Get migration progress
```

### Example: Create Proposal

```bash
curl -X POST http://localhost:3000/proposals/propose \
    -H "Content-Type: application/json" \
    -d '{
        "new_program_buffer": "8x7Hf2vKjP...",
        "description": "Add liquidation improvements"
    }'
```

**Response:**
```json
{
    "proposal_id": "abc-123-def-456",
    "status": "created"
}
```

## 🧪 Testing

### Unit Tests

```bash
# Test Anchor program
anchor test

# Test backend
cd backend && cargo test
```

### Integration Tests

```bash
# Run full integration tests
anchor test --skip-local-validator

# Run specific test
cargo test test_full_upgrade_flow
```

### Local Testing

```bash
# Start local validator
solana-test-validator

# Deploy to localhost
anchor deploy --provider.cluster localnet

# Run tests
anchor test --skip-deploy
```

## 🚢 Deployment

### Devnet Deployment

```bash
# Configure for devnet
solana config set --url devnet

# Airdrop SOL
solana airdrop 2

# Deploy
anchor deploy --provider.cluster devnet

# Initialize
anchor run initialize-multisig --provider.cluster devnet
```

### Mainnet Deployment

```bash
# ⚠️ MAINNET - BE CAREFUL ⚠️

# 1. Audit code thoroughly
cargo clippy -- -D warnings
cargo audit

# 2. Test on devnet extensively
anchor test --provider.cluster devnet

# 3. Configure for mainnet
solana config set --url mainnet-beta

# 4. Deploy
anchor deploy --provider.cluster mainnet-beta

# 5. Verify deployment
solana program show <PROGRAM_ID>

# 6. Initialize multisig with production keys
anchor run initialize-multisig --provider.cluster mainnet-beta

# 7. Transfer upgrade authority to multisig
solana program set-upgrade-authority \
    <PROGRAM_ID> \
    --new-upgrade-authority <MULTISIG_PDA>
```

## 📚 Documentation

- [Architecture](docs/architecture.md) - System design and components
- [Migration Guide](docs/migration_guide.md) - Account migration strategies
- [Operational Runbook](docs/operational_runbook.md) - Day-to-day operations
- [API Reference](docs/api_reference.md) - Complete API documentation

## 🔐 Security

### Security Features

- Multisig governance (prevents single point of control)
- Timelock delays (48-hour review period)
- Audit trail (all actions logged)
- Emergency controls (cancel/rollback)
- Account validation (comprehensive constraints)

### Security Audits

- [ ] Internal code review
- [ ] External security audit
- [ ] Formal verification
- [ ] Bug bounty program

### Reporting Security Issues

**DO NOT** create public GitHub issues for security vulnerabilities.

Email: security@example.com

## 🤝 Contributing

### Development Workflow

```bash
# 1. Fork repository
# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Make changes
# 4. Run tests
anchor test && cd backend && cargo test

# 5. Commit
git commit -m "feat: add my feature"

# 6. Push
git push origin feature/my-feature

# 7. Create Pull Request
```

### Coding Standards

- Follow Rust best practices
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Write tests for new features
- Update documentation

## 📝 License

MIT License - see [LICENSE](LICENSE) file

## 🙋 Support

- **Documentation**: See [docs/](docs/)
- **Issues**: GitHub Issues
- **Discord**: [Join our Discord](#)
- **Email**: support@example.com

## 🗺 Roadmap

### v1.0 (Current)
- [x] Multisig governance
- [x] Timelock mechanism
- [x] Program upgrades
- [x] Account migration
- [x] REST API
- [x] PostgreSQL integration

### v1.1 (Planned)
- [ ] Web dashboard
- [ ] Notification system
- [ ] Advanced analytics
- [ ] Multi-program support

### v2.0 (Future)
- [ ] On-chain voting
- [ ] Token-weighted governance
- [ ] Automatic rollback detection
- [ ] Cross-program orchestration

## 📊 Project Status

- **Status**: Production Ready
- **Version**: 1.0.0
- **Last Updated**: 2024-12-01
- **Maintainers**: [List maintainers]

## 🏆 Acknowledgments

- Solana Foundation
- Anchor Framework Team
- Squads Protocol
- Community contributors

---

**Built with ❤️ for the Solana ecosystem**
