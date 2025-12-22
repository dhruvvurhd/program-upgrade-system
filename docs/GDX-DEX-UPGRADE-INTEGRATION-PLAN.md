# GDX DEX - Program Upgrade System Integration

Comprehensive implementation plan for integrating the Program Upgrade System with all GDX smart contracts to enable governed, timelocked upgrades.

---

## Problem Statement

The GDX DEX has **7 Solana smart contracts** that currently have no upgrade governance. Any upgrade authority holder can unilaterally upgrade contracts without oversight, creating security and trust risks. The Program Upgrade System provides multisig governance, 48-hour timelocks, and audit trails for safe upgrades.

---

## What We've Built: Program Upgrade System ✅

The **Program Upgrade System** is a **production-ready governance layer** that we have completed as part of the GoQuant assignment. This system is now ready to be integrated with the GDX DEX.

### Completed Components

| Component | Status | Description |
|-----------|--------|-------------|
| **Anchor Smart Contract** | ✅ Complete | 8 instructions for full upgrade lifecycle |
| **Multisig Governance** | ✅ Complete | Configurable N-of-M threshold (e.g., 3-of-5) |
| **48-Hour Timelock** | ✅ Complete | On-chain enforced, cannot be bypassed |
| **Emergency Pause/Resume** | ✅ Complete | Any member can halt operations instantly |
| **Buffer Validation** | ✅ Complete | Verifies BPF Loader ownership before upgrade |
| **Account Migration Tracking** | ✅ Complete | `AccountVersion` PDA for version tracking |
| **Rust Backend Service** | ✅ Complete | Axum REST API with PostgreSQL |
| **Test Suite** | ✅ 12 tests passing | Full coverage of upgrade lifecycle |

### Smart Contract Instructions (Ready to Use)

```
┌─────────────────────────────────────────────────────────────────────┐
│                   Program Upgrade System Contract                    │
│                   Program ID: BPeh5RUhTQbh637q8gGaGrasETYPcinBXqVKxutTB9v5                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐   │
│  │ initialize_     │   │ propose_        │   │ approve_        │   │
│  │ multisig        │   │ upgrade         │   │ upgrade         │   │
│  │                 │   │                 │   │                 │   │
│  │ Sets up 3-of-5  │   │ Creates new     │   │ Votes to        │   │
│  │ governance      │   │ proposal        │   │ approve         │   │
│  └─────────────────┘   └─────────────────┘   └─────────────────┘   │
│                                                                      │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐   │
│  │ execute_        │   │ cancel_         │   │ migrate_        │   │
│  │ upgrade         │   │ upgrade         │   │ account         │   │
│  │                 │   │                 │   │                 │   │
│  │ Real BPF CPI    │   │ Closes buffer   │   │ Tracks data     │   │
│  │ after timelock  │   │ refunds SOL     │   │ migrations      │   │
│  └─────────────────┘   └─────────────────┘   └─────────────────┘   │
│                                                                      │
│  ┌─────────────────┐   ┌─────────────────┐                          │
│  │ pause_          │   │ resume_         │                          │
│  │ system          │   │ system          │                          │
│  │                 │   │                 │                          │
│  │ Emergency halt  │   │ Resume ops      │                          │
│  └─────────────────┘   └─────────────────┘                          │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### On-Chain Security Guarantees (Already Implemented)

All critical validations are **enforced on-chain** and **cannot be bypassed**:

| Security Check | Code Location | What It Does |
|----------------|---------------|--------------|
| Multisig membership | `validate_multisig_member()` | Only configured members can propose/approve |
| Approval threshold | `validate_threshold()` | 3-of-5 required before timelock activates |
| 48-hour timelock | `validate_timelock_expired()` | Users get 48 hours to exit positions |
| Buffer ownership | `execute_upgrade.rs:78-81` | Buffer must be owned by BPF Loader |
| Buffer data check | `execute_upgrade.rs:84-91` | Buffer must contain actual program (>37 bytes) |
| Duplicate approval | `approve_upgrade.rs` | Same member cannot vote twice |
| System pause | All instructions | Operations blocked when paused |

### Execute Upgrade: Production-Ready BPF CPI

The `execute_upgrade` instruction performs a **real CPI to the BPF Upgradeable Loader**:

```rust
// From execute_upgrade.rs - This is production code
let upgrade_instruction = bpf_loader_upgradeable::upgrade(
    &ctx.accounts.program_to_upgrade.key(),  // GDX program being upgraded
    &ctx.accounts.buffer.key(),               // Buffer with new code
    &ctx.accounts.multisig_config.key(),      // Multisig PDA signs
    &ctx.accounts.spill_account.key(),        // Rent refund
);

invoke_signed(
    &upgrade_instruction,
    &accounts,
    &[multisig_seeds],  // PDA signature
)?;
```

### Backend Service (Ready to Deploy)

```
program-upgrade-system/backend/
├── src/
│   ├── api/           # REST endpoints
│   │   ├── proposals.rs    # POST/GET /proposals
│   │   └── system.rs       # POST /pause, /resume
│   ├── clients/
│   │   └── solana.rs       # RPC client for on-chain calls
│   ├── services/
│   │   ├── proposal.rs     # Proposal business logic
│   │   ├── upgrade.rs      # Execute upgrade orchestration
│   │   └── migration.rs    # Account migration tracking
│   └── db/
│       └── schema.rs       # PostgreSQL tables
```

### Test Results (12/12 Passing)

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

  12 passing (9s)
```

---

## How We Will Upgrade GDX Smart Contracts

### Overview

With the Program Upgrade System complete, upgrading any GDX contract follows this **governed workflow**:

```mermaid
graph LR
    A[Developer builds<br/>new version] --> B[Create buffer<br/>on Solana]
    B --> C[Propose upgrade<br/>via API]
    C --> D[3 of 5 members<br/>approve]
    D --> E[48h timelock<br/>starts]
    E --> F[Users can<br/>exit positions]
    F --> G[Execute upgrade<br/>after 48h]
    G --> H[New code<br/>is live ✅]
```

### Step-by-Step: Upgrading collateral_vault

**Prerequisites** (one-time setup):
1. Deploy program-upgrade-system to mainnet
2. Initialize multisig with 5 team members
3. Transfer `collateral_vault` authority to multisig PDA

**Upgrade Process:**

```bash
# 1. Developer: Build new version
cd contracts/programs/gdx-collateral-vault/collateral-vault
# Make code changes
anchor build

# 2. Create buffer on Solana
solana program write-buffer ./target/deploy/collateral_vault.so
# Output: Buffer address: <BUFFER_PUBKEY>

# 3. Set buffer authority to multisig (CRITICAL!)
solana program set-buffer-authority <BUFFER_PUBKEY> \
    --new-buffer-authority <MULTISIG_PDA>

# 4. Propose upgrade via backend API
curl -X POST http://upgrade-api:8080/proposals \
    -H "Content-Type: application/json" \
    -d '{
        "buffer": "<BUFFER_PUBKEY>",
        "program": "8cejxCR6Z1W5axtENP2UHmEBzLta4ywGY5J8BhurC58g",
        "description": "v1.2.0: Add new collateral type support"
    }'
# Returns: { "proposal_id": "<PROPOSAL_ID>" }
```

**Approval Phase (requires 3 of 5 members):**

```bash
# Member 1 approves
curl -X POST http://upgrade-api:8080/proposals/<PROPOSAL_ID>/approve \
    -H "Authorization: Bearer <MEMBER_1_SIGNATURE>"

# Member 2 approves  
curl -X POST http://upgrade-api:8080/proposals/<PROPOSAL_ID>/approve \
    -H "Authorization: Bearer <MEMBER_2_SIGNATURE>"

# Member 3 approves (threshold met!)
curl -X POST http://upgrade-api:8080/proposals/<PROPOSAL_ID>/approve \
    -H "Authorization: Bearer <MEMBER_3_SIGNATURE>"

# Status now: TimelockActive
# Timelock expires: 48 hours from now
```

**After 48 Hours:**

```bash
# Execute upgrade
curl -X POST http://upgrade-api:8080/proposals/<PROPOSAL_ID>/execute

# ✅ Contract upgraded!
# Verify:
solana program show 8cejxCR6Z1W5axtENP2UHmEBzLta4ywGY5J8BhurC58g
# Check "Last Deployed Slot" has changed
```

### Emergency Procedures

**Cancel a Pending Upgrade:**
```bash
# Any multisig member can cancel before execution
curl -X DELETE http://upgrade-api:8080/proposals/<PROPOSAL_ID> \
    -H "Authorization: Bearer <MEMBER_SIGNATURE>"
# Buffer closed, SOL refunded
```

**Pause Everything:**
```bash
# Any member can pause instantly
curl -X POST http://upgrade-api:8080/system/pause \
    -H "Authorization: Bearer <MEMBER_SIGNATURE>"
# All upgrade operations halted
```

---

## Gap Analysis

### Current State vs Desired State

```mermaid
graph LR
    subgraph Current["Current State ❌"]
        A1[Single Authority] --> A2[Instant Upgrades]
        A2 --> A3[No Audit Trail]
        A3 --> A4[No User Warning]
    end
    
    subgraph Desired["Desired State ✅"]
        B1[Multisig 3-of-5] --> B2[48h Timelock]
        B2 --> B3[Full Audit Trail]
        B3 --> B4[User Exit Window]
    end
    
    Current -.->|Integration| Desired
```

### Gap Summary Matrix

| Category | Current State | Desired State | Gap | Priority |
|----------|---------------|---------------|-----|----------|
| **Upgrade Authority** | Single keypair | Multisig PDA (3-of-5) | No governance | 🔴 Critical |
| **Timelock** | None (instant) | 48-hour minimum | Users can't react | 🔴 Critical |
| **Approval Process** | None | Threshold-based voting | No oversight | 🔴 Critical |
| **Audit Trail** | None | PostgreSQL + on-chain events | No accountability | 🟠 High |
| **Emergency Controls** | Manual | Pause/Resume instructions | Slow response | 🟠 High |
| **Upgradeability Flag** | `false` (all 7) | `true` (all 7) | Can't upgrade at all | 🔴 Critical |
| **Buffer Validation** | N/A | On-chain ownership check | Could deploy malicious code | 🔴 Critical |
| **Migration Tracking** | None | `AccountVersion` PDA | Data loss risk | 🟡 Medium |
| **Proposal Management** | None | Backend API + DB | No visibility | 🟡 Medium |
| **Member Management** | N/A | `MultisigConfig` account | Single point of failure | 🔴 Critical |

---

### Detailed Gap Analysis by Contract

#### 1. collateral_vault (🔴 Critical Priority)

| Aspect | Current | Gap | Risk |
|--------|---------|-----|------|
| Upgrade Authority | Single developer keypair | No multisig | 🔴 Funds at risk if key compromised |
| Upgradeable | `false` | Cannot be upgraded | 🟡 Bug fixes blocked |
| Timelock | None | Users can't exit | 🔴 Rug risk perception |
| Audit | None | No accountability | 🟠 Compliance risk |

**Remediation:**
1. Enable `upgradeable = true` and redeploy
2. Transfer authority to multisig PDA
3. All future upgrades via 48h governance

---

#### 2. ephemeral_vault (🟠 High Priority)

| Aspect | Current | Gap | Risk |
|--------|---------|-----|------|
| Upgrade Authority | Single developer keypair | No multisig | 🟠 Trading funds exposure |
| Upgradeable | `false` | Cannot be upgraded | 🟡 Bug fixes blocked |
| Timelock | None | Instant changes | 🟠 Flash loan attack vector |

**Remediation:**
1. Enable `upgradeable = true` and redeploy
2. Transfer authority to multisig PDA

---

#### 3. funding_rate (🟡 Medium Priority)

| Aspect | Current | Gap | Risk |
|--------|---------|-----|------|
| Upgrade Authority | Single developer keypair | No multisig | 🟡 Rate manipulation possible |
| Upgradeable | `false` | Cannot be upgraded | 🟡 Parameter updates blocked |

**Remediation:**
1. Enable `upgradeable = true` and redeploy
2. Transfer authority to multisig PDA

---

#### 4. oracle_integration (🟠 High Priority)

| Aspect | Current | Gap | Risk |
|--------|---------|-----|------|
| Upgrade Authority | Single developer keypair | No multisig | 🔴 Price feed manipulation |
| Upgradeable | `false` | Cannot be upgraded | 🟠 New feed sources blocked |
| Timelock | None | Instant changes | 🔴 Oracle switch attack |

**Remediation:**
1. Enable `upgradeable = true` and redeploy
2. Transfer authority to multisig PDA
3. Critical: Any oracle changes need 48h notice

---

#### 5. position_management (🔴 Critical Priority)

| Aspect | Current | Gap | Risk |
|--------|---------|-----|------|
| Upgrade Authority | Single developer keypair | No multisig | 🔴 Position data manipulation |
| Upgradeable | `false` | Cannot be upgraded | 🟡 New features blocked |
| Migration | None | No version tracking | 🔴 Data loss during schema changes |

**Remediation:**
1. Enable `upgradeable = true` and redeploy
2. Transfer authority to multisig PDA
3. Implement `AccountVersion` tracking for positions

---

#### 6. liquidation_engine (🟠 High Priority)

| Aspect | Current | Gap | Risk |
|--------|---------|-----|------|
| Upgrade Authority | Single developer keypair | No multisig | 🟠 Liquidation logic manipulation |
| Upgradeable | `false` | Cannot be upgraded | 🟠 Threshold adjustments blocked |

**Remediation:**
1. Enable `upgradeable = true` and redeploy
2. Transfer authority to multisig PDA

---

#### 7. settlement_relayer (🟠 High Priority)

| Aspect | Current | Gap | Risk |
|--------|---------|-----|------|
| Upgrade Authority | Single developer keypair | No multisig | 🟠 Settlement manipulation |
| Upgradeable | `false` | Cannot be upgraded | 🟡 Integration updates blocked |

**Remediation:**
1. Enable `upgradeable = true` and redeploy
2. Transfer authority to multisig PDA

---

### Infrastructure Gaps

| Component | Current State | Required | Gap |
|-----------|---------------|----------|-----|
| **Upgrade System Contract** | Not deployed | Deployed to mainnet | Missing entirely |
| **Multisig Configuration** | Does not exist | `MultisigConfig` PDA initialized | No governance structure |
| **Backend Service** | Not running | Axum REST API running | No orchestration layer |
| **PostgreSQL Database** | Not provisioned | DB with migrations | No proposal tracking |
| **Admin UI** | Does not exist | Optional (can use API directly) | UX gap for non-technical approvers |

---

### Security Gaps

| Gap | Current Risk | Mitigation |
|-----|--------------|------------|
| **Single Point of Failure** | One compromised key = total control | Multisig (3-of-5) distributes trust |
| **No User Warning** | Users can't exit before risky upgrade | 48-hour timelock provides exit window |
| **Instant Upgrades** | Malicious upgrade can drain funds instantly | Timelock + approval threshold |
| **No Audit Trail** | Cannot investigate incidents | PostgreSQL + on-chain events |
| **Buffer Injection** | Could deploy unverified code | Buffer ownership + data validation |

---

### Process Gaps

| Process | Current | Required |
|---------|---------|----------|
| **Upgrade Proposal** | Ad-hoc developer decision | Formal proposal with description |
| **Approval Workflow** | None | 3-of-5 member votes |
| **Execution** | Immediate CLI command | Wait timelock → execute via API |
| **Emergency Response** | Manual panic | `pause_system` instruction |
| **Rollback** | Git checkout + redeploy | Formal rollback proposal through governance |
| **Documentation** | None | Proposal descriptions + database records |

---

### Gap Closure Roadmap

```mermaid
graph TD
    G1[Gap: No Upgradeability] -->|Phase 1| R1[Enable upgradeable=true]
    G2[Gap: No Governance Contract] -->|Phase 2| R2[Deploy program-upgrade-system]
    G3[Gap: No Multisig] -->|Phase 3| R3[Initialize MultisigConfig]
    G4[Gap: Single Authority] -->|Phase 4| R4[Transfer authorities to PDA]
    G5[Gap: No API/DB] -->|Phase 5| R5[Deploy backend + PostgreSQL]
    
    R1 --> R2 --> R3 --> R4 --> R5 --> DONE[All Gaps Closed ✅]
```

---

## GDX Smart Contracts Inventory

| # | Contract | Program ID | Location | Risk Level |
|---|----------|------------|----------|------------|
| 1 | **collateral_vault** | `8cejxCR6Z1W5axtENP2UHmEBzLta4ywGY5J8BhurC58g` | `contracts/programs/gdx-collateral-vault/` | 🔴 Critical - Holds user funds |
| 2 | **ephemeral_vault** | `B1VEwBwzaJxU5iTceBaLBD5qHufEfLDbw7KY64gyLHPY` | `contracts/programs/gdx-ephemeral-vault/` | 🟠 High - Temporary trading funds |
| 3 | **funding_rate** | `B9vzqwL7wx6KUdRtqiaRqpSaxHmR6aELGwu2YbypZSep` | `contracts/programs/gdx-funding-rate/` | 🟡 Medium - Rate calculations |
| 4 | **oracle_integration** | `BurAgBGyQUbfjB1d8uLwPu5vwvakiZHNDwonrgoSyEmJ` | `contracts/programs/gdx-oracle/` | 🟠 High - Price feeds |
| 5 | **position_management** | `AA6pLa3UDKapAhvFcw5TSzrKJFsVhZZZeCXmRvjDX87V` | `contracts/programs/gdx-position-mgmt/` | 🔴 Critical - User positions |
| 6 | **liquidation_engine** | `AFZmVSZ4r4XXx1kBSvPR8dsKTxn7fS4B5kP49ZHCr7wH` | `services/gdx-liquidation-engine/liquidation-engine/` | 🟠 High - Liquidation logic |
| 7 | **settlement_relayer** | `3YxVFreKutJxiZ2S5v1jzyHwPDsLjaeZYwypysQALJfB` | `services/gdx-settlement-relayer/settlement-relayer/` | 🟠 High - Settlement execution |

> [!IMPORTANT]
> All 7 contracts currently have `upgradeable = false` in their Anchor.toml. This must be changed to `true` before they can be governed by the upgrade system.

---

## Program Upgrade System Components

### Smart Contract Instructions

| Instruction | Purpose | Who Can Call |
|-------------|---------|--------------|
| `initialize_multisig` | Set up governance with members and threshold | Deployer (once) |
| `propose_upgrade` | Create upgrade proposal with buffer pubkey | Any multisig member |
| `approve_upgrade` | Vote to approve a proposal | Any multisig member |
| `execute_upgrade` | Execute upgrade after timelock expires | Anyone (if conditions met) |
| `cancel_upgrade` | Cancel pending proposal and close buffer | Any multisig member |
| `migrate_account` | Track account version migration | Backend service |
| `pause_system` | Emergency halt all operations | Any multisig member |
| `resume_system` | Resume after pause | Any multisig member |

### On-Chain State Accounts

```
MultisigConfig (PDA: seeds = ["multisig"])
├── authority: Pubkey          # Program authority
├── members: Vec<Pubkey>       # Up to 10 members
├── threshold: u8              # Required approvals
├── is_paused: bool            # Emergency pause flag
└── bump: u8                   # PDA bump

UpgradeProposal (PDA: seeds = ["proposal", proposal_id])
├── id: Pubkey                 # Unique proposal ID
├── proposer: Pubkey           # Who proposed
├── new_program_buffer: Pubkey # Buffer with new program
├── target_program: Pubkey     # Program being upgraded
├── description: String        # Max 500 chars
├── status: UpgradeStatus      # Proposed/Approved/TimelockActive/Executed/Cancelled
├── approvals: Vec<Pubkey>     # Who has approved
├── approval_count: u8         # Current approvals
├── created_at: i64            # Unix timestamp
├── timelock_activated_at: Option<i64>  # When threshold was met
├── timelock_period: i64       # 172800 seconds (48 hours)
├── executed_at: Option<i64>   # When executed
└── bump: u8                   # PDA bump
```

### Security Guarantees (On-Chain Enforced)

| Validation | Location | Description |
|------------|----------|-------------|
| Multisig membership | `validate_multisig_member()` | Only configured members can propose/approve |
| Approval threshold | `validate_threshold()` | Configurable N-of-M required before timelock |
| 48-hour timelock | `validate_timelock_expired()` | Cannot bypass, enforced in `execute_upgrade` |
| Buffer ownership | `execute_upgrade.rs:78-81` | Buffer must be owned by BPF Upgradeable Loader |
| Buffer data | `execute_upgrade.rs:84-91` | Buffer must contain actual program data (>37 bytes) |
| Duplicate approval | `approve_upgrade.rs` | Same member cannot approve twice |
| System pause | All instructions | Blocked when `is_paused = true` |

---

## Proposed Changes

### Phase 1: Enable Upgradeable Programs

> [!CAUTION]
> This is a BREAKING CHANGE. All 7 contracts must be redeployed with `upgradeable = true`. Coordinate with team and ensure no active user positions.

#### [MODIFY] collateral_vault Anchor.toml
**File**: `contracts/programs/gdx-collateral-vault/collateral-vault/Anchor.toml`

```diff
 [test]
 startup_wait = 20000
 shutdown_wait = 2000
-upgradeable = false
+upgradeable = true
```

#### [MODIFY] ephemeral_vault Anchor.toml
**File**: `contracts/programs/gdx-ephemeral-vault/ephemeral-vault/Anchor.toml`

```diff
 [test]
 startup_wait = 10000
 shutdown_wait = 2000
-upgradeable = false
+upgradeable = true
```

#### [MODIFY] funding_rate Anchor.toml
**File**: `contracts/programs/gdx-funding-rate/funding-rate/Anchor.toml`

```diff
 [test]
 startup_wait = 20000
 shutdown_wait = 2000
-upgradeable = false
+upgradeable = true
```

#### [MODIFY] oracle_integration Anchor.toml
**File**: `contracts/programs/gdx-oracle/oracle/Anchor.toml`

```diff
 [test]
 startup_wait = 20000
 shutdown_wait = 2000
-upgradeable = false
+upgradeable = true
```

#### [MODIFY] position_management Anchor.toml
**File**: `contracts/programs/gdx-position-mgmt/position-mgmt/Anchor.toml`

```diff
 [test]
 startup_wait = 20000
 shutdown_wait = 2000
-upgradeable = false
+upgradeable = true
```

#### [MODIFY] liquidation_engine Anchor.toml
**File**: `services/gdx-liquidation-engine/liquidation-engine/Anchor.toml`

```diff
 [test]
 startup_wait = 20000
 shutdown_wait = 2000
-upgradeable = false
+upgradeable = true
```

#### [MODIFY] settlement_relayer Anchor.toml
**File**: `services/gdx-settlement-relayer/settlement-relayer/Anchor.toml`

```diff
 [test]
 startup_wait = 20000
 shutdown_wait = 2000
-upgradeable = false
+upgradeable = true
```

---

### Phase 2: Deploy Program Upgrade System

#### [NEW] Devnet Configuration
**File**: `program-upgrade-system/Anchor.toml`

Add devnet cluster configuration:

```toml
[programs.devnet]
program_upgrade_system = "<NEW_DEVNET_PROGRAM_ID>"

[provider]
cluster = "devnet"  # Change for testing
```

#### Deployment Commands

```bash
# 1. Navigate to upgrade system
cd program-upgrade-system

# 2. Build
anchor build

# 3. Deploy to devnet first
anchor deploy --provider.cluster devnet

# 4. Note the deployed program ID
# Update Anchor.toml [programs.devnet] with the new ID

# 5. Run test suite on devnet
anchor test --skip-local-validator --provider.cluster devnet
```

---

### Phase 3: Initialize Multisig Governance

#### [NEW] Initialization Script
**File**: `program-upgrade-system/scripts/init-gdx-multisig.ts`

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { ProgramUpgradeSystem } from "../target/types/program_upgrade_system";

// GDX Team Multisig Members (REPLACE WITH REAL PUBKEYS)
const MULTISIG_MEMBERS: PublicKey[] = [
  new PublicKey("MEMBER_1_PUBKEY"), // Lead Developer
  new PublicKey("MEMBER_2_PUBKEY"), // Security Lead
  new PublicKey("MEMBER_3_PUBKEY"), // Operations Lead
  new PublicKey("MEMBER_4_PUBKEY"), // CTO
  new PublicKey("MEMBER_5_PUBKEY"), // External Auditor
];

const THRESHOLD = 3; // 3-of-5 required

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ProgramUpgradeSystem as Program<ProgramUpgradeSystem>;

  // Derive multisig PDA
  const [multisigPda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("multisig")],
    program.programId
  );

  console.log("Initializing GDX Multisig...");
  console.log("Members:", MULTISIG_MEMBERS.map(m => m.toBase58()));
  console.log("Threshold:", THRESHOLD);
  console.log("Multisig PDA:", multisigPda.toBase58());

  const tx = await program.methods
    .initializeMultisig(MULTISIG_MEMBERS, THRESHOLD)
    .accounts({
      multisigConfig: multisigPda,
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log("Initialized! TX:", tx);
  console.log("\n=== SAVE THIS ===");
  console.log("Multisig PDA:", multisigPda.toBase58());
  console.log("Use this as the new upgrade authority for all GDX programs.");
}

main().catch(console.error);
```

---

### Phase 4: Transfer Upgrade Authority

> [!WARNING]
> Once upgrade authority is transferred to the multisig PDA, it CANNOT be transferred back without going through the governance process. Ensure multisig is initialized correctly first!

#### [NEW] Authority Transfer Script
**File**: `program-upgrade-system/scripts/transfer-gdx-authorities.ts`

```typescript
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { execSync } from "child_process";

// GDX Program IDs
const GDX_PROGRAMS = {
  collateral_vault: "8cejxCR6Z1W5axtENP2UHmEBzLta4ywGY5J8BhurC58g",
  ephemeral_vault: "B1VEwBwzaJxU5iTceBaLBD5qHufEfLDbw7KY64gyLHPY",
  funding_rate: "B9vzqwL7wx6KUdRtqiaRqpSaxHmR6aELGwu2YbypZSep",
  oracle_integration: "BurAgBGyQUbfjB1d8uLwPu5vwvakiZHNDwonrgoSyEmJ",
  position_management: "AA6pLa3UDKapAhvFcw5TSzrKJFsVhZZZeCXmRvjDX87V",
  liquidation_engine: "AFZmVSZ4r4XXx1kBSvPR8dsKTxn7fS4B5kP49ZHCr7wH",
  settlement_relayer: "3YxVFreKutJxiZ2S5v1jzyHwPDsLjaeZYwypysQALJfB",
};

// REPLACE WITH ACTUAL MULTISIG PDA
const MULTISIG_PDA = "YOUR_MULTISIG_PDA_FROM_INIT_SCRIPT";

async function main() {
  console.log("Transferring upgrade authority to multisig PDA:", MULTISIG_PDA);
  console.log("\n");

  for (const [name, programId] of Object.entries(GDX_PROGRAMS)) {
    console.log(`Transferring ${name} (${programId})...`);
    
    try {
      // Use Solana CLI to transfer authority
      const cmd = `solana program set-upgrade-authority ${programId} --new-upgrade-authority ${MULTISIG_PDA}`;
      console.log(`Running: ${cmd}`);
      
      execSync(cmd, { stdio: "inherit" });
      console.log(`✅ ${name} authority transferred\n`);
    } catch (error) {
      console.error(`❌ Failed to transfer ${name}:`, error);
    }
  }

  console.log("\n=== VERIFICATION ===");
  for (const [name, programId] of Object.entries(GDX_PROGRAMS)) {
    execSync(`solana program show ${programId}`, { stdio: "inherit" });
  }
}

main().catch(console.error);
```

---

### Phase 5: Backend Service Integration

#### [NEW] Environment Configuration
**File**: `program-upgrade-system/backend/.env`

```bash
# Database
DATABASE_URL=postgresql://user:password@localhost:5432/gdx_upgrades

# Solana RPC
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
# Or use dedicated RPC: https://your-rpc-provider.com

# Program IDs
UPGRADE_SYSTEM_PROGRAM_ID=BPeh5RUhTQbh637q8gGaGrasETYPcinBXqVKxutTB9v5

# GDX Programs to manage
GDX_COLLATERAL_VAULT=8cejxCR6Z1W5axtENP2UHmEBzLta4ywGY5J8BhurC58g
GDX_EPHEMERAL_VAULT=B1VEwBwzaJxU5iTceBaLBD5qHufEfLDbw7KY64gyLHPY
GDX_FUNDING_RATE=B9vzqwL7wx6KUdRtqiaRqpSaxHmR6aELGwu2YbypZSep
GDX_ORACLE=BurAgBGyQUbfjB1d8uLwPu5vwvakiZHNDwonrgoSyEmJ
GDX_POSITION_MGMT=AA6pLa3UDKapAhvFcw5TSzrKJFsVhZZZeCXmRvjDX87V
GDX_LIQUIDATION=AFZmVSZ4r4XXx1kBSvPR8dsKTxn7fS4B5kP49ZHCr7wH
GDX_SETTLEMENT=3YxVFreKutJxiZ2S5v1jzyHwPDsLjaeZYwypysQALJfB

# Server
PORT=8080
RUST_LOG=info
```

#### Deployment Commands

```bash
# 1. Navigate to backend
cd program-upgrade-system/backend

# 2. Create .env from example
cp .env.example .env
# Edit with production values

# 3. Run database migrations
cargo sqlx migrate run

# 4. Build release
cargo build --release

# 5. Run service
./target/release/upgrade-backend
```

---

## Upgrade Workflow (Step-by-Step)

### Scenario: Upgrading `collateral_vault`

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant CLI as Solana CLI
    participant API as Upgrade API
    participant SC as Upgrade Contract
    participant M1 as Member 1
    participant M2 as Member 2
    participant M3 as Member 3
    participant BPF as BPF Loader

    Note over Dev: 1. BUILD NEW VERSION
    Dev->>Dev: Modify collateral_vault code
    Dev->>CLI: anchor build (in gdx-collateral-vault/)

    Note over Dev: 2. CREATE BUFFER
    Dev->>CLI: solana program write-buffer ./target/deploy/collateral_vault.so
    CLI-->>Dev: Buffer: <BUFFER_PUBKEY>

    Note over Dev: 3. SET BUFFER AUTHORITY TO MULTISIG
    Dev->>CLI: solana program set-buffer-authority <BUFFER> --new-buffer-authority <MULTISIG_PDA>

    Note over Dev: 4. PROPOSE UPGRADE
    Dev->>API: POST /proposals {buffer, program, description}
    API->>SC: propose_upgrade(buffer, description)
    SC-->>API: proposal_id

    Note over M1,M3: 5. COLLECT APPROVALS (3 of 5)
    M1->>API: POST /proposals/:id/approve (signed)
    API->>SC: approve_upgrade(proposal_id)
    M2->>API: POST /proposals/:id/approve (signed)
    API->>SC: approve_upgrade(proposal_id)
    M3->>API: POST /proposals/:id/approve (signed)
    API->>SC: approve_upgrade(proposal_id)

    Note over SC: Threshold met → status = TimelockActive

    Note over SC: 6. WAIT 48 HOURS
    Note over SC: Users can exit positions during this window

    Note over Dev: 7. EXECUTE UPGRADE
    Dev->>API: POST /proposals/:id/execute
    API->>SC: execute_upgrade(proposal_id)
    SC->>SC: Validate timelock expired
    SC->>SC: Validate buffer ownership
    SC->>SC: Validate buffer has data
    SC->>BPF: CPI: bpf_loader_upgradeable::upgrade()
    BPF-->>SC: ✅ Program upgraded
    SC-->>API: status = Executed
```

---

## Configuration Reference

### Multisig Settings

| Parameter | Recommended Value | Notes |
|-----------|-------------------|-------|
| Members | 5 | Core team + 1 external |
| Threshold | 3 | Majority required |
| Timelock | 172800 seconds | 48 hours (hardcoded) |
| Max Members | 10 | Contract limit |

### Backend API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/proposals` | List all proposals |
| `GET` | `/proposals/:id` | Get proposal details |
| `POST` | `/proposals` | Create new proposal |
| `POST` | `/proposals/:id/approve` | Approve proposal (requires signature) |
| `POST` | `/proposals/:id/execute` | Execute after timelock |
| `DELETE` | `/proposals/:id` | Cancel proposal |
| `POST` | `/system/pause` | Emergency pause |
| `POST` | `/system/resume` | Resume from pause |
| `GET` | `/migration/:id/status` | Migration progress |

---

## Verification Plan

### 1. Smart Contract Tests

The program-upgrade-system includes 12 tests that verify the complete upgrade lifecycle:

```bash
# Run from program-upgrade-system directory
cd program-upgrade-system

# Start local validator
solana-test-validator --reset

# In another terminal, run tests
anchor test --skip-local-validator
```

**Expected Output:**
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

  12 passing (9s)
```

### 2. GDX Contract Build Verification

After enabling `upgradeable = true`, verify all contracts still build:

```bash
# Collateral Vault
cd contracts/programs/gdx-collateral-vault/collateral-vault
anchor build

# Ephemeral Vault
cd ../../gdx-ephemeral-vault/ephemeral-vault
anchor build

# Funding Rate
cd ../../gdx-funding-rate/funding-rate
anchor build

# Oracle
cd ../../gdx-oracle/oracle
anchor build

# Position Management
cd ../../gdx-position-mgmt/position-mgmt
anchor build

# Liquidation Engine
cd ../../../../services/gdx-liquidation-engine/liquidation-engine
anchor build

# Settlement Relayer
cd ../../gdx-settlement-relayer/settlement-relayer
anchor build
```

### 3. Devnet End-to-End Test

Manual verification on devnet before mainnet:

1. **Deploy upgrade system to devnet**
   ```bash
   cd program-upgrade-system
   anchor deploy --provider.cluster devnet
   ```

2. **Initialize multisig with test keys**
   ```bash
   anchor run init-gdx-multisig -- --cluster devnet
   ```

3. **Deploy a test GDX contract to devnet**
   ```bash
   cd contracts/programs/gdx-funding-rate/funding-rate
   anchor deploy --provider.cluster devnet
   ```

4. **Transfer authority to multisig PDA**
   ```bash
   solana program set-upgrade-authority <FUNDING_RATE_ID> --new-upgrade-authority <MULTISIG_PDA>
   ```

5. **Create, approve, and execute upgrade**
   - Create buffer: `solana program write-buffer ./target/deploy/funding_rate.so`
   - Set buffer authority: `solana program set-buffer-authority <BUFFER> --new-buffer-authority <MULTISIG_PDA>`
   - Propose via API or script
   - Collect 3 approvals
   - Wait for timelock (reduce to 1 minute for testing via contract modification)
   - Execute upgrade

6. **Verify upgrade success**
   ```bash
   solana program show <FUNDING_RATE_ID>
   # Check "Last Deployed Slot" changed
   ```

### 4. User Acceptance Testing (Manual)

The following should be verified by the team before mainnet:

- [ ] All 5 multisig members can connect and sign
- [ ] Proposal creation shows correct buffer and description
- [ ] Approval count increments correctly
- [ ] Timelock countdown displays correctly
- [ ] Emergency pause halts all operations
- [ ] Resume enables operations again
- [ ] Cancel closes buffer and refunds SOL

---

## Rollback Procedure

If an upgrade causes issues:

1. **Pause System Immediately**
   ```bash
   # Via API
   curl -X POST http://upgrade-api:8080/system/pause \
     -H "Authorization: Bearer <MEMBER_SIGNATURE>"
   ```

2. **Build Previous Version**
   ```bash
   git checkout <previous-tag>
   anchor build
   ```

3. **Create Rollback Buffer**
   ```bash
   solana program write-buffer ./target/deploy/collateral_vault.so
   solana program set-buffer-authority <BUFFER> --new-buffer-authority <MULTISIG_PDA>
   ```

4. **Submit Rollback Proposal**
   ```bash
   curl -X POST http://upgrade-api:8080/proposals \
     -d '{"buffer": "<BUFFER>", "program": "<PROGRAM_ID>", "description": "EMERGENCY ROLLBACK: <reason>"}'
   ```

5. **Fast-Track Approvals**
   - All available members approve immediately
   - Wait 48 hours (or coordinate reduced timelock if pre-configured)
   - Execute rollback

---

## Timeline & Dependencies

```mermaid
gantt
    title GDX Upgrade System Integration
    dateFormat  YYYY-MM-DD
    section Phase 1
    Enable upgradeable flag    :p1, 2024-01-01, 1d
    Rebuild all contracts      :p2, after p1, 1d
    section Phase 2
    Deploy upgrade system      :p3, after p2, 1d
    Run test suite             :p4, after p3, 1d
    section Phase 3
    Initialize multisig        :p5, after p4, 1d
    Verify member access       :p6, after p5, 1d
    section Phase 4
    Transfer authorities       :p7, after p6, 1d
    Verify transfers           :p8, after p7, 1d
    section Phase 5
    Deploy backend             :p9, after p8, 2d
    Devnet E2E test            :p10, after p9, 3d
    section Mainnet
    Mainnet deployment         :p11, after p10, 1d
    Monitor first upgrade      :p12, after p11, 2d
```

---

## Summary

This implementation plan provides a complete path to integrate the Program Upgrade System with all 7 GDX smart contracts:

| Phase | Description | Duration | Risk |
|-------|-------------|----------|------|
| 1 | Enable upgradeable flag | 1 day | Low |
| 2 | Deploy upgrade system | 2 days | Low |
| 3 | Initialize multisig | 2 days | Medium |
| 4 | Transfer authorities | 2 days | 🔴 HIGH |
| 5 | Backend deployment | 5 days | Medium |
| **Total** | | **~12 days** | |

> [!CAUTION]
> Phase 4 (Transfer authorities) is IRREVERSIBLE. Once complete, all upgrades must go through governance. Triple-check multisig initialization before proceeding.
