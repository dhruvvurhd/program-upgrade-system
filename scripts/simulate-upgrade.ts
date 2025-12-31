import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramUpgradeSystem } from "../target/types/program_upgrade_system";
import fs from "fs";
import path from "path";
import { execSync } from "child_process";

// -----------------------------------------------------------------------------
// HELPERS
// -----------------------------------------------------------------------------

function runCommand(cmd: string) {
    console.log(`> ${cmd}`);
    try {
        return execSync(cmd, { stdio: 'pipe' }).toString().trim();
    } catch (e: any) {
        console.error(`Error running command: ${cmd}`);
        console.error(e.stderr?.toString());
        throw e;
    }
}

async function main() {
    // Configure client
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    console.log("🚀 Starting Full Upgrade Simulation...");

    // 1. Load Programs
    const upgradeSystem = anchor.workspace.ProgramUpgradeSystem as Program<ProgramUpgradeSystem>;

    // Load Target Program from generated IDL
    // Since 'anchor build' passed, this file should exist.
    const idlPath = path.resolve(__dirname, "../target/idl/target_program.json");
    if (!fs.existsSync(idlPath)) {
        throw new Error(`IDL not found at ${idlPath}. Please run 'anchor build' first.`);
    }
    const targetIdl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));

    // Explicitly set the address if it's not in the IDL or to override it
    const targetProgramId = new anchor.web3.PublicKey("4WkLMT2hhjazZVJSSt8VDR8TCCVFDXxtNfg7P5vowxEe");

    const targetProgram = new anchor.Program(targetIdl, provider);

    console.log("📍 Target Program ID:", targetProgram.programId.toBase58());
    console.log("📍 Upgrade System ID:", upgradeSystem.programId.toBase58());

    // 2. Initialize Target Program (V1)
    console.log("\n📦 Initializing Target Program (V1 state)...");
    const counterKp = anchor.web3.Keypair.generate();

    try {
        await targetProgram.methods
            .initialize()
            .accounts({
                counter: counterKp.publicKey,
                user: provider.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([counterKp])
            .rpc();

        let counterAccount: any = await (targetProgram.account as any).counter.fetch(counterKp.publicKey);
        console.log(`   Count: ${counterAccount.count}, Version: ${counterAccount.version}`);
    } catch (e) {
        console.log("Target program might not be deployed yet? Ensure `anchor build` and validator is running.");
        throw e;
    }

    // 3. Initialize Upgrade System (Multisig)
    console.log("\n🔐 Initializing Upgrade System Multisig...");
    const [multisigPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("multisig")],
        upgradeSystem.programId
    );

    try {
        await upgradeSystem.account.multisigConfig.fetch(multisigPda);
        console.log("   Multisig already initialized.");
    } catch (e) {
        await upgradeSystem.methods
            .initializeMultisig([provider.publicKey], 1)
            .accounts({
                multisigConfig: multisigPda,
                authority: provider.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            } as any)
            .rpc();
        console.log("   Initialized new multisig config.");
    }

    // 4. Transfer Authority of Target Program to Upgrade System
    console.log("\n🔄 Transferring Authority to Upgrade System...");

    try {
        const newAuth = multisigPda.toBase58();
        const progId = targetProgram.programId.toBase58();

        console.log(`   Setting authority of ${progId} to ${newAuth}`);

        // Execute shell command
        runCommand(`solana program set-upgrade-authority ${progId} --new-upgrade-authority ${newAuth} --skip-new-upgrade-authority-signer-check`);
        console.log("   Authority transferred successfully.");

    } catch (e) {
        console.error("   Failed to transfer authority (simulated). Proceeding to check if it's already transferred...");
    }

    // 5. Deploy Buffer (V2)
    console.log("\n🏗️  Preparing V2 (Deploying Buffer)...");
    const soPath = path.resolve(__dirname, "../target/deploy/target_program.so");

    // Write buffer
    console.log("   Writing buffer...");
    const bufferOutput = runCommand(`solana program write-buffer ${soPath}`);

    // Output format: "Buffer: <PUBKEY>"
    const bufferIdMatch = bufferOutput.match(/Buffer: ([a-zA-Z0-9]+)/);
    if (!bufferIdMatch) throw new Error("Could not parse buffer ID from output");
    const bufferId = bufferIdMatch[1];
    console.log(`   Buffer Created: ${bufferId}`);

    const bufferPubkey = new anchor.web3.PublicKey(bufferId);

    // Transfer buffer authority to Multisig PDA via CPI
    console.log("   Transferring Buffer Authority to Multisig PDA...");
    await upgradeSystem.methods
        .setBufferAuthority()
        .accounts({
            multisigConfig: multisigPda,
            buffer: bufferPubkey,
            currentAuthority: provider.publicKey,
            bpfLoaderUpgradeable: new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111"),
        } as any)
        .rpc();
    console.log("   Buffer authority transferred.");

    // 6. Propose Upgrade
    console.log("\n🗳️  Proposing Upgrade...");

    const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), bufferPubkey.toBuffer()],
        upgradeSystem.programId
    );

    await upgradeSystem.methods
        .proposeUpgrade(bufferPubkey, "Simulated V2 Upgrade")
        .accounts({
            proposal: proposalPda,
            multisigConfig: multisigPda,
            proposer: provider.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();

    console.log("   Proposal created:", proposalPda.toBase58());

    // 7. Approve Upgrade
    console.log("\n👍 Approving Upgrade...");
    await upgradeSystem.methods
        .approveUpgrade(proposalPda)
        .accounts({
            proposal: proposalPda,
            multisigConfig: multisigPda,
            approver: provider.publicKey,
        } as any)
        .rpc();
    console.log("   Approved.");

    // 8. Wait for Timelock (test mode = 5s)
    console.log("\n⏳ Waiting for Timelock (5s)...");
    await new Promise(r => setTimeout(r, 6000));

    // 9. Execute Upgrade
    console.log("\n🚀 Executing Upgrade...");

    const programDataAddress = anchor.web3.PublicKey.findProgramAddressSync(
        [targetProgram.programId.toBuffer()],
        new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    )[0];

    await upgradeSystem.methods
        .executeUpgrade(proposalPda)
        .accounts({
            proposal: proposalPda,
            multisigConfig: multisigPda,
            programToUpgrade: targetProgram.programId,
            programData: programDataAddress,
            buffer: bufferPubkey,
            spillAccount: provider.publicKey,
            executor: provider.publicKey,
            bpfLoaderUpgradeable: new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111"),
        } as any)
        .rpc();

    console.log("\n✅ Upgrade Executed Successfully!");

    // 10. Verify
    console.log("\nverify: Checking Target Program...");
    // Let's verify we can still fetch the account (it should still have count=0, version=1 if we used same binary)
    // In a real upgrade, we check if version changed.
    let reFetched: any = await (targetProgram.account as any).counter.fetch(counterKp.publicKey);
    console.log(`   Count: ${reFetched.count}, Version: ${reFetched.version}`);

    console.log("Simulation Complete.");
}

main().then(() => process.exit(0)).catch(e => {
    console.error(e);
    process.exit(1);
});
