use crate::{
    handlers::{game, room, signing, user},
    utils::authorization::Key,
    websocket::WsPlayers,
};
use axum::{
    routing::{delete, get, patch, put},
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
                .delete(room::delete)
                .layer(Extension(key.clone()))
                .with_state(pool.clone()),
        )
        .merge(
            Router::new().route(
                "/api/room",
                put(room::create)
                    .layer(Extension(key.clone()))
                    .with_state(pool.clone()),
            ),
        )
        .merge(
            Router::new().route(
                "/api/room/:id/players",
                get(room::get_players)
                    .layer(Extension(key))
                    .with_state(pool),
            ),
        )
}

fn game(pool: Pool, key: Key, players: WsPlayers) -> Router {
    Router::new()
        .route("/api/game/:id", get(game::game).layer(Extension(key)))
        .with_state((pool, players))
}

fn user(pool: Pool, key: Key) -> Router {
    Router::new()
        .route(
            "/api/user",
            get(user::get_self).layer(Extension(key.clone())),
        )
        .with_state(pool.clone())
        .merge(
            Router::new()
                .route("/api/user/:id", get(user::get).layer(Extension(key)))
                .with_state(pool),
        )
}

pub fn routes(pool: Pool, key: Key, players: WsPlayers) -> Router {
    singning(pool.clone(), key.clone())
        .merge(rooms(pool.clone(), key.clone()))
        .merge(room(pool.clone(), key.clone()))
        .merge(game(pool.clone(), key.clone(), players))
        .merge(user(pool, key))
}
