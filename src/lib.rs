mod utils;

use monero_rust::{MoneroWallet, Language, Network};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // Generate a mnemonic seed in English
    let mnemonic = MoneroWallet::generate_mnemonic(Language::English);
    println!("Generated mnemonic: {}", mnemonic);

    let mut primary_address = String::new();
    // Create a wallet from the mnemonic
    match MoneroWallet::new(&mnemonic, Network::Mainnet) {
        Ok(wallet) => {
            // Get the primary address of the wallet
            primary_address = wallet.get_primary_address();
            println!("Primary address: {}", primary_address);
        }
        Err(e) => {
            eprintln!("Failed to create wallet: {}", e);
        }
    }

    let message = format!("{}: {}", &mnemonic, &primary_address);
    alert(message.as_str());
}
