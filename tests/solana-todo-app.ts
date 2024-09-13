import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { SolanaTodoApp } from "../target/types/solana_todo_app";
// import * as assert from "assert";

describe("solana-todo-app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTodoApp;

  it("Initialize user", async () => {
    const tx = await program.methods.initializeUser().rpc();
    console.log("Your transaction signature", tx);
  });
});
