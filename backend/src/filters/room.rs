use super::{add_struct, auth_validation};
use crate::handlers::room;
use deadpool_postgres::Pool;
use hmac::Hmac;
use sha2::Sha512;
use warp::{Filter, Rejection, Reply};

fn put_create(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::put()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::create)
}
fn patch_join(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::patch()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path::param())
        .and(warp::path("join"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::join)
}

fn get_room(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_one)
}

fn get_owned(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("rooms"))
        .and(warp::path("owned"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_owned)
}

fn get_joined(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("rooms"))
        .and(warp::path("joined"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_joined)
}

fn get_players(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("room"))
        .and(warp::path::param())
        .and(warp::path("players"))
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and_then(room::get_players)
}

pub fn room(
    pool: Pool,
    key: Hmac<Sha512>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    put_create(pool.clone(), key.clone())
        .or(patch_join(pool.clone(), key.clone()))
        .or(get_room(pool.clone(), key.clone()))
        .or(get_owned(pool.clone(), key.clone()))
        .or(get_joined(pool.clone(), key.clone()))
        .or(get_players(pool.clone(), key.clone()))
}
