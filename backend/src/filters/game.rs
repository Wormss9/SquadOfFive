use super::{add_struct, auth_validation};
use crate::{handlers, websocket::WsPlayers};
use deadpool_postgres::Pool;
use hmac::Hmac;
use sha2::Sha512;
use warp::{Filter, Rejection, Reply};

pub fn game(
    pool: Pool,
    key: Hmac<Sha512>,
    rooms: WsPlayers,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("api")
        .and(warp::path("game"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(auth_validation(key))
        .and(add_struct(pool))
        .and(add_struct(rooms))
        .and(warp::ws())
        .map(handlers::game::join_game)
}
