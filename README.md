# Solana Program Upgrade System

A secure, multisig-controlled upgrade and migration system for Solana programs with governance, timelock, and emergency controls.

## 🚀 Features

| Feature | Description |
|---------|-------------|
| **Multisig Governance** | Threshold-based approval (e.g., 3 of 5 members) |
| **48-Hour Timelock** | Delay between approval and execution |
| **Emergency Controls** | Pause/Resume system operations |
| **Account Migration** | Version tracking for data migrations |
| **Audit Trail** | Database logging of all actions |
| **REST API** | Backend service for off-chain integration |

## 📋 Prerequisites

- **Rust**: 1.75+
- **Solana CLI**: 1.18.x
- **Anchor CLI**: 0.30.x
- **Node.js**: 18+
- **PostgreSQL**: 14+ (for backend)

## 🏗 Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Client Applications                   │
└─────────────────────────────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              ▼                           ▼
┌─────────────────────┐       ┌─────────────────────┐
│   Anchor Program    │       │   Backend (Axum)    │
│   8 Instructions    │       │   REST API          │
└─────────────────────┘       └─────────────────────┘
```

### Smart Contract Instructions

| Instruction | Purpose |
|-------------|---------|
| `initialize_multisig` | Setup governance |
| `propose_upgrade` | Create proposal |
| `approve_upgrade` | Vote on proposal |
| `execute_upgrade` | Apply upgrade (after timelock) |
| `cancel_upgrade` | Emergency cancellation |
| `migrate_account` | Track account versions |
| `pause_system` | Emergency pause |
| `resume_system` | Resume operations |

## 🏃‍♂️ Quick Start

### 1. Start Local Validator
```bash
solana-test-validator -r --quiet
```

### 2. Build & Deploy
```bash
anchor build
anchor deploy
```

### 3. Run Tests
```bash
anchor test --skip-local-validator
```

Expected output:
```
  12 passing (8s)
```

## 📁 Project Structure

```
├── programs/program-upgrade-system/   # Anchor smart contract
│   └── src/instructions/              # 8 instruction handlers
├── backend/                           # Rust REST API server
│   └── src/                           
│       ├── api/                       # Route handlers
│       ├── db/                        # PostgreSQL schema
│       └── services/                  # Business logic
├── tests/                             # TypeScript integration tests
└── docs/                              # Documentation
    ├── architecture.md
    ├── api-reference.md
    ├── governance.md
    ├── migration-guide.md
    └── testing-guide.md
```

## 📖 Documentation

- [Architecture Overview](docs/architecture.md)
- [API Reference](docs/api-reference.md)
- [Governance Model](docs/governance.md)
- [Migration Guide](docs/migration-guide.md)
- [Testing Guide](docs/testing-guide.md)

## 🧪 Test Coverage

| Category | Tests |
|----------|-------|
| Core Workflow | 6 tests |
| Edge Cases | 3 tests |
| Pause/Resume | 3 tests |
| **Total** | **12 tests** |

## 🔧 Configuration

Copy `.env.example` to `.env` in the backend directory:
```bash
cp backend/.env.example backend/.env
```

## 📜 License

MIT
