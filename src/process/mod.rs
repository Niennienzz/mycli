use base64::{Engine as _, engine::general_purpose};
use sha2::{Digest, Sha256};

pub use chacha::{process_decrypt, process_encrypt};

mod chacha;

// Helper function to encode data to base64 (URL safe, no padding).
// This can be shared between ChaCha and JWT.
fn b64_url_safe_no_pad_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

// Helper function to decode base64 string (URL safe, no padding).
// This can be shared between ChaCha and JWT.
fn b64_url_safe_no_pad_decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(data)
}

// Helper function to hash a string to a 32-byte vector.
// This can be shared between ChaCha and JWT.
fn str_to_sha256_vec(user_key: &str) -> Vec<u8> {
    let bytes = user_key.trim().as_bytes().to_vec();
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&result[..]);
    hash_bytes.to_vec()
}
