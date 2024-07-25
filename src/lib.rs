use {
    solana_client::rpc_client::RpcClient,
    solana_program::{
        pubkey::Pubkey,
        system_instruction::transfer,
    },
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        signer::keypair::{Keypair, read_keypair_file},
        signer::Signer,
        transaction::Transaction,
    },
    std::str::FromStr,
    crate::cli::{bs58_to_wallet::bs58_to_wallet, wallet_to_bs58::wallet_to_bs58}
};

pub mod cli;

const RPC_URL: &str = "https://api.devnet.solana.com";
const WBA_PUBKEY_ADDRESS: &str = "FyScGJc8PWZGs2XVTZyNdkDyfKmiyvKLuuAUxiHAPPKL";

pub fn keygen() {
    let keypair = Keypair::new();
    println!("You've generated a new Solana wallet: {}", keypair.pubkey().to_string());
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", keypair.to_bytes());
}

pub fn airdrop() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    println!("Airdropping 2 SOL to {:?}", keypair.pubkey().to_string());
    let client = RpcClient::new(RPC_URL);

    let sol_to_claim = 2;
    match client.request_airdrop(&keypair.pubkey(), sol_to_claim * LAMPORTS_PER_SOL) {
        Ok(s) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
        },
        Err(e) => println!("Oops, something went wrong: {}", e.to_string())
    };
}

pub fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let to_pubkey = Pubkey::from_str(WBA_PUBKEY_ADDRESS).expect("Failed to parse pubkey");

    let client = RpcClient::new(RPC_URL);
    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let sol_to_transfer = 0.1;
    let lamports_to_transfer = (sol_to_transfer * LAMPORTS_PER_SOL as f64) as u64;
    let instruction = transfer(&keypair.pubkey(), &to_pubkey, lamports_to_transfer);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        recent_blockhash
    );

    let signature = client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        keygen();
    }

    #[test]
    fn test_airdrop() {
        airdrop();
    }

    #[test]
    fn test_transfer_sol() {
        transfer_sol();
    }

    // TODO: Refactor this test to not rely on manual stdin
    // #[test]
    // fn test_bs58_to_wallet() {
    //     bs58_to_wallet();
    // }

    // TODO: Refactor this test to not rely on manual stdin
    // #[test]
    // fn test_wallet_to_bs58() {
    //     wallet_to_bs58();
    // }
}
