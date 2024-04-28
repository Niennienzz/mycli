use std::io::Read;

use chacha20poly1305::{aead::{Aead, AeadCore, KeyInit, OsRng}, ChaCha20Poly1305};
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;

use super::{
    b64_url_safe_no_pad_decode,
    b64_url_safe_no_pad_encode,
    str_to_sha256_vec,
};

pub fn process_decrypt(user_key: &str, reader: &mut dyn Read) -> anyhow::Result<String> {
    // Use the user key to construct the cipher.
    let cipher = ChaCha20Poly1305::new(
        GenericArray::from_slice(&str_to_sha256_vec(user_key)),
    );

    // Read the input.
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    // Decode the input, split the nonce & ciphertext, and decrypt.
    let encrypted = b64_url_safe_no_pad_decode(buf)?;
    type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
    let (nonce, ciphertext) = encrypted.split_at(NonceSize::to_usize());
    let nonce = GenericArray::from_slice(nonce);
    let decrypted = cipher.decrypt(nonce, ciphertext);
    match decrypted {
        Ok(result) => Ok(String::from_utf8(result)?),
        Err(err) => anyhow::bail!("Error decrypting: {}", err),
    }
}

pub fn process_encrypt(user_key: &str, reader: &mut dyn Read) -> anyhow::Result<String> {
    // Use the user key to construct the cipher.
    let cipher = ChaCha20Poly1305::new(
        GenericArray::from_slice(&str_to_sha256_vec(user_key)),
    );

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
            Ok(b64_url_safe_no_pad_encode(&result))
        }
        Err(err) => anyhow::bail!("Error encrypting: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encrypt_decrypt() {
        let user_key = "test-key";
        let input = "test-input";
        let mut reader = input.as_bytes();
        let encrypted = process_encrypt(user_key, &mut reader).unwrap();
        let mut reader = encrypted.as_bytes();
        let decrypted = process_decrypt(user_key, &mut reader).unwrap();
        assert_eq!(input, decrypted);
    }
}
