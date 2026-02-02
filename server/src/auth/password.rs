use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e))? // <-- wrap the error manually for now.
        .to_string();

    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> anyhow::Result<bool> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!(e))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "mysecret123";

        let hashed = hash_password(password).expect("hashing failed");
        let valid = verify_password(&hashed, password).expect("verification failed");
        assert!(valid, "Password should verify correctly");

        let invalid = verify_password(&hashed, "wrongpassword").expect("verification failed");
        assert!(!invalid, "Wrong password should not verify");
    }
}
