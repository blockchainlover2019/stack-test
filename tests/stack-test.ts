import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StackTest } from "../target/types/stack_test";
import { LargeStackApp } from "../target/types/large_stack_app";

import { createMint, createAccount, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Keypair, PublicKey, Connection, LAMPORTS_PER_SOL, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';

describe("stack-test", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const stackTestProgram = anchor.workspace.StackTest as Program<StackTest>;
  const largeStackProgram = anchor.workspace.LargeStackApp as Program<LargeStackApp>;
  
  const testKey = Keypair.generate();

  const tokenAccounts: Array<PublicKey> = [];
  it("setup mints and accounts", async () => {
    await airdropSol(provider, testKey.publicKey, 10 * LAMPORTS_PER_SOL);
    const tokenMint = await createMint(provider.connection, testKey, testKey.publicKey, null, 6);
    for (let i = 0; i < 11; i ++) {
      console.log('i =', i);
      let accKey = await createAccount(provider.connection, testKey, tokenMint, testKey.publicKey, Keypair.generate())
      tokenAccounts.push(accKey);
    }
  });

  it("Use large stack", async () => {
    // Add your test here.
    const tx = new Transaction().add(await largeStackProgram.methods.useLargeStack()
    .accounts({
      tester: testKey.publicKey,
      account0: tokenAccounts[0],
      account1: tokenAccounts[1],
      account2: tokenAccounts[2],
      account3: tokenAccounts[3],
      account4: tokenAccounts[4],
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .instruction());
    console.log(await simulateTxn(provider.connection, tx, testKey.publicKey));
    await sendAndConfirmTransaction(provider.connection, tx, [testKey]);
  });

  it("Do stack test", async () => {
    // Add your test here.
    const tx = new Transaction().add(await stackTestProgram.methods.testStack()
    .accounts({
      tester: testKey.publicKey,
      account0: tokenAccounts[0],
      account1: tokenAccounts[1],
      account2: tokenAccounts[2],
      account3: tokenAccounts[3],
      account4: tokenAccounts[4],
      account5: tokenAccounts[5],
      account6: tokenAccounts[6],
      account7: tokenAccounts[7],
      account8: tokenAccounts[8],
      account9: tokenAccounts[9],
      account10: tokenAccounts[10],
      tokenProgram: TOKEN_PROGRAM_ID,
      largeStackProgram: largeStackProgram.programId,
    })
    .instruction());
    console.log(await simulateTxn(provider.connection, tx, testKey.publicKey));
    await sendAndConfirmTransaction(provider.connection, tx, [testKey]);
  });
});

export const airdropSol = async (
  provider: anchor.AnchorProvider,
  target: PublicKey,
  lamps: number
): Promise<string> => {
  const sig: string = await provider.connection.requestAirdrop(target, lamps);
  await provider.connection.confirmTransaction(sig);
  return sig;
};

export const simulateTxn = async (
  connection: Connection,
  tx: Transaction,
  userPublicKey: PublicKey
) => {
  const blockhashInfo = await connection.getLatestBlockhash();
  tx.recentBlockhash = blockhashInfo.blockhash;
  tx.feePayer = userPublicKey;
  const simulationResult = await connection.simulateTransaction(
    tx.compileMessage()
  );
  return simulationResult;
};
