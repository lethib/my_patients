use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub pid: String,
  pub exp: i64,
  pub iat: i64,
}

pub struct JwtService {
  encoding_key: EncodingKey,
  decoding_key: DecodingKey,
  validation: Validation,
}

impl JwtService {
  pub fn new(secret: &str) -> Self {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    Self {
      encoding_key: EncodingKey::from_secret(secret.as_bytes()),
      decoding_key: DecodingKey::from_secret(secret.as_bytes()),
      validation,
    }
  }

  pub fn generate_token(
    &self,
    pid: &str,
    expiration_seconds: u64,
  ) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::seconds(expiration_seconds as i64);

    let claims = Claims {
      pid: pid.to_string(),
      exp: exp.timestamp(),
      iat: now.timestamp(),
    };

    encode(&Header::default(), &claims, &self.encoding_key)
  }

  pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)?;
    Ok(token_data.claims)
  }
}
