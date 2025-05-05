
use std::str::FromStr;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::{Address, CompressedPublicKey, Network, PrivateKey};
use bip39::{Language, Mnemonic};
use rand_core::{TryRngCore, OsRng};
 #[derive(Debug)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
}

pub fn generate_seed_phrase() -> String {
  // create a new randomly generated mnemonic phrase
  let mut entropy: [u8; 32] = [0; 32];
  OsRng.try_fill_bytes(&mut entropy).unwrap();
  let phrase = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
  format!("{}", phrase)
}

pub fn create_wallet_from_seed_phrase(seed_phrase: &str) -> Wallet {
  let network = Network::Testnet; 
  let mnemonic = Mnemonic::from_str(&seed_phrase).unwrap();
  let entropy = mnemonic.to_entropy();

  let secp: Secp256k1<bitcoin::secp256k1::All> = Secp256k1::new();

  let secret_key = SecretKey::from_slice(&entropy).expect("32 bytes, within curve order");
  let public_key = PublicKey::from_secret_key(&secp, &secret_key);

  let private_key = PrivateKey::new(secret_key, network);
  let public_key = CompressedPublicKey(public_key);

  let address = Address::p2wpkh(&public_key, network);

  let wallet: Wallet = { Wallet {
    public_key: public_key.to_string(),
    private_key: private_key.to_string(),
    address: address.to_string(),
  } };

  wallet
}
