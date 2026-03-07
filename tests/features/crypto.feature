Feature: Sensitive data encryption and hashing
  As a system
  I want to protect sensitive patient data
  In order to ensure medical confidentiality

  Rule: AES-256-GCM encryption protects data and is reversible

    Scenario: Encrypting then decrypting retrieves the original value
      When I encrypt the value "1234567890123456"
      Then I can decrypt and retrieve "1234567890123456"

    Scenario: Encrypting the same value twice produces different results
      When I encrypt the value "1234567890123456"
      And I encrypt the value "1234567890123456" a second time
      Then the two encrypted results are different

    Scenario: Decrypting invalid data fails
      When I try to decrypt "this!is!not!valid!base64"
      Then the decryption fails

    Scenario: Decrypting data that is too short fails
      When I try to decrypt a base64-encoded value that is too short
      Then the decryption fails

  Rule: Argon2id hashing is deterministic with the same salt

    Scenario: Hashing the same value with the same salt always gives the same result
      When I hash "my_secret" with salt "test_salt_1234"
      And I hash "my_secret" with salt "test_salt_1234" again
      Then the two hashes are identical

    Scenario: The hash produces a value in Argon2 format
      When I hash "my_secret" with salt "test_salt_1234"
      Then the hash starts with "$argon2"
