use super::database::Player;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::env;

pub fn hash_password(password: String) -> String {
    bcrypt::hash(password).unwrap()
}
pub fn verify_password(pass: String, hash: &str) -> bool {
    bcrypt::verify(pass, hash)
}
pub fn get_key() -> Hmac<Sha512> {
    Hmac::new_from_slice(
        env::var("LOGIN_SECRET")
            .expect("Missing LOGIN_SECRET")
            .as_bytes(),
    )
    .expect("Failed to create key")
}
pub fn create_token(player: Player, key: Hmac<Sha512>) -> Result<String, jwt::Error> {
    let header = Header {
        algorithm: AlgorithmType::Hs512,
        ..Default::default()
    };
    let token = Token::new(header, player.get_identification()).sign_with_key(&key)?;
    Ok(token.as_str().to_owned())
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Login {
    pub name: String,
    pub password: String,
}
