use base64::{Engine as _, engine::general_purpose};

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
