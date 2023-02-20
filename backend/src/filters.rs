use std::convert::Infallible;

use hmac::Hmac;
use http::StatusCode;
use sha2::Sha512;
use warp::{Filter, Rejection};

use crate::{database::game_user::UserIdentification, authorization::authorize_token, rejection::MyRejection};

pub mod login;
pub mod room;

pub fn add_struct<T: Clone + std::marker::Send>(
    add: T,
) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || -> T { add.clone() })
}

pub fn auth_validation(
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