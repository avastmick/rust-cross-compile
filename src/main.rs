//! A handler for the flow of cryptographic functions etc.

extern crate sodiumoxide;

use sodiumoxide::crypto::secretbox;
use std::string::String;

/// Initialises sodiumoxide in order to prevent threading faults
/// This needs only be done once per usage.
/// Will fatally panic if libsodium cannot be initialised.
fn init() {
    sodiumoxide::init().expect("FATAL: Cannot init libsodium via sodiumoxide binding; exiting.");
}

fn main() {
    println!("Starting secretbox encryption testing!");
    init();
    // Create a key to use to encrypt the secret plaintext
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let plaintext = "I dreamed a dream and the dream was data...";
    println!("{:?}", plaintext);
    let ciphertext = secretbox::seal(plaintext.as_bytes(), &nonce, &key);
    println!("{:?}", ciphertext);
    let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();
    println!("{:?}", String::from_utf8(their_plaintext).unwrap());
}

#[cfg(test)]
mod tests {

    use super::*;
    use sodiumoxide::crypto::secretbox;

    #[test]
    fn init_sodiumoxide() {
        init();
    }

    #[test]
    fn test_rust_sodium_encrypt() {
        let key = secretbox::gen_key();
        let nonce = secretbox::gen_nonce();
        let plaintext = b"some data";
        let ciphertext = secretbox::seal(plaintext, &nonce, &key);
        let their_plaintext = secretbox::open(&ciphertext, &nonce, &key).unwrap();
        assert!(plaintext == &their_plaintext[..]);
    }
}
