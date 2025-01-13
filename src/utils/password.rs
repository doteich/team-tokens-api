use argon2::{
    password_hash::{
        self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

fn hash_password(pass: String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(OsRng);

    let hash_res = Argon2::default().hash_password(pass.as_bytes(), &salt);

    let hash = match hash_res {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            return Err(e);
        }
    };
    return Ok(hash);
}
