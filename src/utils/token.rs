use jwt_simple::{claims, prelude::*, reexports::anyhow::Ok};
use std::fmt::Error;

pub struct JWT {
    key: HS256Key,
}

impl JWT {
    pub fn create_token(&self) -> Result<String, jwt_simple::Error> {
        let claims = Claims::create(Duration::from_hours(2));
        let res = self.key.authenticate(claims)?;
        return Ok(res);
    }

    pub fn verify_token(&self, t: String) -> Result<String, jwt_simple::Error> {
        let claims = self.key.verify_token::<NoCustomClaims>(&t, None)?;
        let id = claims.jwt_id;

        match id {
            Some(x) => Ok(x),
            None => return Ok("".to_string()),
        }
    }
}

pub fn init_jwt() -> JWT {
    let key = HS256Key::generate();
    return JWT { key: key };
}
