use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    #[serde(rename = "sub")]
    pub sub: String,
    #[serde(rename = "aud")]
    pub aud: String,
    #[serde(rename = "exp")]
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
        validation.set_audience(&crate::utils::ALLOWED_JWT_AUDIENCES);
        decode::<JwtClaims>(&token, &DecodingKey::from_secret(user_key.as_bytes()), &validation)
            .map(|data| format!("Verified: {:?}", data.claims))
            .map_err(|err| anyhow::anyhow!("ERROR JWT verifying: {}", err))
    }
}
