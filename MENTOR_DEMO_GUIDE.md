# Mentor Demo Guide: Program Upgrade System

This guide provides step-by-step instructions to manually run and demonstrate the Solana Program Upgrade System. It includes explanations for each step to help you present it to your mentor.

## Prerequisites
- Terminal open in: `/Users/dhruvmishra/UPGRADECPI/program-upgrade-system`
- Ensure no other validators are running (`pkill -f solana-test-validator`).

## Step 1: Start the Local Validator (The Blockchain)
We run a local Solana cluster to test our program without spending real money.
**Note**: We use a specific command to avoid macOS file system errors (`._genesis.bin`).

```bash
# 1. Create a clean directory for the validator to avoid file conflicts
mkdir -p validator-run
cd validator-run

# 2. Start the validator with metadata file creation disabled
export COPYFILE_DISABLE=1
solana-test-validator --reset
```
> **Why a new directory?**
> On macOS, the file system creates hidden metadata files (starting with `._`) that confuse the Solana validator when it tries to create the ledger (the database of the blockchain). By creating a clean `validator-run` folder and using `COPYFILE_DISABLE=1`, we ensure these hidden files don't crash the startup.

*Keep this terminal open. Open a new terminal for the next steps.*

## Step 2: Build the Program
We compile the Rust code into a BPF (Berkeley Packet Filter) binary that runs on Solana.

```bash
# In a NEW terminal, go to the project root
cd /Users/dhruvmishra/UPGRADECPI/program-upgrade-system

# Build using Anchor
anchor build
```
**Explanation**: This uses the Anchor framework to compile `programs/program-upgrade-system/src/lib.rs` into `target/deploy/program_upgrade_system.so`.

## Step 3: Deploy the Program
We upload the compiled binary to our local blockchain.

```bash
# Deploy using Solana CLI
solana program deploy target/deploy/program_upgrade_system.so \
  --program-id target/deploy/program_upgrade_system-keypair.json \
  --url http://127.0.0.1:8899
```
**Explanation**:
- We use `solana program deploy` instead of `anchor deploy` to have direct control over the Program ID.
- **Program ID**: `F4rYDGUKQHtJt14aPyGxtzacx9x7x9MH7rper2TuRdVz` (This is the address of your smart contract).

## Step 4: Run the Integration Tests
We run TypeScript tests to verify the logic (Initialize -> Propose -> Approve).

```bash
# Run tests using ts-mocha
env ANCHOR_PROVIDER_URL=http://127.0.0.1:8899 \
    ANCHOR_WALLET=/Users/dhruvmishra/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts
```
**Explanation**:
- We manually set the `ANCHOR_PROVIDER_URL` to point to our local validator.
- The tests connect to the deployed program and execute transactions to test the "Multisig" logic.

## Key Concepts to Explain
1.  **Multisig Authority**: The program is controlled by a "Multisig" (multiple signatures). No single person can upgrade it; it requires approval from members.
2.  **PDA (Program Derived Address)**: We use PDAs for the `MultisigConfig` and `UpgradeProposal` accounts. This ensures that only this specific program can modify them, making it secure.
3.  **Upgrade Flow**:
    - **Propose**: A member proposes a new program buffer (the new code).
    - **Approve**: Other members vote to approve it.
    - **Execute**: Once enough votes are cast (threshold met), the upgrade happens.

## Cleanup
If you need to stop the validator (e.g., to restart it or because it's stuck), run this command:

```bash
pkill -f solana-test-validator
```
**Note**: This forcefully stops any running validator processes.
