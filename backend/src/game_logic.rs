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
) -> Result<(), String> {
    let room = Room::get(pool.clone(), room)
        .await
        .map_err(|_| "Room error".to_owned())?;

    if room.turn != player.turn {
        return Err("Not your turn".to_owned());
    }

    if &result.kind == "skip" {
        return update_turn(&room, &pool, &player, &players).await;
    };

    if &result.kind == "play" {
        let old_hand = if let MessageType::Cards(cards) = result.message {
            cards
        } else {
            return Err("Bad cards".to_owned());
        };

        let new_hand =
            subtract(&player.cards, &old_hand).map_err(|_| "Not your cards".to_owned())?;

        let table = Play::new(room.play.clone());
        let play = Play::new(old_hand.clone());
        if play.beats(&table) {
            update_play(&room, old_hand.clone(), &pool, player, &players).await?;
            update_hand(&room, &player, &pool, new_hand, &players, tx).await?;
        } else {
            send(MyMessage::error("Wrong play"), tx);
        }
    };
    Ok(())
}

fn subtract(subtrahends: &[Card], minuends: &Vec<Card>) -> Result<Vec<Card>, String> {
    let mut result = subtrahends.to_owned();
    for minuend in minuends {
        let index = match result.iter().position(|subtrahend| subtrahend == minuend) {
            Some(i) => i,
            None => return Err("Not your cards".to_owned()),
        };
        result.remove(index);
    }
    Ok(result)
}

async fn update_play(
    room: &Room,
    cards: Vec<Card>,
    pool: &Pool,
    player: &Player,
    players: &WsPlayers,
) -> Result<(), String> {
    room.update_play(pool.clone(), cards.clone())
        .await
        .map_err(|_| "Error updating play".to_owned())?;
    broadcast(MyMessage::table(cards), &room.ulid, players).await;
    room.update_last_turn(pool.clone(), player.turn)
        .await
        .map_err(|_| "Error updating last turn".to_owned())?;
    Ok(())
}

async fn update_hand(
    room: &Room,
    player: &Player,
    pool: &Pool,
    new_hand: Vec<Card>,
    players: &WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) -> Result<(), String> {
    player
        .update_hand(pool.clone(), new_hand.clone())
        .await
        .map_err(|_| "Error updating hand".to_owned())?;
    broadcast(
        MyMessage::card_amount(player.id, new_hand.len() as i32),
        &room.ulid,
        &players,
    )
    .await;
    send(MyMessage::cards(new_hand), tx);
    Ok(())
}

async fn update_turn(
    room: &Room,
    pool: &Pool,
    player: &Player,
    players: &WsPlayers,
) -> Result<(), String> {
    let turn = if room.turn == 3 { 0 } else { room.turn + 1 };
    match room.update_turn(pool.clone(), turn).await {
        Ok(_) => (),
        Err(_) => return Err("Error updating turn".to_owned()),
    }
    if room.last_turn == turn {
        update_play(&room, vec![], pool, player, players).await?;
    }
    broadcast(MyMessage::turn(turn), &room.ulid, players).await;
    Ok(())
}
