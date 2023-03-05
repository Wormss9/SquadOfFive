use self::play::Card;
use crate::{
    database::{Player, Room},
    websocket::{
        broadcast,
        message::{MessageType, MyMessage},
        send, WsPlayers,
    },
};
use axum::extract::ws::Message;
use deadpool_postgres::Pool;
use play::Play;
use tokio::sync::mpsc::UnboundedSender;

pub mod play;

pub async fn handle_join(
    room: &str,
    player: &Player,
    pool: Pool,
    players: WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) {
    broadcast(MyMessage::joined(player.id), room, &players).await;
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
    let room = match Room::get(pool.clone(), room).await {
        Ok(r) => r,
        Err(_) => return,
    };

    if room.turn != player.turn {
        send(MyMessage::error("Not your turn"), tx);
        return;
    }

    if &result.kind == "skip" {
        let turn = if room.turn == 3 { 0 } else { room.turn + 1 };
        match room.update_turn(pool, turn).await {
            Ok(_) => (),
            Err(_) => send(MyMessage::error("Error updating turn"), tx),
        };
        return;
    };

    if &result.kind == "play" {
        let cards = if let MessageType::Cards(cards) = result.message {
            cards
        } else {
            send(MyMessage::error("Bad cards"), tx);
            return;
        };

        let new_hand = match subtract(&player.cards, &cards) {
            Ok(c) => c,
            Err(_) => {
                send(MyMessage::error("Not your cards"), tx);
                return;
            }
        };

        let table = Play::new(room.play.clone());
        let play = Play::new(cards.clone());
        if play.beats(&table) {
            let turn = if room.turn == 3 { 0 } else { room.turn + 1 };
            match room.update_turn(pool.clone(), turn).await {
                Ok(_) => (),
                Err(_) => send(MyMessage::error("Error updating turn"), tx),
            };
            match room.update_play(pool.clone(), cards.clone()).await {
                Ok(_) => (),
                Err(_) => send(MyMessage::error("Error updating play"), tx),
            };
            match player.update_hand(pool, new_hand.clone()).await {
                Ok(_) => (),
                Err(_) => send(MyMessage::error("Error updating hand"), tx),
            };
            broadcast(MyMessage::turn(turn), &room.ulid, &players).await;
            broadcast(MyMessage::table(cards), &room.ulid, &players).await;
            broadcast(
                MyMessage::card_amount(player.id, new_hand.len() as i32),
                &room.ulid,
                &players,
            )
            .await;
            send(MyMessage::cards(new_hand), tx)
        } else {
            send(MyMessage::error("Play too weak"), tx);
        }
    };
}

fn subtract(subtrahends: &[Card], minuends: &Vec<Card>) -> Result<Vec<Card>, ()> {
    let mut result = subtrahends.to_owned();
    for minuend in minuends {
        let index = match result.iter().position(|subtrahend| subtrahend == minuend) {
            Some(i) => i,
            None => return Err(()),
        };
        result.remove(index);
    }
    Ok(result)
}
