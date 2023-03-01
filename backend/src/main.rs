use database::get_pool;
use std::net::SocketAddr;
use utils::{authorization::get_key, layers::cors};
use websocket::WsPlayers;

mod database;
mod game_logic;
mod handlers;
mod routes;
mod utils;
mod websocket;

#[tokio::main]
async fn main() {
    let pool = get_pool().await;
    let key = get_key();
    let players = WsPlayers::default();

    let app = routes::routes(pool, key, players).layer(cors());

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
