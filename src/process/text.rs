use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

use crate::{cli::SignFormat, get_reader, process_genpass};
use anyhow::{Ok, Result};

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    // TODO: 这里为啥不需要 mut reader: impl Read ?
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}

trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized; // not str / [u8]
}

trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 注意这里需要先用hash绑定，再转换成bytes，否则&[u8]将会引用一个被free掉的value
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    // 关于什么时候使用 try_into()
    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, false, false, false, false)?;
        Ok(vec![key.into_bytes()])
    }
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let sk = signing_key.to_bytes().to_vec();

        let verifying_key = signing_key.verifying_key();
        let vk = verifying_key.to_bytes().to_vec();
        Ok(vec![sk, vk])
    }
}

// input: input filename or std-in
// key: key filename
// format: blake3 or ed25519
pub fn process_text_sign(input: &str, key: &str, format: SignFormat) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;

    let signature = match format {
        SignFormat::Blake3 => {
            let blake3 = Blake3::load(key)?;
            blake3.sign(&mut reader)
        }
        SignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)
        }
    }?;

    Ok(signature)
}

pub fn process_text_verify(input: &str, key: &str, sig: &str, format: SignFormat) -> Result<bool> {
    let reader = get_reader(input)?;

    let sig = BASE64_URL_SAFE_NO_PAD.decode(sig.as_bytes())?;
    let res = match format {
        SignFormat::Blake3 => {
            let blake3 = Blake3::load(key)?;
            blake3.verify(reader, &sig)
        }
        SignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(reader, &sig)
        }
    }?;

    Ok(res)
}

pub fn process_generate_keys(format: SignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        SignFormat::Blake3 => Blake3::generate(),
        SignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() {
        let key = [0; 32];
        let blake3 = Blake3 { key };
        let mut reader = std::io::Cursor::new(b"hello!");
        let sig = blake3.sign(&mut reader).unwrap();
        reader.set_position(0);
        assert!(blake3.verify(reader, &sig).unwrap());
    }
}
