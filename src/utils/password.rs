use argon2::{
    password_hash::{
        self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use regex::Regex;
use std::error::Error;

pub fn hash_password(pass: String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(OsRng);

    let hash_res = Argon2::default().hash_password(pass.as_bytes(), &salt);

    let hash = match hash_res {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            return Err(e);
        }
    };
    Ok(hash)
}

pub fn validate_password(pass: String) -> Result<bool, Box<dyn std::error::Error>> {
    let has_uppercase = Regex::new(r"[A-Z]")?;
    let has_number = Regex::new(r"[0-9]")?;
    let has_special = Regex::new(r"[!@#$%^&*(),.?\:{}|<>~]")?;

    if has_uppercase.is_match(&pass) && has_number.is_match(&pass) && has_special.is_match(&pass) {
        return Ok(true);
    };
    return Ok(false);
}

pub fn verify_password(
    password: String,
    hashed_password: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let parsed_hash =
        PasswordHash::new(&hashed_password).map_err(|e| Box::<dyn Error>::from(e.to_string()))?;

    let ver_res = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    match ver_res {
        Ok(_) => {
            return Ok(true);
        }
        Err(_) => {
            return Ok(false);
        }
    }
}
