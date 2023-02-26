use std::net::SocketAddr;
mod database;
use authorization::get_key;
use database::get_pool;
use layers::cors;
use websocket::WsPlayers;
mod authorization;
mod error;
mod game_logic;
mod handlers;
mod layers;
mod routes;
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
