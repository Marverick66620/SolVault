import * as anchor from "@coral-xyz/anchor";

describe("blueshift_anchor_vault", () => {
  // Set the provider to the local cluster and wallet
  anchor.setProvider(anchor.AnchorProvider.env());

  // Get a handle to the deployed program
  const program = anchor.workspace.BlueshiftAnchorVault;

  it("Deposit lamports into vault", async () => {
    // Get the wallet keypair from the provider (signer)
    const signer = anchor.getProvider().wallet;
    const signerPublicKey = signer.publicKey;

    // Derive the PDA vault address and bump seed
    const [vaultPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), signerPublicKey.toBuffer()],
      program.programId
    );

    // Amount to deposit (in lamports)
    const amount = new anchor.BN(1_000_000);

    // Call the deposit instruction
    const tx = await program.methods.deposit(amount)
      .accounts({
        vault: vaultPda,                   // PDA vault account
        signer: signerPublicKey,          // Signer's public key
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Deposit transaction signature:", tx);
  });
});
