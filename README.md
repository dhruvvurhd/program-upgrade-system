# Program Upgrade System

**Production-ready governance layer for Solana program upgrades with multisig approval and timelock protection.**

[![Tests](https://img.shields.io/badge/tests-12%20passing-brightgreen)](tests/)
[![Solana](https://img.shields.io/badge/Solana-v1.18+-blueviolet)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-0.32.1-blue)](https://anchor-lang.com/)

---

## Overview

A governance system that enforces multisig approval and 48-hour timelocks before any Solana program can be upgraded. Prevents unilateral upgrades and gives users time to exit positions before changes take effect.

### Key Features

| Feature | Description |
|---------|-------------|
| **Multisig Governance** | Configurable N-of-M threshold (e.g., 3-of-5) |
| **48-Hour Timelock** | On-chain enforced, cannot be bypassed |
| **Buffer Validation** | Verifies BPF Loader ownership before upgrade |
| **Emergency Pause** | Any member can halt operations instantly |
| **Migration Tracking** | Account version tracking via PDAs |

---

## Quick Start

```bash
# Install dependencies
yarn install

# Run tests (12 passing)
anchor test
```

---

## Smart Contract Instructions

| Instruction | Purpose |
|-------------|---------|
| `initialize_multisig` | Set up governance with members and threshold |
| `propose_upgrade` | Create upgrade proposal with buffer pubkey |
| `approve_upgrade` | Vote to approve a proposal |
| `execute_upgrade` | Execute upgrade after timelock expires |
| `cancel_upgrade` | Cancel proposal and close buffer |
| `migrate_account` | Track account version migration |
| `pause_system` | Emergency halt all operations |
| `resume_system` | Resume after pause |

---

## Security Guarantees (On-Chain Enforced)

| Rule | Status |
|------|--------|
| Multisig membership | ✅ Enforced |
| Approval threshold | ✅ Enforced |
| 48-hour timelock | ✅ Enforced |
| Buffer ownership (BPF Loader) | ✅ Enforced |
| Duplicate approval prevention | ✅ Enforced |

---

## Architecture

```
program-upgrade-system/
├── programs/program-upgrade-system/   # Anchor smart contract
│   └── src/
│       ├── instructions/              # 8 instruction handlers
│       ├── state/                     # Account structures
│       └── utils.rs                   # Validation helpers
├── backend/                           # Rust REST API
│   └── src/
│       ├── api/                       # HTTP endpoints
│       ├── clients/                   # Solana RPC client
│       └── services/                  # Business logic
├── tests/                             # TypeScript tests
└── docs/                              # Documentation
```

---

## Upgrade Workflow

```
1. Build new program version
2. Create buffer: solana program write-buffer ./program.so
3. Set buffer authority to multisig PDA
4. Propose upgrade via API
5. 3 of 5 members approve
6. Wait 48 hours (timelock)
7. Execute upgrade
```

---

## Test Results

```
  program-upgrade-system
    ✔ Is initialized!
    ✔ Proposes an upgrade
    ✔ Approves an upgrade (timelock activates)
    ✔ Executes an upgrade (simulation)
    ✔ Cancels an upgrade
    ✔ Migrates an account
    Edge Cases
      ✔ Prevents duplicate approval
      ✔ Prevents double cancel
      ✔ Verifies proposal state after approval
    Pause/Resume System
      ✔ Pauses the system
      ✔ Resumes the system
      ✔ Prevents double pause

  12 passing
```

---

## Environment Variables

```env
DATABASE_URL=postgresql://user:pass@localhost/upgrade_system
RPC_URL=http://localhost:8899
PROGRAM_ID=BPeh5RUhTQbh637q8gGaGrasETYPcinBXqVKxutTB9v5
PAYER_KEYPAIR_PATH=/path/to/keypair.json
```

---

## Requirements

- Solana CLI 1.18+
- Anchor 0.32.1
- Rust 1.75+
- Node.js 18+
- PostgreSQL 14+ (for backend)

---

## Documentation

- [GDX DEX Integration Plan](docs/GDX-DEX-UPGRADE-INTEGRATION-PLAN.md)
- [Architecture](docs/architecture.md)
- [Governance Model](docs/governance.md)
- [API Reference](docs/api-reference.md)
- [Migration Guide](docs/migration-guide.md)
- [Testing Guide](docs/testing-guide.md)

---

## License

MIT
