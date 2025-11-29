import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { ProgramUpgradeSystem } from "../target/types/program_upgrade_system";

describe("program-upgrade-system", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // const program = anchor.workspace.programUpgradeSystem;
  const idl = require("../target/idl/program_upgrade_system.json");
  const programId = new anchor.web3.PublicKey("F4rYDGUKQHtJt14aPyGxtzacx9x7x9MH7rper2TuRdVz");
  const program = new anchor.Program(idl, programId, anchor.getProvider());

  let buffer: anchor.web3.PublicKey;
  let proposalPda: anchor.web3.PublicKey;
  let multisigConfigPda: anchor.web3.PublicKey;

  it("Is initialized!", async () => {
    const authority = anchor.getProvider().publicKey;
    const members = [authority];
    const threshold = 1;

    [multisigConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("multisig")],
      program.programId
    );

    console.log("Authority:", authority.toBase58());
    console.log("MultisigConfig PDA:", multisigConfigPda.toBase58());

    const tx = await program.methods
      .initializeMultisig(members, threshold)
      .accounts({
        multisigConfig: multisigConfigPda,
        authority: authority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Proposes an upgrade", async () => {
    buffer = anchor.web3.Keypair.generate().publicKey;
    const description = "Test upgrade proposal";

    [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), buffer.toBuffer()],
      program.programId
    );

    console.log("Proposal PDA:", proposalPda.toBase58());

    const tx = await program.methods
      .proposeUpgrade(buffer, description)
      .accounts({
        proposal: proposalPda,
        multisigConfig: multisigConfigPda,
        proposer: anchor.getProvider().publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Propose transaction signature", tx);
  });

  it("Approves an upgrade", async () => {
    const tx = await program.methods
      .approveUpgrade(proposalPda)
      .accounts({
        proposal: proposalPda,
        multisigConfig: multisigConfigPda,
        approver: anchor.getProvider().publicKey,
      })
      .rpc();

    console.log("Approve transaction signature", tx);
  });
});
