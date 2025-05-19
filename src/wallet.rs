
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::{Address, CompressedPublicKey, Network, PrivateKey};
use bip39::{Language, Mnemonic};
use rand_core::{TryRngCore, OsRng};
 #[derive(Debug)]
pub struct Wallet {
   private_key: String,
   public_key: String,
   address: String,
}

pub trait BitcoinWallet {
    fn new(seed_phrase: &str) -> Self;
    fn generate_seed_phrase() -> String;
    fn save_private_key(&self) -> ();
}

impl BitcoinWallet for Wallet {
    fn new(seed_phrase: &str) -> Self {
      let network = Network::Testnet; 
      let mnemonic = Mnemonic::from_str(&seed_phrase).unwrap();
      let entropy = mnemonic.to_entropy();

      let secp: Secp256k1<bitcoin::secp256k1::All> = Secp256k1::new();

      let secret_key = SecretKey::from_slice(&entropy).expect("32 bytes, within curve order");
      let public_key = PublicKey::from_secret_key(&secp, &secret_key);

      let private_key = PrivateKey::new(secret_key, network);
      let public_key = CompressedPublicKey(public_key);

      let address = Address::p2wpkh(&public_key, network);

      Self {
        public_key: public_key.to_string(),
        private_key: private_key.to_string(),
        address: address.to_string(),
      }
    }

    fn generate_seed_phrase() -> String {
      // create a new randomly generated mnemonic phrase
      let mut entropy: [u8; 32] = [0; 32];
      OsRng.try_fill_bytes(&mut entropy).unwrap();
      let phrase = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
      format!("{}", phrase)
    }

    fn save_private_key(&self) -> () {
      let mut file = File::create("private_key").expect("Error");
      file.write_all(self.private_key.to_string().as_bytes()).expect("Unable to write to file");
      ()
    }
}