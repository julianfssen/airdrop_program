use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        signer::keypair::{Keypair, read_keypair_file},
        signer::Signer
    },
    crate::cli::{bs58_to_wallet::bs58_to_wallet, wallet_to_bs58::wallet_to_bs58}
};

pub mod cli;

const RPC_URL: &str = "https://api.devnet.solana.com";

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
    fn transfer_sol() {}

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
