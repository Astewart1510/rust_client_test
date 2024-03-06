use std::str::FromStr;

use dotenv::dotenv;
use rust_client_test::ping_program_transaction;
use rust_client_test::{check_balance, transfer};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    signature::Keypair,
    signer::{keypair, Signer},
};

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
    println!("Hello, world!");

    let receiver = Pubkey::from_str("37e31h3VDfBThRnz87cRxYsG6S5uVGG8KpaPfeFL5V37").unwrap();

    let transfer_amount = 0.01;

    match transfer(&rpc_client, &keypair, &receiver, transfer_amount) {
        Ok(signature) => {
            println!(
                "Tranfer of {} SOL was successful, signature: {:?}",
                transfer_amount, signature
            );
            if let Ok(balance) = check_balance(&rpc_client, &keypair.pubkey()) {
                println!("New balance of sender: {}", balance);
            }
            if let Ok(balance) = check_balance(&rpc_client, &receiver) {
                println!("New balance of receiver: {}", balance);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    match ping_program_transaction(&rpc_client, &keypair) {
        Ok(signature) => {
            println!(
                "Ping program transaction was successful, signature: {:?}",
                signature
            );
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
