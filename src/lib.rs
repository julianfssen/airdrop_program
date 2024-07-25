use solana_sdk::{signer::keypair::Keypair, signer::Signer};

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
}
