use aes::Aes256;
use aes::cipher::{KeyIvInit, StreamCipher};
use ctr::Ctr64BE;
use solana_sdk::signer::keypair::Keypair;
use base64::{encode, decode};
use std::fs;

/// Re-export Solana PublicKey for convenience
pub use solana_sdk::pubkey::Pubkey;

/// Encrypts a Solana Keypair with the given password.
pub fn encrypt_keypair(keypair: &Keypair, password: &str) -> Vec<u8> {
    let key = derive_key(password);
    let iv = [0u8; 16];
    let mut cipher = Ctr64BE::<Aes256>::new(&key.into(), &iv.into());

    let serialized = bincode::serialize(keypair).expect("Failed to serialize keypair");
    let mut encrypted = serialized.clone();
    cipher.apply_keystream(&mut encrypted);

    encrypted
}

/// Decrypts an encrypted Solana Keypair using the given password.
pub fn decrypt_keypair(encrypted: &[u8], password: &str) -> Keypair {
    let key = derive_key(password);
    let iv = [0u8; 16];
    let mut cipher = Ctr64BE::<Aes256>::new(&key.into(), &iv.into());

    let mut decrypted = encrypted.to_vec();
    cipher.apply_keystream(&mut decrypted);

    bincode::deserialize(&decrypted).expect("Failed to deserialize keypair")
}

/// Derives a 32-byte key from a password (for simplicity; not recommended for production).
pub fn derive_key(password: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    for (i, byte) in password.bytes().cycle().take(32).enumerate() {
        key[i] = byte;
    }
    key
}

/// Reads an encrypted keypair from a file.
pub fn read_keypair_from_file(file: &str, password: &str) -> Keypair {
    let encrypted_data = fs::read(file).expect("Failed to read keypair file");
    let encrypted = decode(encrypted_data).expect("Failed to decode encrypted data");
    decrypt_keypair(&encrypted, password)
}

/// Writes an encrypted keypair to a file.
pub fn write_keypair_to_file(file: &str, keypair: &Keypair, password: &str) {
    let encrypted = encrypt_keypair(keypair, password);
    fs::write(file, encode(encrypted)).expect("Failed to write encrypted keypair");
}
