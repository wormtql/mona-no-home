use serde::{Serialize, Deserialize};
use crate::common::utils::get_current_timestamp;
use crate::models::user::User;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use crate::common::error::MyStringError;

#[derive(Serialize, Deserialize)]
pub struct LoginToken {
    pub iss: String,
    pub exp: usize,
    pub user_id: i32,
    pub admin: bool
}

pub fn encode_login_token(user: &User) -> Result<String, MyStringError> {
    dotenv().ok();

    let key = match env::var("JWT_TOKEN") {
        Ok(v) => v,
        Err(e) => return Err(MyStringError::from(&e.to_string()))
    };

    let claims = LoginToken {
        iss: String::from("mona"),
        exp: get_current_timestamp() + 5 * 60 * 1000,
        user_id: user.id,
        admin: user.admin
    };

    let token = match jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_bytes())) {
        Ok(v) => v,
        Err(e) => return Err(MyStringError::from(&e.to_string()))
    };
    Ok(token)
}

pub fn decode_login_token(token: &str) -> Result<LoginToken, MyStringError> {
    dotenv().ok();

    let key = match env::var("JWT_TOKEN") {
        Ok(v) => v,
        Err(e) => return Err(MyStringError::from(&e.to_string()))
    };

    let claims = match decode::<LoginToken>(token, &DecodingKey::from_secret(key.as_bytes()), &Validation::default()) {
        Ok(v) => v,
        Err(e) => return Err(MyStringError::from(&e.to_string()))
    };
    Ok(claims.claims)
}
