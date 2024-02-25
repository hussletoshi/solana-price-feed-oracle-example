This guide assumes you have all the tools needed to run anchor programs 

Including Rust, Solana-cli and all related tools, npm and yarn, and anchor / avm.

Instructions 

`anchor build`

and 

`npm i`

Following run the following to get your deployment key 

`solana-keygen pubkey ./target/deploy/hello_pyth-keypair.json`

Replace the system program jey `111...111` with the key you get in /programs/hello-pyth/lib.rs (declareid section)
and anchor.toml 

Following that run 

`anchor deploy` and `anchor test`

