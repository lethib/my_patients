use base64::{engine::general_purpose::STANDARD, Engine};
use cucumber::{given, then, when};
use my_patients::services::crypto::Crypto;

use crate::AppWorld;

#[given("a valid encryption key is configured")]
fn configure_encryption_key(_world: &mut AppWorld) {
    std::env::set_var("SSN_ENCRYPTION_KEY", "12345678901234567890123456789012");
}

#[when(expr = "I encrypt the value {string}")]
fn encrypt_value(world: &mut AppWorld, value: String) {
    world.crypto.encrypted = Some(Crypto::encrypt(&value).unwrap());
}

#[when(expr = "I encrypt the value {string} a second time")]
fn encrypt_value_again(world: &mut AppWorld, value: String) {
    world.crypto.second_encrypted = Some(Crypto::encrypt(&value).unwrap());
}

#[then(expr = "I can decrypt and retrieve {string}")]
fn decrypt_and_verify(world: &mut AppWorld, expected: String) {
    let encrypted = world.crypto.encrypted.as_ref().unwrap();
    let decrypted = Crypto::decrypt(encrypted).unwrap();
    assert_eq!(decrypted, expected);
}

#[then("the two encrypted results are different")]
fn encrypted_values_differ(world: &mut AppWorld) {
    assert_ne!(world.crypto.encrypted, world.crypto.second_encrypted);
}

#[when(expr = "I try to decrypt {string}")]
fn try_decrypt_invalid(world: &mut AppWorld, value: String) {
    world.crypto.decrypt_failed = Crypto::decrypt(&value).is_err();
}

#[when("I try to decrypt a base64-encoded value that is too short")]
fn try_decrypt_too_short(world: &mut AppWorld) {
    // 5 bytes < 12 (minimum nonce size)
    let short_b64 = STANDARD.encode(b"short");
    world.crypto.decrypt_failed = Crypto::decrypt(&short_b64).is_err();
}

#[then("the decryption fails")]
fn decryption_should_fail(world: &mut AppWorld) {
    assert!(world.crypto.decrypt_failed, "decryption should have failed");
}

#[when(expr = "I hash {string} with salt {string}")]
fn hash_value(world: &mut AppWorld, value: String, salt: String) {
    world.crypto.hashed = Some(Crypto::hash(&value, &salt).unwrap());
}

#[when(expr = "I hash {string} with salt {string} again")]
fn hash_value_again(world: &mut AppWorld, value: String, salt: String) {
    world.crypto.second_hashed = Some(Crypto::hash(&value, &salt).unwrap());
}

#[then("the two hashes are identical")]
fn hashes_are_identical(world: &mut AppWorld) {
    assert_eq!(world.crypto.hashed, world.crypto.second_hashed);
}

#[then(expr = "the hash starts with {string}")]
fn hash_starts_with(world: &mut AppWorld, prefix: String) {
    let hash = world.crypto.hashed.as_ref().unwrap();
    assert!(
        hash.starts_with(&prefix),
        "expected hash to start with '{}', got '{}'",
        prefix,
        hash
    );
}
