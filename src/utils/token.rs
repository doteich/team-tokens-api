use jwt_simple::{prelude::*, reexports::anyhow::Ok};
use std::fmt::Error;

pub struct JWT {
    key: HS256Key,
}

pub impl JWT {
    fn create_key(&self) {
        self.key = HS256Key::generate();
    }

    fn create_token(&self) -> Result<String, Error> {
        let claims = Claims::create(Duration::from_hours(2));
        let res = key.authenticate(claims)?;
        return Ok(res);
    }

    pub fn verify_token(&self, t: String) -> Result<bool, Error> {

        let claims = self.key.verify_token::<NoCustomClaims>(&token, None)?;

    }
}
