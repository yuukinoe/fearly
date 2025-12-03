use rand::RngCore;
use base64::{engine::general_purpose, Engine};

fn main() {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    println!("{}", general_purpose::STANDARD.encode(&key));
}
