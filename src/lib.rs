use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction::create_account;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signer::Signer;
use solana_sdk::{system_instruction, system_program};
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    instruction::Instruction,
    signature::{Keypair, Signature},
};
use spl_token::instruction::initialize_mint;
use spl_token::instruction::initialize_account;
use spl_token::instruction::mint_to;
use spl_token::state::{Account};
use std::{error::Error, str::FromStr}; // Add this import // Add this import
use spl_token::state::Mint;
use solana_sdk::program_pack::Pack;


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

pub fn initialize_token_mint(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint_keypair: &Keypair,
) -> Result<Signature, Box<dyn Error>>{
    let token_program_id = spl_token::id();
    let minimumrentbalance = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

     // Create an instruction to create a new token account for the mint
    let create_account_instruction = create_account(
        &payer.pubkey(),
       &mint_keypair.pubkey(),
       minimumrentbalance,
       Mint::LEN as u64,
       &token_program_id,
   );
    // Create an instruction to initialize the token mint with 9 decimals
    let initialize_mint_instruction = initialize_mint(
        &token_program_id,
        &mint_keypair.pubkey(),
        &payer.pubkey(),
        None,
        9,
    )?;

   
    // Create a transaction that includes both instructions
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &mint_keypair], recent_blockhash);

    // Send and confirm the transaction
    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}

pub fn initialize_token_account(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint_keypair: &Keypair,
    token_account_keypair: &Keypair,
) -> Result<Signature, Box<dyn Error>> {
    let token_program_id = spl_token::id();

    // Generate a new token account keypair
    let minimum_rent_balance = rpc_client.get_minimum_balance_for_rent_exemption(Account::LEN)?;

    // Create an instruction to create a new token account for the mint
    let create_account_instruction = create_account(
        &payer.pubkey(),
        &token_account_keypair.pubkey(),
        minimum_rent_balance,
        Account::LEN as u64,
        &token_program_id,
    );

    // Create an instruction to associate the token account with the mint
    let initialize_account_instruction = initialize_account(
        &token_program_id,
        &token_account_keypair.pubkey(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
    )?;

    // Create a transaction that includes both instructions
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(
        &[create_account_instruction, initialize_account_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &token_account_keypair], recent_blockhash);

    // Send and confirm the transaction
    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}

pub fn mint_to_account(
    rpc_client: &RpcClient,
    payer: &Keypair,
    mint_keypair: &Keypair,
    token_account_keypair: &Keypair,
    amount: u64,
) -> Result<Signature, Box<dyn Error>> {
    let token_program_id = spl_token::id();

    // Create an instruction to mint tokens to the destination account
    let mint_to_instruction = mint_to(
        &token_program_id,
        &mint_keypair.pubkey(),
        &token_account_keypair.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        amount,
    )?;

    // Create a transaction that includes the mint_to instruction
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(
        &[mint_to_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &payer], recent_blockhash);

    // Send and confirm the transaction
    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}

pub fn check_account_balance(
    rpc_client: &RpcClient,
    token_account_keypair: &Keypair,
) -> Result<u64, Box<dyn Error>> {
    let account = rpc_client.get_account(&token_account_keypair.pubkey())?;
    let account_data = Account::unpack(&account.data)?;
    Ok(account_data.amount)
}