use rust_client_test::check_account_balance;
use rust_client_test::check_balance;
use rust_client_test::fetch_deserialise_my_movie;
use rust_client_test::initialize_token_account;
use rust_client_test::mint_to_account;
use rust_client_test::movie_review_transaction;
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
    println!("Hello, mint token transaction!");
    println!("Create new keypair for token mint account...");
    let mint_account_keypair = Keypair::new();
    println!("Mint account keypair: {:?}", mint_account_keypair.pubkey());
    println!("Create new keypair for token account...");
    let token_account_keypair = Keypair::new();
    println!("1. Initialize token mint account...");
    match initialize_token_mint(&rpc_client, &keypair, &mint_account_keypair) {
        Ok(signature) => {
            println!("Successful initailisation of mint account : Signature: {:?}", signature);

            println!("2. Initialize token holding account...");
            match initialize_token_account(&rpc_client, &keypair, &mint_account_keypair, &token_account_keypair) {
                Ok(signature) => {
                    println!("Successfull initailisation of token holding account : Signature: {:?}", signature);
                    // Rest of your code here
                    // Mint 100 tokens to the token account
                    println!("3. Mint 100 tokens to the token account...");
                    match mint_to_account(&rpc_client, &keypair, &mint_account_keypair, &token_account_keypair, 100) {
                        Ok(signature) => {
                            println!("Successfully minted 100 tokens to the token account: Signature: {:?}", signature);
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    
    let account_balance = check_account_balance(&rpc_client, &token_account_keypair).unwrap();
    println!("Account balance: {:?}", account_balance);
    assert_eq!(account_balance, 100);
}
