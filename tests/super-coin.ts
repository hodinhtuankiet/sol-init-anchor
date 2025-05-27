import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SuperCoin } from "../target/types/super_coin";

describe("todo-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TodoProgram as Program<TodoProgram>;
  const name = "Khac Vy";

  it("Create profile", async () => {
    const [profile, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), provider.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createProfile(name)
      .accounts({
        creator: provider.publicKey,
        profile: profile,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });
});

