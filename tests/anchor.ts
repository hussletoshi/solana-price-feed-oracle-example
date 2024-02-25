import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import type { HelloPyth } from "../target/types/hello_pyth";

describe("Oracle Test BTC.", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());


  const program = anchor.workspace.HelloPyth as anchor.Program<HelloPyth>;
  
  it("Get BTC Price", async () => {
    // Send transaction
    const txHash = await program.methods
      .fetchBtcPrice()
      .accounts({
        priceFeed: new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J"), // Oracle Account aka Data Feed.
      })
      .rpc({skipPreflight: true});
    console.log(`${txHash}`);

    // Confirm transaction
    await program.provider.connection.confirmTransaction(txHash);

  });

  it("Get ETH Price", async () => {
    // Send transaction
    const txHash = await program.methods
      .fetchEthPrice()
      .accounts({
        priceFeedEth: new anchor.web3.PublicKey("EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw"), // Oracle Account aka Data Feed.
      })
      .rpc({skipPreflight: true});
    console.log(`${txHash}`);

    // Confirm transaction
    await program.provider.connection.confirmTransaction(txHash);

  });
});
