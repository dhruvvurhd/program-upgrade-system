# Program Upgrade System - Manual Testing Guide

This guide walks you through manually testing each function of the Program Upgrade System.

## Prerequisites

```bash
# Terminal 1: Start local validator
cd /Users/dhruvmishra/UPGRADECPI/program-upgrade-system
solana-test-validator -r --quiet

# Terminal 2: Build and deploy
anchor build
anchor deploy
```

---

## Step 1: Initialize Multisig

**Purpose**: Create the governance structure (members + voting threshold).

### Run Test
```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');
const { programUpgradeSystem } = require('./target/types/program_upgrade_system');

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  const authority = anchor.getProvider().publicKey;
  
  const [multisigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('multisig')],
    program.programId
  );
  
  console.log('Multisig PDA:', multisigPda.toBase58());
  
  const tx = await program.methods
    .initializeMultisig([authority], 1)
    .accounts({
      multisigConfig: multisigPda,
      authority: authority,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  
  console.log('✅ Initialized! Tx:', tx);
}
main();
"
```

### Expected Output
```
Multisig PDA: Fy1my1YyQmjkVF8zDJD7rS8tUePPaeBAXnPsQbTxH8QF
✅ Initialized! Tx: 5FFTdFWA8u4BsKezypGXMBeTTB1hruPgSNWnUcn1eAN...
```

---

## Step 2: Propose Upgrade

**Purpose**: Create a new upgrade proposal.

### Run Test
```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  const authority = anchor.getProvider().publicKey;
  
  const buffer = anchor.web3.Keypair.generate().publicKey;
  
  const [multisigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('multisig')],
    program.programId
  );
  
  const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('proposal'), buffer.toBuffer()],
    program.programId
  );
  
  console.log('Buffer:', buffer.toBase58());
  console.log('Proposal PDA:', proposalPda.toBase58());
  
  const tx = await program.methods
    .proposeUpgrade(buffer, 'Upgrade to v2.0 with new features')
    .accounts({
      proposal: proposalPda,
      multisigConfig: multisigPda,
      proposer: authority,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  
  console.log('✅ Proposed! Tx:', tx);
  console.log('');
  console.log('SAVE THIS: Proposal PDA =', proposalPda.toBase58());
}
main();
"
```

### Expected Output
```
Buffer: 4wJ6ysndCHtW3YTFaTDJVap72UC5jvYbsEDoFWUZwWRe
Proposal PDA: 8p2PRjkJmKr4qSA4bQSz7VknrXkJJh7nimLtj19H5Gnt
✅ Proposed! Tx: 4QFoh...
SAVE THIS: Proposal PDA = 8p2PRjkJmKr4qSA4bQSz7VknrXkJJh7nimLtj19H5Gnt
```

---

## Step 3: Approve Upgrade

**Purpose**: Vote to approve the proposal.

### Run Test
```bash
# Replace PROPOSAL_PDA with the one from Step 2
PROPOSAL_PDA="8p2PRjkJmKr4qSA4bQSz7VknrXkJJh7nimLtj19H5Gnt"

npx ts-node -e "
const anchor = require('@coral-xyz/anchor');

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  const authority = anchor.getProvider().publicKey;
  
  const proposalPda = new anchor.web3.PublicKey('$PROPOSAL_PDA');
  
  const [multisigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('multisig')],
    program.programId
  );
  
  const tx = await program.methods
    .approveUpgrade(proposalPda)
    .accounts({
      proposal: proposalPda,
      multisigConfig: multisigPda,
      approver: authority,
    })
    .rpc();
  
  console.log('✅ Approved! Tx:', tx);
  
  // Check status
  const proposal = await program.account.upgradeProposal.fetch(proposalPda);
  console.log('Status:', proposal.status);
  console.log('Approvals:', proposal.approvals.length);
}
main();
"
```

### Expected Output
```
✅ Approved! Tx: xXQZUC5sRWWQqqgpRiGjmRG6vWgbq9Xi6BcKdxurmmpS...
Status: { timelockActive: {} }
Approvals: 1
```

---

## Step 4: Check Proposal Status

**Purpose**: View current state of a proposal.

```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  
  const proposalPda = new anchor.web3.PublicKey('$PROPOSAL_PDA');
  
  const proposal = await program.account.upgradeProposal.fetch(proposalPda);
  
  console.log('=== Proposal Status ===');
  console.log('Status:', JSON.stringify(proposal.status));
  console.log('Description:', proposal.description);
  console.log('Approvals:', proposal.approvals.length);
  console.log('Created:', new Date(proposal.createdAt.toNumber() * 1000));
}
main();
"
```

---

## Step 5: Pause System (Emergency)

**Purpose**: Emergency pause all operations.

```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  const authority = anchor.getProvider().publicKey;
  
  const [multisigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('multisig')],
    program.programId
  );
  
  const tx = await program.methods
    .pauseSystem()
    .accounts({
      multisigConfig: multisigPda,
      pauser: authority,
    })
    .rpc();
  
  console.log('🛑 System PAUSED! Tx:', tx);
}
main();
"
```

---

## Step 6: Resume System

**Purpose**: Resume after emergency pause.

```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  const authority = anchor.getProvider().publicKey;
  
  const [multisigPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('multisig')],
    program.programId
  );
  
  const tx = await program.methods
    .resumeSystem()
    .accounts({
      multisigConfig: multisigPda,
      resumer: authority,
    })
    .rpc();
  
  console.log('🟢 System RESUMED! Tx:', tx);
}
main();
"
```

---

## Quick Test: Run All Automated Tests

```bash
# Make sure validator is running first
anchor test --skip-local-validator
```

Expected: `12 passing`

---

## Error Reference

| Error | Meaning | Fix |
|-------|---------|-----|
| `TimelockNotExpired` | Must wait 48h | Wait or use cancel |
| `InvalidProposalState` | Wrong status | Check current status |
| `ProposalAlreadyCancelled` | Already cancelled | Create new proposal |
| `SystemAlreadyPaused` | Already paused | Run resume first |
| `NotAMember` | Not a multisig member | Use correct wallet |
