//! A handler for the flow of cryptographic functions etc.
//! TODO: This does nothing right now, just tests the libsodium binding

extern crate sodiumoxide;

fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {

    use sodiumoxide::crypto::secretbox;

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
