use crate::database::{player::PublicPlayer, Player, Room};
use axum::extract::ws::{Message, WebSocket};
use deadpool_postgres::Pool;
use futures::StreamExt;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub type WsPlayers =
    Arc<RwLock<HashMap<PublicPlayer, mpsc::UnboundedSender<Result<Message, axum::Error>>>>>;

pub async fn join(room: Room, player: Player, pool: Pool, players: WsPlayers, socket: WebSocket) {
    let ulid = &room.ulid;
    // Establishing a connection
    let (user_tx, mut user_rx) = socket.split();
    let (tx, rx) = mpsc::unbounded_channel();

    let rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(rx.forward(user_tx));
    players.write().await.insert(player.public(), tx);

    // Reading and broadcasting messages
    while let Some(result) = user_rx.next().await {
        broadcast_msg(result.expect("Failed to fetch message"), ulid, &players).await;
    }

    // Disconnect
    disconnect(player.public(), &players).await;
}

pub async fn broadcast_msg(msg: Message, room: &str, players: &WsPlayers) {
    if msg.to_text().is_ok() {
        let players = players.read().await;
        for (receiver, tx) in players.iter().clone() {
            if room == receiver.room {
                tx.send(Ok(msg.clone())).expect("Failed to send message");
            }
        }
    }
}

pub async fn disconnect(player: PublicPlayer, players: &WsPlayers) {
    players.write().await.remove(&player);
}
