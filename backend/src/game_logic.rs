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
    pool: &Pool,
    room: &Room,
    player: &Player,
    players: &WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) {
    broadcast(MyMessage::joined(player.id), room, players).await;
    send(MyMessage::cards(player.cards.clone()), tx);
    let players = players.read().await;
    let online = players
        .iter()
        .filter(|(player, _)| player.room == room.ulid)
        .map(|(player, _)| player.id)
        .collect();

    match room.get_players(pool.clone()).await {
        Ok(p) => p,
        Err(_) => return,
    }
    .iter()
    .map(|player| (player.id, player.cards.len() as i32))
    .for_each(|(id, ammount)| send(MyMessage::card_amount(id, ammount), tx));

    send(MyMessage::table(room.play.clone()), tx);
    send(MyMessage::turn(room.turn), tx);
    send(MyMessage::online(online), tx);
}

pub async fn handle_message(
    pool: &Pool,
    room: &str,
    player: &Player,
    result: MyMessage,
    players: &WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) -> Result<(), String> {
    let room = Room::get(pool.clone(), room)
        .await
        .map_err(|_| "Room update error".to_owned())?;
    let player = Player::get(pool.clone(), &player.id)
        .await
        .map_err(|_| "Player update error".to_owned())?;
    if room.turn != player.turn {
        return Err("Not your turn".to_owned());
    }

    if &result.kind == "skip" {
        if room.play.len() == 0 {
            return Err("Empty table".to_owned());
        }
        return update_turn(pool, &room, players).await;
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
            update_play(pool, &room, &old_hand, players).await?;
            update_last_turn(pool, &room, &player).await?;
            update_hand(pool, &room, &player, &new_hand, players, tx).await?;
            update_turn(pool, &room, players).await?;
        } else {
            send(MyMessage::error("Wrong play"), tx);
        }
    };
    Ok(())
}

async fn update_play(
    pool: &Pool,
    room: &Room,
    cards: &[Card],
    players: &WsPlayers,
) -> Result<(), String> {
    room.update_play(pool.clone(), cards.to_vec())
        .await
        .map_err(|_| "Error updating play".to_owned())?;
    broadcast(MyMessage::table(cards.to_vec()), room, players).await;
    Ok(())
}

async fn update_last_turn(pool: &Pool, room: &Room, player: &Player) -> Result<(), String> {
    room.update_last_turn(pool.clone(), player.turn)
        .await
        .map_err(|_| "Error updating last turn".to_owned())
}

async fn update_hand(
    pool: &Pool,
    room: &Room,
    player: &Player,
    new_hand: &Vec<Card>,
    players: &WsPlayers,
    tx: &UnboundedSender<Result<Message, axum::Error>>,
) -> Result<(), String> {
    player
        .update_hand(pool.clone(), new_hand.clone())
        .await
        .map_err(|_| "Error updating hand".to_owned())?;
    broadcast(
        MyMessage::card_amount(player.id, new_hand.len() as i32),
        room,
        players,
    )
    .await;
    send(MyMessage::cards(new_hand.clone()), tx);
    Ok(())
}

async fn update_turn(pool: &Pool, room: &Room, players: &WsPlayers) -> Result<(), String> {
    let turn = if room.turn == 3 { 0 } else { room.turn + 1 };
    match room.update_turn(pool.clone(), turn).await {
        Ok(_) => (),
        Err(_) => return Err("Error updating turn".to_owned()),
    }
    if room.last_turn == turn {
        update_play(pool, room, &[], players).await?;
    }
    broadcast(MyMessage::turn(turn), room, players).await;
    Ok(())
}

fn subtract<T: std::cmp::PartialEq + Clone>(
    subtrahends: &[T],
    minuends: &[T],
) -> Result<Vec<T>, String> {
    let mut result = subtrahends.to_vec();
    for minuend in minuends {
        let index = match result.iter().position(|subtrahend| subtrahend == minuend) {
            Some(i) => i,
            None => return Err("Not your cards".to_owned()),
        };
        result.remove(index);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;
    #[test]
    fn subtract_test() {
        let x = [1, 2, 3, 4, 5];
        let y = [1, 2, 4, 5];
        let z = [1, 1, 1, 2, 3];
        let a = [1, 1, 3];
        let error = Err("Not your cards".to_owned());

        assert_eq!(subtract(&x, &x), Ok(vec![]));
        assert_eq!(subtract(&y, &y), Ok(vec![]));
        assert_eq!(subtract(&z, &z), Ok(vec![]));
        assert_eq!(subtract(&a, &a), Ok(vec![]));
        assert_eq!(subtract(&y, &z), error);
        assert_eq!(subtract(&z, &a), Ok(vec![1, 2]));
    }
}
