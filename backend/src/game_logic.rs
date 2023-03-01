use crate::{
    database::Player,
    websocket::{broadcast, join, message::MyMessage, send, WsPlayers},
};
use axum::extract::ws::Message;
use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender;

pub mod play;

pub async fn handle_join(
    room: &str,
    player: &Player,
    players: WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) {
    broadcast(MyMessage::joined(player.id), &room, &players).await;
    send(MyMessage::cards(player.cards.clone()), tx)
}

pub async fn handle_message(
    result: MyMessage,
    room: &str,
    player: &Player,
    pool: Pool,
    players: WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) {
}
