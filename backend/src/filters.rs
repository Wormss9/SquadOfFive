use deadpool_postgres::Pool;
use hmac::Hmac;
use sha2::Sha512;
use warp::reply::{WithHeader, WithStatus};
use warp::Filter;
// use serde_derive::{Deserialize, Serialize};
use crate::authorization::Login;
// use crate::database::Player;
use crate::handlers;
use crate::utils::json_body;

fn db_filter(
    pool: Pool,
) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || -> Pool { pool.clone() })
}

fn key_filter(
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (Hmac<Sha512>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || -> Hmac<Sha512> { key.clone() })
}

pub fn get_login(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (WithHeader<String>,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(db_filter(pool))
        .and(key_filter(key))
        .and_then(handlers::login)
}

pub fn put_register(
    pool: Pool,
) -> impl Filter<Extract = (WithStatus<String>,), Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path("api"))
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(json_body::<Login>())
        .and(db_filter(pool))
        .and_then(handlers::register)
}
