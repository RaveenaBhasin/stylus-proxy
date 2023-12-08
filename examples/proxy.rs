//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
// use eyre::eyre;
// use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let program_address = "0xae1F94e92c9b27ec2a34B3B474ebA460CE498E1B";
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    let priv_key = "e788f2866a5775c1e34be91f5c7b0abf92f4e79e80d5fdcdfff194ea718322cf";
    abigen!(
        Proxy,
        r#"[
            function init(address owner) external
            function getImplementation() external view returns (address)
            function setImplementation(address implementation) external
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    // let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&priv_key)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let proxy = Proxy::new(address, client);
    let _owner_address: Address = ("0x3647fc3a4209a4b302dcf8f7bb5d58defa6b9708").parse()?;
    // proxy.init(owner_address).send().await?;

    let implementation_address = proxy.get_implementation().call().await?;
    println!("Current implementation address: {:?}", implementation_address);

    // let new_implementation_address: Address = ("0xEEA6Da9Ea2eA6D65380608349b7e957805De10B7").parse()?; 
    // proxy.set_implementation(new_implementation_address).send().await?;

    // let updated_implementation_address = proxy.get_implementation().call().await?;
    // println!("Updated implementation address: {:?}", updated_implementation_address);



    Ok(())
}
