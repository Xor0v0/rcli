use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Error as ChaCha20Error,
};

use crate::get_reader;

pub fn process_encrypt(input: &str, key: &[u8]) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let cipher = chacha20poly1305::ChaCha20Poly1305::new_from_slice(key)?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let cipher_text = cipher
        .encrypt(&nonce, buf.as_slice())
        .map_err(|e: ChaCha20Error| anyhow!(format!("Error with chacha20poly1305: {:?}", e)))?;
    Ok(cipher_text)
}

pub fn process_decrypt(input: &str, key: &[u8]) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let cipher = chacha20poly1305::ChaCha20Poly1305::new_from_slice(key)?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let plain_text = cipher
        .decrypt(&nonce, buf.as_slice())
        .map_err(|e: ChaCha20Error| anyhow!(format!("Error with chacha20poly1305: {:?}", e)))?;
    Ok(plain_text)
}
