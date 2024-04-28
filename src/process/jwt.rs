use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::ALLOWED_JWT_AUDIENCES;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub aud: String,
    pub exp: i64,
}

pub struct Jwt;

impl Jwt {
    pub fn process_sign(user_key: &str, claims: &JwtClaims) -> anyhow::Result<String> {
        encode(&Header::default(), claims, &EncodingKey::from_secret(user_key.as_bytes()))
            .map_err(|err| anyhow::anyhow!("ERROR JWT signing: {}", err))
    }

    pub fn process_verify(user_key: &str, token: &str) -> anyhow::Result<String> {
        let mut validation = Validation::default();
        validation.set_audience(&ALLOWED_JWT_AUDIENCES);
        decode::<JwtClaims>(&token, &DecodingKey::from_secret(user_key.as_bytes()), &validation)
            .map(|data| format!("Verified: {:?}", data.claims))
            .map_err(|err| anyhow::anyhow!("ERROR JWT verifying: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_sign_and_verify() {
        let key = "test-key";
        let bad_key = "bad-key";
        let claims = JwtClaims {
            sub: "subject".to_string(),
            aud: ALLOWED_JWT_AUDIENCES[0].to_string(),
            exp: 25246260000,
        };

        let signed_token = Jwt::process_sign(key, &claims).unwrap();
        assert!(!signed_token.is_empty(), "The token should not be empty");

        let result = Jwt::process_verify(key, &signed_token).unwrap();
        assert_eq!(result, format!("Verified: {:?}", claims), "Claims should match");

        let result = Jwt::process_verify(bad_key, &signed_token);
        assert!(result.is_err(), "Should fail with the wrong key");
    }
}
