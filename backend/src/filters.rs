use crate::{database::game_user::UserIdentification, handlers::authorization::authorize_token};
use hmac::Hmac;
use http::StatusCode;
use rejection::MyRejection;
use sha2::Sha512;
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub mod game;
pub mod rejection;
pub mod room;
pub mod signing;
mod utils;

pub fn add_struct<T: Clone + std::marker::Send>(
    add: T,
) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || -> T { add.clone() })
}

pub fn cookie_auth(
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (UserIdentification,), Error = Rejection> + Clone {
    warp::cookie::<String>("token")
        .and(add_struct(key))
        .and_then(|token: String, key: Hmac<Sha512>| async move {
            match authorize_token(&key.clone(), token.clone()) {
                Ok(p) => Ok(p),
                Err(_) => Err(MyRejection::code(StatusCode::UNAUTHORIZED)),
            }
        })
}

pub fn header_auth(
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (UserIdentification,), Error = Rejection> + Clone {
    warp::header::<String>("Authentication")
        .and(add_struct(key))
        .and_then(|token: String, key: Hmac<Sha512>| async move {
            match authorize_token(&key.clone(), token.clone()) {
                Ok(p) => Ok(p),
                Err(_) => Err(MyRejection::code(StatusCode::UNAUTHORIZED)),
            }
        })
}
