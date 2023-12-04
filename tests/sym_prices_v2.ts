import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SymPricesV2 } from "../target/types/sym_prices_v2";
import { PublicKey, Connection, Keypair , Transaction } from "@solana/web3.js";
import fs from "fs";
import { BN } from "bn.js";

describe("sym_prices_v2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SymPricesV2 as Program<SymPricesV2>;
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = new Connection("https://api.devnet.solana.com");
  const keypairPath = "/Users/macintoshhd/my-solana-wallet/my-keypair.json";
  const keypairString = fs.readFileSync(keypairPath, "utf8");
  const keypairBuffer = new Uint8Array(JSON.parse(keypairString));
  const walletKeypair = Keypair.fromSecretKey(keypairBuffer);

  it("Is initialized!", async () => {
    let blockhash = await connection.getLatestBlockhash();
    let recentBlockhash = blockhash.blockhash;
    // Add your test here.
    //@ts-ignore
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
    // let tx = await program.methods.loadPrices(
    //   24
    // ).accounts({
    //   oracle: new PublicKey("6TBAS2cYjLzhQqsYvB2asit8KD6kPqEJtaY4fjoPcUvS"),
      
    // })
    // // .rpc();
    // .instruction();

    let tx = await program.methods.refreshOracles({
      data: {
        oracleTimestamp: new BN(0),
        price: [],
  
      }
    }).accounts({
      oracle: new PublicKey("4zathfrvoJMpJzwUxj1ycP9P54N8B8TZwEsaTwS7y7Xi"),
      function: new PublicKey("BLY1poKaEyNqKWR1jUUQGXrMsZh6BBqyShrYtekLNVUd"),
      enclaveSigner: new PublicKey("D4MyLhsDeHaRKafCXNC6t3itvew4w4gUKutchDcv4KVw")
    }).instruction();
    const transaction = new anchor.web3.Transaction({recentBlockhash: recentBlockhash}).add(
      tx,
    );
    let sgn = transaction.sign(walletKeypair);
    console.log("signature", sgn);
    let serialized = transaction.serialize();
    var signature = await connection.sendRawTransaction(
          serialized,
          {skipPreflight: true, preflightCommitment: "confirmed"}
      );
    console.log(signature);
  });
});
