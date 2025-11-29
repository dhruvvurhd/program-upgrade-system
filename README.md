# Solana Program Upgrade System

A secure, multisig-controlled upgrade system for Solana programs. This project demonstrates how to implement a secure governance mechanism for upgrading smart contracts using Cross-Program Invocations (CPI) to the BPF Loader Upgradeable.

## 🚀 Features

- **Multisig Control**: Program upgrades require approval from a threshold of authorized members.
- **Secure Upgrades**: Uses CPI to the BPF Loader Upgradeable program to execute upgrades safely.
- **Timelock Mechanism**: Optional delay between approval and execution for added security.
- **Full Test Coverage**: Integration tests covering the entire workflow (Initialize -> Propose -> Approve).

## 🛠 Architecture

The system consists of three main instructions:
1.  **`InitializeMultisig`**: Sets up the governance config (members, threshold).
2.  **`ProposeUpgrade`**: A member proposes a new program buffer (compiled code).
3.  **`ApproveUpgrade`**: Members vote. Once the threshold is met, the upgrade is approved.
4.  **`ExecuteUpgrade`**: The program executes the upgrade via CPI.

## 📋 Prerequisites

- **Rust**: 1.75.0 (Required for Solana 1.18 compatibility)
- **Solana CLI**: 1.18.26
- **Anchor CLI**: 0.28.0 (or compatible version)
- **Node.js**: 18+

## 🏃‍♂️ Quick Start (Demo Guide)

Follow these steps to run the project locally.

### 1. Start the Local Validator
**Note for macOS Users**: We use a specific command to avoid file system errors (`._genesis.bin`).

```bash
# Create a clean directory for the validator
mkdir -p validator-run
cd validator-run

# Start validator with metadata file creation disabled
export COPYFILE_DISABLE=1
solana-test-validator --reset
```
*Keep this terminal open.*

### 2. Build the Program
In a new terminal, navigate to the project root:

```bash
# Build the program binary
anchor build
```

### 3. Deploy the Program
We use the Solana CLI for precise control over the Program ID.

```bash
solana program deploy target/deploy/program_upgrade_system.so \
  --program-id target/deploy/program_upgrade_system-keypair.json \
  --url http://127.0.0.1:8899
```

### 4. Run Tests
Verify the system works as expected.

```bash
env ANCHOR_PROVIDER_URL=http://127.0.0.1:8899 \
    ANCHOR_WALLET=$HOME/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts
```

## ✅ Verification Results

The latest test run confirmed:
- [x] Multisig Initialization
- [x] Upgrade Proposal Creation
- [x] Approval Workflow
- [x] PDA Derivation & Security Checks

## 🔧 Troubleshooting

**Validator fails to start?**
Ensure you are running it from the `validator-run` directory with `COPYFILE_DISABLE=1`.

**Program ID Mismatch?**
If you see a mismatch error, ensure you redeploy using the command in Step 3.

## 📜 License
MIT
