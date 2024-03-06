use rust_client_test::check_balance;
use rust_client_test::movie_review_transaction;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};

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
    println!("Hello, Movie Review Transaction!");

    match movie_review_transaction(&rpc_client, &keypair) {
        Ok(signature) => {
            println!(
                "Movie transaction was successful, signature: {:?}",
                signature
            );
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
