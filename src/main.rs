mod wallet;
use std::io;
use wallet::{Wallet, BitcoinWallet};

fn main() {
    let seed_phrase = find_or_create_seed_phrase();
    let wallet = Wallet::new(&seed_phrase);

    wallet.save_private_key();

    println!("{}", seed_phrase);
    println!("{:#?}", wallet);

}

fn find_or_create_seed_phrase() -> String {
  let mut buffer = String::new();

  println!("Enter seed phrase or enter 1 to create new");
  io::stdin().read_line(&mut buffer).expect("Failed");
  let input: String = buffer.trim().parse().unwrap();

  let mut seed_phrase: String = "".to_string();
  if input == "1" {
    seed_phrase = Wallet::generate_seed_phrase();
  } else {
    seed_phrase = input;
  }

  seed_phrase
}
