import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingProgram } from "../target/types/staking_program";
import { Connection, Keypair } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { min } from "bn.js";

describe("staking-program", () => {
  // Configure the client to use the local cluster.
  // const web3 = require("@solana/web3.js");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  // const connection = new web3.Connection(
  //   web3.clusterApiUrl("localhost"),
  //   "confirmed"
  // );
  const connection = new Connection("https://127.0.0.1:8899", "confirmed");
  const mintKeypair = Keypair.fromSecretKey(
    new Uint8Array([
      197, 76, 73, 208, 214, 194, 127, 66, 164, 140, 63, 12, 41, 175, 75, 167,
      156, 161, 14, 98, 113, 30, 89, 120, 181, 135, 217, 2, 20, 13, 188, 25,
      161, 131, 222, 59, 137, 112, 135, 218, 117, 12, 207, 74, 160, 223, 116,
      69, 161, 41, 16, 30, 166, 101, 76, 152, 179, 142, 253, 246, 69, 70, 132,
      200,
    ])
  );

  // console.log(mintKeypair);

  // async function createMintToken() {
  //   const mint = await createMint(
  //     connection,
  //     payer.payer,
  //     payer.publicKey,
  //     payer.publicKey,
  //     9,
  //     mintKeypair
  //   );
  //   console.log(mint);
  // }

  const program = anchor.workspace.StakingProgram as Program<StakingProgram>;

  it("Is initialized!", async () => {
   // await createMintToken();

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
