use std::str::FromStr;

use rust_client_test::check_account_balance;
use rust_client_test::check_balance;
use rust_client_test::fetch_deserialise_my_movie;
use rust_client_test::initialize_token_account;
use rust_client_test::mint_to_account;
use rust_client_test::movie_review_transaction;
use rust_client_test::send_hello_world_transaction;
use rust_client_test::MyMovie;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use rust_client_test::initialize_token_mint;

const URL: &str = "https://api.devnet.solana.com";

fn main() {
    dotenv::dotenv().ok();
    let rpc_client = RpcClient::new(URL.to_string());
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let keypair = Keypair::from_base58_string(&secret_key);
    println!(
        "balance: {:?}",
        (check_balance(&rpc_client, &keypair.pubkey()).unwrap())
    );
    println!("Call hello_world program");
    let hello_world_program_id = solana_sdk::pubkey::Pubkey::from_str("4K6V6DQTFcxDxtpHn7b5wbrvGDdZqNZUYPzdYUWK8zgq").unwrap();

    match send_hello_world_transaction(&rpc_client, &keypair, &hello_world_program_id) {
        Ok(signature) => {
            println!("Signature: {:?}", signature);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
