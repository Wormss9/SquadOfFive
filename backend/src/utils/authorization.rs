use crate::database::{game_user::UserIdentification, GameUser};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::env;

pub type Key = Hmac<Sha512>;

pub fn hash_password(password: String) -> String {
    bcrypt::hash(password).unwrap()
}
pub fn verify_password(pass: String, hash: &str) -> bool {
    bcrypt::verify(pass, hash)
}
pub fn get_key() -> Key {
    Hmac::new_from_slice(
        env::var("LOGIN_SECRET")
            .expect("Missing LOGIN_SECRET")
            .as_bytes(),
    )
    .expect("Failed to create key")
}
pub fn create_token(player: GameUser, key: Key) -> Result<Authorization, jwt::Error> {
    let header = Header {
        algorithm: AlgorithmType::Hs512,
        ..Default::default()
    };
    let token = Token::new(header, player.get_identification()).sign_with_key(&key)?;
    Ok(Authorization {
        Authorization: token.as_str().to_owned(),
    })
}

pub fn authorize_token(key: &Key, token: String) -> Result<UserIdentification, jwt::Error> {
    token.verify_with_key(key)
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Login {
    pub name: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Authorization {
    pub Authorization: String,
}
