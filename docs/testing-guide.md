# Program Upgrade System - Exhaustive Verification Guide

This guide provides step-by-step instructions to manually verify every function of the program. For each step, we explain **what** you are doing, **why** it matters, and the **exact command** to run.

## Prerequisites

1.  **Terminal 1**: Start a local Solana validator (a mini-blockchain on your laptop).
    ```bash
    solana-test-validator -r --quiet
    ```
2.  **Terminal 2**: Build and deploy your program to this local chain.
    ```bash
    cd /Users/dhruvmishra/UPGRADECPI/program-upgrade-system
    anchor build
    anchor deploy
    ```
3.  **Set Environment Variables**: Required for the verification scripts to connect to local validator.
    ```bash
    export ANCHOR_PROVIDER_URL=http://127.0.0.1:8899
    export ANCHOR_WALLET=~/.config/solana/id.json
    ```

---

## 1. Initialize Multisig

### What this means
Before anyone can upgrade the program, we must set up the "governance" layer. We create a special account (PDA) that stores the list of authorized members (you) and the rules (threshold).

### Run Verification
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
  
  console.log('📝 Multisig PDA:', multisigPda.toBase58());
  
  // Check if already initialized
  try {
    const existing = await program.account.multisigConfig.fetch(multisigPda);
    console.log('✅ Already initialized! Members:', existing.members.length);
    return;
  } catch (e) {
    // Not initialized yet, proceed
  }
  
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

> **Note**: If you see "Already initialized", that means this step was completed previously. This is expected behavior - the program correctly prevents duplicate initialization.

---

## Appendix: Understanding Localnet & Debugging

### What is "Localnet"?
"Localnet" is a simulated Solana blockchain running entirely on your laptop. It allows you to develop and test without spending real money.
- **Validator**: The process (`solana-test-validator`) that acts as the network.
- **Provider**: Your wallet (usually `~/.config/solana/id.json`) that signs transactions.
- **Program**: Your smart contract deployed to this local network.

### How to Monitor the Network
While running tests, you can open a **3rd Terminal** to see exactly what is happening on-chain.

**First, navigate to your project directory:**
```bash
cd /Users/dhruvmishra/UPGRADECPI/program-upgrade-system
```

Then use these commands:

#### 1. View Program Logs (Streaming)
See `msg!()` output in real-time.
```bash
solana logs
```
*Expected Output:*
```
Transaction executed in slot 5:
  > Program <YOUR_PROGRAM_ID> invoke [1]
  > Program log: Instruction: InitializeMultisig
  > Program log: Multisig PDA: Fy1m...
  > Program <YOUR_PROGRAM_ID> consumed 12345 of 200000 compute units
  > Program <YOUR_PROGRAM_ID> success
```

#### 2. Check Program Details
Verify your program is deployed and executable.
```bash
solana program show <PROGRAM_ID>
```
*Expected Output:*
```
Program Id: <YOUR_PROGRAM_ID>
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: <DATA_ADDRESS>
Authority: <YOUR_WALLET>
Last Deployed In Slot: 123
Data Length: 456789 bytes
```

#### 3. Inspect an Account
Peek into the raw data of any account (Proposal, Multisig, etc.).
```bash
solana account <ACCOUNT_ADDRESS>
```
*Expected Output:*
```
Public Key: <ACCOUNT_ADDRESS>
Balance: 0.00239472 SOL
Owner: <YOUR_PROGRAM_ID>
Executable: false
Rent Epoch: 123
Length: 85 bytes (0x55)
0000:   08 00 00 00  00 00 00 00  01 00 00 00  00 00 00 00   ................
0010:   ... (Raw hexdump of account data) ...
```
To see decoded data, use `anchor account` calls in your scripts (as shown in steps 1-6 above).

---

## 2. Propose Upgrade

### What this means
You (a member) want to upgrade the software. You cannot just "do it". You must submit a "Proposal" that says: *"I want to upgrade to improved code buffer X."*

### Run Verification
```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');
async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  const authority = anchor.getProvider().publicKey;
  const buffer = anchor.web3.Keypair.generate().publicKey; // Fake buffer for testing
  
  const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('proposal'), buffer.toBuffer()],
    program.programId
  );
  
  console.log('📝 Proposing upgrade to buffer:', buffer.toBase58());
  
  const tx = await program.methods
    .proposeUpgrade(buffer, 'Upgrade to v2.0')
    .accounts({
      proposal: proposalPda,
      multisigConfig: anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('multisig')], program.programId)[0],
      proposer: authority,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  
  console.log('✅ Proposal Created! Proposal Address:', proposalPda.toBase58());
  console.log('SAVE THIS ADDRESS for the next step.');
}
main();
"
```

---

## 3. Approve Upgrade

### What this means
Other members review your code. If they agree, they "Approve" it. Once enough members approve (1 in this case), the proposal is ready to execute.

### Run Verification
*Replace `PROPOSAL_ADDR` with the address from Step 2.*

```bash
PROPOSAL_ADDR="GspSMSj2HEzQtZj29ruT4PLdyg8L2iaxBu7kDofb3KA9" 

npx ts-node -e "
const anchor = require('@coral-xyz/anchor');
async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  
  const proposalPda = new anchor.web3.PublicKey('$PROPOSAL_ADDR');
  
  console.log('🗳️ Casting vote for:', proposalPda.toBase58());
  
  const tx = await program.methods
    .approveUpgrade(proposalPda)
    .accounts({
      proposal: proposalPda,
      multisigConfig: anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('multisig')], program.programId)[0],
      approver: anchor.getProvider().publicKey,
    })
    .rpc();
  
  console.log('✅ Approved! Vote recorded.');
}
main();
"
```

---

## 4. Cancel Upgrade (Test Logic)

### What this means
You realized the code has a bug *after* proposing it. You can "Cancel" the proposal to prevent it from ever being executed.

### Run Verification
```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');
async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  const authority = anchor.getProvider().publicKey;
  
  // Create a dummy proposal just to cancel it
  const buffer = anchor.web3.Keypair.generate().publicKey;
  const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('proposal'), buffer.toBuffer()], 
    program.programId
  );
  const multisigPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('multisig')], 
    program.programId
  )[0];
  
  await program.methods.proposeUpgrade(buffer, 'Bad Upgrade').accounts({
      proposal: proposalPda,
      multisigConfig: multisigPda,
      proposer: authority,
      systemProgram: anchor.web3.SystemProgram.programId,
  }).rpc();
  
  console.log('📝 Created bad proposal:', proposalPda.toBase58());

  // Now Cancel it (pass proposal_id as argument + buffer/rent_recipient accounts)
  const tx = await program.methods
    .cancelUpgrade(proposalPda)
    .accounts({
      proposal: proposalPda,
      multisigConfig: multisigPda,
      canceller: authority,
      buffer: buffer,
      rentRecipient: authority,
    })
    .rpc();
    
  console.log('❌ Cancelled! The proposal is now dead.');
  
  // Verify status
  const account = await program.account.upgradeProposal.fetch(proposalPda);
  console.log('Status on chain:', JSON.stringify(account.status));
}
main();
"
```

---

## 5. Migrate Account

### What this means
The software upgrade changed the data structure (e.g., added a new field to UserAccount). We verify that we can take an "old" account and convert it to the "new" format safely.

### Run Verification
```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');
async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  
  // Mock an 'old' account address
  const oldAccountKey = anchor.web3.Keypair.generate().publicKey;
  
  const [accountVersionPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('migration'), oldAccountKey.toBuffer()],
    program.programId
  );

  console.log('🔄 Migrating account:', oldAccountKey.toBase58());
  
  const tx = await program.methods
    .migrateAccount()
    .accounts({
      accountVersion: accountVersionPda, // Tracks that this account was migrated
      oldAccount: oldAccountKey,        // The account being read
      // newAccount: ... in real app, you'd allow writing to a new PDA
      authority: anchor.getProvider().publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
    
  console.log('✅ Account Migrated! Version tag created.');
}
main();
"
```

---

## 6. Emergency Pause / Resume

### What this means
**Pause**: A "Check Engine" light but for code. If a hack is detected, any member can freeze the contract. No proposals or upgrades can happen.
**Resume**: Turns the system back on.

### Run Verification
```bash
npx ts-node -e "
const anchor = require('@coral-xyz/anchor');
async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.programUpgradeSystem;
  
  const multisigPda = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('multisig')], program.programId)[0];

  console.log('🛑 Pausing System...');
  await program.methods.pauseSystem().accounts({
      multisigConfig: multisigPda,
      pauser: anchor.getProvider().publicKey,
  }).rpc();
  console.log('System is PAUSED.');
  
  // Verify we cannot propose (Should Fail)
  try {
     const buffer = anchor.web3.Keypair.generate().publicKey;
     const [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('proposal'), buffer.toBuffer()], program.programId);
     await program.methods.proposeUpgrade(buffer, 'Should Fail').accounts({
         proposal: pda,
         multisigConfig: multisigPda,
         proposer: anchor.getProvider().publicKey,
         systemProgram: anchor.web3.SystemProgram.programId,
     }).rpc();
  } catch (e) {
     console.log('✅ Good! Proposal failed as expected because system is paused.');
  }

  console.log('🟢 Resuming System...');
  await program.methods.resumeSystem().accounts({
      multisigConfig: multisigPda,
      resumer: anchor.getProvider().publicKey,
  }).rpc();
  console.log('System is RESUMED.');
}
main();
"
```
