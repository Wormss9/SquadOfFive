use crate::{
    database::{player::PublicPlayer, Player, Room},
    game_logic::{handle_join, handle_message},
};
use axum::{
    extract::ws::{Message, WebSocket},
    Error,
};
use deadpool_postgres::Pool;
use futures::StreamExt;
use message::MyMessage;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    RwLock,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub mod message;

pub type WsPlayers =
    Arc<RwLock<HashMap<PublicPlayer, UnboundedSender<Result<Message, axum::Error>>>>>;

pub async fn join(
    room: &Room,
    player: &Player,
    pool: Pool,
    players: &WsPlayers,
    socket: WebSocket,
) {
    // Establishing a connection
    let (user_tx, mut user_rx) = socket.split();
    let (tx, rx) = unbounded_channel();

    let rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(rx.forward(user_tx));
    players.write().await.insert(player.public(), tx.clone());

    handle_join(&pool, room, player, players, &tx).await;

    while let Some(result) = user_rx.next().await {
        let me = match match result {
            Ok(r) => {
                if let Message::Pong(_) = r {
                    continue;
                }
                MyMessage::try_from(r)
            }
            Err(x) => {
                send(MyMessage::error(("Report 1", x)), &tx);
                continue;
            }
        } {
            Ok(m) => m,
            Err(x) => {
                send(MyMessage::error(("Report 2", x)), &tx);
                continue;
            }
        };
        if let Err(message) =
            handle_message(&pool, &room.ulid, player, me, players, &tx).await
        {
            send(MyMessage::error(message), &tx);
        }
    }

    disconnect(player.public(), players).await;
    broadcast(MyMessage::disconnect(player.id), room, players).await;
}

pub async fn broadcast(msg: MyMessage, room: &Room, players: &WsPlayers) {
    let players = players.read().await;
    for (_receiver, tx) in players
        .iter()
        .filter(|(player, _)| player.room == room.ulid)
    {
        tx.send(Ok(msg.as_message()))
            .expect("Failed to send message");
    }
}

pub fn send(msg: MyMessage, tx: &UnboundedSender<Result<Message, Error>>) {
    tx.send(Ok(msg.as_message()))
        .expect("Failed to send message");
}

async fn disconnect(player: PublicPlayer, players: &WsPlayers) {
    players.write().await.remove(&player);
}
