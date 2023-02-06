use deadpool_postgres::Pool;
use hmac::{Hmac, Mac};
use http::StatusCode;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::env;
use warp::{reply::WithStatus, Filter, Rejection};

use crate::rejection::MyRejection;

use super::database::Player;

pub fn authorization() -> impl Filter<Extract = (Option<Player>,), Error = warp::Rejection> + Clone
{
    warp::header::header::<String>("Authorization").map(|x| -> Option<Player> {
        Some(Player {
            avatar: "".to_owned(),
            nick: "".to_owned(),
            id: 0,
            name: None,
            password: None,
            steam_id: None,
        })
    })
}

pub async fn test() -> Result<WithStatus<String>, Rejection> {
    Ok(warp::reply::with_status(
        "token.as_str().to_owned()".to_owned(),
        StatusCode::OK,
    ))
}

pub async fn login(login: Login, pool: Pool) -> Result<WithStatus<String>, Rejection> {
    let player = match match Player::get(pool, &login.name).await {
        Ok(p) => p,
        Err(_) => return Err(MyRejection::new(StatusCode::INTERNAL_SERVER_ERROR, "1")),
    } {
        Some(p) => p,
        None => return Err(MyRejection::code(StatusCode::NOT_FOUND)),
    };
    let player_password = match &player.password {
        Some(x) => x,
        None => {
            return Err(MyRejection::new(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "2",
            ))
        }
    };
    if !bcrypt::verify(login.password, player_password) {
        return Err(MyRejection::code(http::StatusCode::BAD_REQUEST));
    }
    let key: Hmac<Sha512> = Hmac::new_from_slice(
        env::var("LOGIN_SECRET")
            .expect("Missing LOGIN_SECRET")
            .as_bytes(),
    )
    .expect("Authorization token key creation error");
    let header = Header {
        algorithm: AlgorithmType::Hs512,
        ..Default::default()
    };
    // let mut claims = BTreeMap::new();
    // claims.insert("sub", "someone");
    match Token::new(header, player)
        .sign_with_key(&key)
        .map_err(|e| -> String { e.to_string() })
    {
        Ok(token) => Ok(warp::reply::with_status(
            token.as_str().to_owned(),
            StatusCode::OK,
        )),
        Err(_) => Err(MyRejection::new(StatusCode::INTERNAL_SERVER_ERROR, "3")),
    }
}

pub fn hash_password(password: String) -> String {
    bcrypt::hash(password).unwrap()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Login {
    pub name: String,
    pub password: String,
}

// fn main() {
//     let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
//     let header = Header {
//         algorithm: AlgorithmType::Hs384,
//         ..Default::default()
//     };
//     let mut claims = BTreeMap::new();
//     claims.insert("sub", "someone");
//     let token = Token::new(header, claims).sign_with_key(&key)?;
//     assert_eq!(token.as_str(), "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIn0.WM_WnPUkHK6zm6Wz7zk1kmIxz990Te7nlDjQ3vzcye29szZ-Sj47rLNSTJNzpQd_");
// }
// fn main2() {
//     let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret")?;
//     let token_str = "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIn0.WM_WnPUkHK6zm6Wz7zk1kmIxz990Te7nlDjQ3vzcye29szZ-Sj47rLNSTJNzpQd_";
//     let token: Token<Header, BTreeMap<String, String>, _> = token_str.verify_with_key(&key)?;
//     let header = token.header();
//     let claims = token.claims();
//     assert_eq!(header.algorithm, AlgorithmType::Hs384);
//     assert_eq!(claims["sub"], "someone");
// }
