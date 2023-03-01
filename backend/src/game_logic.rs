use crate::{
    database::{Player, Room},
    websocket::{broadcast, join, message::MyMessage, send, WsPlayers},
};
use axum::extract::ws::Message;
use deadpool_postgres::Pool;
use tokio::sync::mpsc::UnboundedSender;

pub mod play;

pub async fn handle_join(
    room: &str,
    player: &Player,
    pool: Pool,
    players: WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) {
    broadcast(MyMessage::joined(player.id), &room, &players).await;
    send(MyMessage::cards(player.cards.clone()), tx);
    let room = match Room::get(pool.clone(), room).await {
        Ok(r) => r,
        Err(_) => return,
    };
    let players = players.read().await;
    let online = players
        .iter()
        .filter(|(player, _)| player.room == room.ulid)
        .map(|(player, _)| player.id)
        .collect();

    match room.get_players(pool).await {
        Ok(p) => p,
        Err(_) => return,
    }
    .iter()
    .map(|player| (player.id, player.cards.len() as i32))
    .for_each(|(id, ammount)| send(MyMessage::card_amount(id, ammount), tx));

    send(MyMessage::table(room.play), tx);
    send(MyMessage::turn(room.turn), tx);
    send(MyMessage::online(online), tx);
}

pub async fn handle_message(
    result: MyMessage,
    room: &str,
    player: &Player,
    pool: Pool,
    players: WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) {
    // result.as_string()
}
