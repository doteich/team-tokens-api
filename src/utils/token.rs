use jwt_simple::{claims, prelude::*, reexports::anyhow::Ok};
use std::fmt::Error;

pub struct JWT {
    key: HS256Key,
}

impl JWT {
    pub fn create_token(&self, user_id: String) -> Result<String, jwt_simple::Error> {
        let claims = Claims::create(Duration::from_hours(2)).with_subject(user_id);
        let res = self.key.authenticate(claims)?;
        return Ok(res);
    }

    pub fn verify_token(&self, t: &str) -> Result<String, jwt_simple::Error> {
        let claims = self.key.verify_token::<NoCustomClaims>(t, None)?;

        let user_id = claims.subject;

        match user_id {
            None => return Ok(String::new()),
            Some(id) => return Ok(id),
        }
    }
}

pub fn init_jwt() -> JWT {
    let key = HS256Key::generate();
    return JWT { key: key };
}
