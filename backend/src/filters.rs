use std::convert::Infallible;

use deadpool_postgres::Pool;
use hmac::Hmac;
use http::StatusCode;
use sha2::Sha512;
use warp::reply::{WithHeader, WithStatus};
use warp::{Filter, Rejection};
// use serde_derive::{Deserialize, Serialize};
use crate::authorization::{authorize_token, Login};
use crate::database::player::PlayerIdentification;
// use crate::database::Player;
use crate::handlers;
use crate::rejection::MyRejection;
use crate::utils::json_body;

pub fn add_struct<T: Clone + std::marker::Send>(
    add: T,
) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || -> T { add.clone() })
}

pub fn get_login(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (WithHeader<String>,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(add_struct(pool))
        .and(add_struct(key))
        .and_then(handlers::login)
}

pub fn put_register(
    pool: Pool,
) -> impl Filter<Extract = (WithStatus<String>,), Error = Rejection> + Clone {
    warp::put()
        .and(warp::path("api"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(add_struct(pool))
        .and_then(handlers::register)
}

fn auth_validation(
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (PlayerIdentification,), Error = Rejection> + Clone {
    warp::cookie::<String>("token")
        .and(add_struct(key))
        .and_then(|token: String, key: Hmac<Sha512>| async move {
            match authorize_token(&key.clone(), token.clone()) {
                Ok(p) => Ok(p),
                Err(_) => Err(MyRejection::code(StatusCode::UNAUTHORIZED)),
            }
        })
}

pub fn get_restricted(
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (WithStatus<String>,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("restricted"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and_then(handlers::restricted_handler)
}

