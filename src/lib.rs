use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::nonblocking::rpc_client;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signature},
};
use std::{error::Error, str::FromStr}; // Add this import // Add this import

const LAMPORTS_PER_SOL: f64 = 1000000000.0;
const MOVIE_PROGRAM_ADDRESS: &str = "CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN";

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(public_key)? as f64 / LAMPORTS_PER_SOL)
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MyMovieInstruction {
    pub variant: u8,
    pub title: String,
    pub rating: u8,
    pub description: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MyMovie {
    pub initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String,
}

pub fn movie_review_transaction(
    rpc_client: &RpcClient,
    sender: &Keypair,
) -> Result<Signature, Box<dyn Error>> {
    let program_id = Pubkey::from_str(MOVIE_PROGRAM_ADDRESS)?;

    let movie_data = MyMovieInstruction {
        variant: 0,
        title: "The Incredibles".to_string(),
        rating: 5,
        description: "A movie about a little family with super powers.".to_string(),
    };

    let program_derived_address = Pubkey::find_program_address(
        &[sender.pubkey().as_ref(), movie_data.title.as_bytes()],
        &program_id,
    )
    .0;

    let instruction = Instruction::new_with_borsh(
        program_id,
        &movie_data,
        vec![
            AccountMeta::new(sender.pubkey(), true),
            AccountMeta::new(program_derived_address, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&sender.pubkey()));
    transaction.sign(&[sender], recent_blockhash);

    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}

pub fn fetch_deserialise_my_movie(
    rpc_client: &RpcClient,
    sender: &Keypair,
    movie_data: &MyMovie,
) -> Result<(), Box<dyn Error>> {
    let program_id = Pubkey::from_str(MOVIE_PROGRAM_ADDRESS)?;
    let program_derived_address = Pubkey::find_program_address(
        &[sender.pubkey().as_ref(), movie_data.title.as_bytes()],
        &program_id,
    )
    .0;

    println!("Program Derived Address: {:?}", program_derived_address);
    let account_data = rpc_client.get_account(&program_derived_address)?;
    println!(
        "Raw data for {}: {:?}",
        program_derived_address, &account_data.data
    );
    let deserialised_movie_data = MyMovie::try_from_slice(&account_data.data)?;
    println!("Movie Title: {}", deserialised_movie_data.title);
    println!("Movie Rating: {}", deserialised_movie_data.rating);
    println!("Movie Description: {}", deserialised_movie_data.description);
    println!("Movie Address: {}", program_derived_address);

    Ok(())
}
