use {
    solana_sdk::{signer::keypair::Keypair, signer::Signer},
    crate::cli::{bs58_to_wallet::bs58_to_wallet, wallet_to_bs58::wallet_to_bs58}
};

pub mod cli;

pub fn keygen() {
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        keygen();
    }

    #[test]
    fn airdrop() {}
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
