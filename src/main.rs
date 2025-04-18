mod wallet;
use std::fs::File;
use std::io::Write;
use std::io;

fn main() {
    let seed_phrase = find_or_create_seed_phrase();
    let wallet = wallet::create_wallet_from_seed_phrase(&seed_phrase);

    save_wallet_private_key(&wallet.private_key);

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
    seed_phrase = wallet::generate_seed_phrase();
  } else {
    seed_phrase = input;
  }

  seed_phrase
}

fn save_wallet_private_key(private_key: &str) {
  let mut file = File::create("private_key").expect("Error");
  file.write_all(private_key.to_string().as_bytes()).expect("Unable to write to file");
}
