use aes_gcm::{
  aead::{Aead, OsRng},
  AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use base64::{engine::general_purpose as Base64Engine, Engine};

use crate::models::my_errors::{unexpected_error::UnexpectedError, MyErrors, ToErr};

pub struct Crypto {
  pub encryption_key: Key<Aes256Gcm>,
}

impl Crypto {
  fn new() -> Result<Self, MyErrors> {
    let key_string = std::env::var("SSN_ENCRYPTION_KEY")?;

    if key_string.len() != 32 {
      return UnexpectedError::SHOULD_NOT_HAPPEN.to_err();
    }

    return Ok(Crypto {
      encryption_key: *Key::<Aes256Gcm>::from_slice(key_string.as_bytes()),
    });
  }

  pub fn encrypt(str_to_encrypt: &str) -> Result<String, MyErrors> {
    let encryption_key = Self::new()?.encryption_key;
    let cipher = Aes256Gcm::new(&encryption_key);
    let nonce = Aes256Gcm::generate_nonce(OsRng);

    let encrypted_str = cipher
      .encrypt(&nonce, str_to_encrypt.as_bytes())
      .map_err(|err| UnexpectedError::new(err.to_string().into()).to_my_error())?;

    let mut final_encryption = nonce.to_vec();
    final_encryption.extend_from_slice(&encrypted_str);

    Ok(Base64Engine::STANDARD.encode(&final_encryption))
  }

  pub fn decrypt(encrypted_str: &str) -> Result<String, MyErrors> {
    let decryption_key = Self::new()?.encryption_key;
    let cipher = Aes256Gcm::new(&decryption_key);

    let encrypted_data = Base64Engine::STANDARD
      .decode(encrypted_str)
      .map_err(|err| UnexpectedError::new(err.to_string().into()).to_my_error())?;

    if encrypted_data.len() < 12 {
      return UnexpectedError::SHOULD_NOT_HAPPEN.to_err();
    }

    let (nonce_bytes, encrypted_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_bytes = cipher
      .decrypt(&nonce, encrypted_data)
      .map_err(|err| UnexpectedError::new(err.to_string().into()).to_my_error())?;

    String::from_utf8(decrypted_bytes)
      .map_err(|err| UnexpectedError::new(err.to_string().into()).to_my_error())
  }
}
