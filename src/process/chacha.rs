use std::io::Read;

use base64::Engine;
use base64::engine::general_purpose;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use sha2::{Digest, Sha256};

pub struct ChaCha;

impl ChaCha {
    pub fn process_encrypt(user_key: &str, reader: &mut dyn Read) -> anyhow::Result<String> {
        // Use the user key to construct the cipher.
        let cipher = str_to_sha256_vec(user_key);

        // Generate a random nonce.
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        // Read the input.
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // Encrypt the input, prepend the nonce, and encode.
        let ciphertext = cipher.encrypt(&nonce, &*buf);
        match ciphertext {
            Ok(mut result) => {
                result.splice(..0, nonce.iter().copied());
                Ok(general_purpose::URL_SAFE.encode(&result))
            }
            Err(err) => anyhow::bail!("ERROR ChaCha20Poly1305 encrypting: {}", err),
        }
    }

    pub fn process_decrypt(user_key: &str, reader: &mut dyn Read) -> anyhow::Result<String> {
        // Use the user key to construct the cipher.
        let cipher = str_to_sha256_vec(user_key);

        // Read the input.
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let buf = buf.trim();

        // Decode the input, split the nonce & ciphertext, and decrypt.
        let encrypted = general_purpose::URL_SAFE.decode(buf)?;
        type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
        let (nonce, ciphertext) = encrypted.split_at(NonceSize::to_usize());
        let nonce = GenericArray::from_slice(nonce);
        let decrypted = cipher.decrypt(nonce, ciphertext);
        match decrypted {
            Ok(result) => Ok(String::from_utf8(result)?),
            Err(err) => anyhow::bail!("ERROR ChaCha20Poly1305 decrypting: {}", err),
        }
    }
}

fn str_to_sha256_vec(user_key: &str) -> ChaCha20Poly1305 {
    let bytes = user_key.trim().as_bytes().to_vec();
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&result[..]);
    hash_bytes.to_vec();
    ChaCha20Poly1305::new(GenericArray::from_slice(&hash_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encrypt_decrypt() {
        let key = "test-key";
        let bad_key = "bad-key";
        let input = "test-input";
        let mut reader = input.as_bytes();
        let encrypted = ChaCha::process_encrypt(key, &mut reader).unwrap();
        let mut reader = encrypted.as_bytes();
        let decrypted = ChaCha::process_decrypt(key, &mut reader).unwrap();
        assert_eq!(input, decrypted);

        let mut reader = input.as_bytes();
        let failed = ChaCha::process_decrypt(bad_key, &mut reader);
        assert!(failed.is_err());
    }
}
