use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    instruction::{self, Instruction},
    signature::{Keypair, Signature},
    system_instruction, system_transaction,
};
use std::{error::Error, str::FromStr};

const LAMPORTS_PER_SOL: f64 = 1000000000.0;
const PING_PROGRAM_ADDRESS: &str = "ChT1B39WKLS8qUrkLvFDXMhEJ4F1XZzwUNHUt4AU9aVa";
const PING_PROGRAM_DATA_ADDRESS: &str = "Ah9K7dQ8EHaZqcAsgBW8w37yN2eAy3koFmUn4x3CJtod";

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn transfer(
    rpc_client: &RpcClient,
    sender: &Keypair,
    recipient: &Pubkey,
    amount: f64,
) -> Result<Signature, Box<dyn Error>> {
    let amount = (amount * LAMPORTS_PER_SOL) as u64;
    let transaction = system_transaction::transfer(
        &sender,
        recipient,
        amount,
        rpc_client.get_latest_blockhash()?,
    );
    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}

pub fn ping_program_transaction(
    rpc_client: &RpcClient,
    sender: &Keypair,
) -> Result<Signature, Box<dyn Error>> {
    let program_id = Pubkey::from_str(PING_PROGRAM_ADDRESS)?;
    let ping_program_data_pubkey = Pubkey::from_str(PING_PROGRAM_DATA_ADDRESS)?;

    let instruction = Instruction {
        program_id: program_id,
        accounts: vec![AccountMeta::new(ping_program_data_pubkey, false)],
        data: vec![],
    };

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&sender.pubkey()));
    transaction.sign(&[sender], recent_blockhash);
    Ok(rpc_client.send_and_confirm_transaction(&transaction)?)
}
