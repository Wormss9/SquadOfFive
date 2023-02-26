use crate::{
    authorization::Key,
    handlers::{game, room, signing},
    websocket::WsPlayers,
};
use axum::{
    routing::{get, patch, put},
    Extension, Router,
};
use deadpool_postgres::Pool;

fn singning(pool: Pool, key: Key) -> Router {
    Router::new().route(
        "/api/signing/user",
        get(signing::user_login)
            .put(signing::user_register)
            .with_state((pool, key)),
    )
}

fn rooms(pool: Pool, key: Key) -> Router {
    Router::new()
        .route(
            "/api/rooms/owned",
            get(room::get_owned)
                .layer(Extension(key.clone()))
                .with_state(pool.clone()),
        )
        .merge(Router::new().route(
            "/api/rooms/joined",
            get(room::get_joined).layer(Extension(key)).with_state(pool),
        ))
}

fn room(pool: Pool, key: Key) -> Router {
    Router::new()
        .route(
            "/api/room/:id",
            patch(room::join)
                .layer(Extension(key.clone()))
                .with_state(pool.clone()),
        )
        .merge(Router::new().route(
            "/api/room",
            put(room::create).layer(Extension(key)).with_state(pool),
        ))
}

fn game(pool: Pool, key: Key, players: WsPlayers) -> Router {
    Router::new()
        .route("/api/game/:id", get(game::game).layer(Extension(key)))
        .with_state((pool, players))
}

pub fn routes(pool: Pool, key: Key, players: WsPlayers) -> Router {
    singning(pool.clone(), key.clone())
        .merge(rooms(pool.clone(), key.clone()))
        .merge(room(pool.clone(), key.clone()))
        .merge(game(pool, key, players))
}
