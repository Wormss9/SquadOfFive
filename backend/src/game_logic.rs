use self::play::deal_cards;
use crate::{
    database::{commit, initialize_client, initialize_transaction, Player, Room},
    websocket::{
        broadcast,
        message::{MessageType, MyMessage},
        send, WsPlayers,
    },
};
use axum::extract::ws::Message;
use deadpool_postgres::{Pool, Transaction};
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

    match room.get_players(pool).await {
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
    let mut room = Room::get(pool, room)
        .await
        .map_err(|_| "Room update error".to_owned())?;
    let mut player = Player::get(pool, &player.id)
        .await
        .map_err(|_| "Player update error".to_owned())?;
    if room.turn != player.turn {
        return Err("Not your turn".to_owned());
    }
    let mut client = initialize_client(pool)
        .await
        .map_err(|_| "Error getting client".to_owned())?;
    let transaction = initialize_transaction(&mut client)
        .await
        .map_err(|_| "Error getting transaction".to_owned())?;
    match &result.kind as &str {
        "skip" => {
            if room.play.is_empty() {
                return Err("Empty table".to_owned());
            }
            update_turn(&transaction, &mut room, players).await?;
            commit(transaction)
                .await
                .map_err(|_| "Error commiting transaction".to_owned())?;
            Ok(())
        }
        "play" => {
            let play_cards = if let MessageType::Cards(cards) = result.message {
                cards
            } else {
                return Err("Bad cards".to_owned());
            };

            let new_hand =
                subtract(&player.cards, &play_cards).map_err(|_| "Not your cards".to_owned())?;

            let table = Play::new(room.play.clone());
            let play = Play::new(play_cards.clone());
            if !play.beats(&table) {
                return Err("Wrong play".to_owned());
            }
            player.cards = new_hand.clone();
            player
                .update(&transaction)
                .await
                .map_err(|_| "Player update error".to_owned())?;
            if !new_hand.is_empty() {
                room.play = play_cards.clone();
                room.last_turn = player.turn;
                room.increment_turn()
                    .update(&transaction)
                    .await
                    .map_err(|_| "Room update error".to_owned())?;
                send(MyMessage::cards(player.cards.clone()), tx);
                broadcast(MyMessage::table(play_cards), &room, players).await;
                broadcast(
                    MyMessage::card_amount(player.id, player.cards.len() as i32),
                    &room,
                    players,
                )
                .await;
                broadcast(MyMessage::turn(room.turn), &room, players).await;
                commit(transaction)
                    .await
                    .map_err(|_| "Error commiting transaction".to_owned())?;
                Ok(())
            } else {
                let new_cards = deal_cards();
                room.play = vec![];
                room.update(&transaction)
                    .await
                    .map_err(|_| "Room update error".to_owned())?;
                player.cards = new_hand;
                player
                    .update(&transaction)
                    .await
                    .map_err(|_| "Player update error".to_owned())?;
                let mut r_players = room
                    .get_players(pool)
                    .await
                    .map_err(|_| "Player update error".to_owned())?;
                let mut endgame = false;
                for (player, cards) in r_players.iter_mut().zip(new_cards) {
                    player.points += card_amount_to_points(player.cards.len());
                    if player.points >= 100 {
                        endgame = true;
                    }
                    player.cards = cards;
                    player
                        .update(&transaction)
                        .await
                        .map_err(|_| "Player update error".to_owned())?;
                }
                broadcast(MyMessage::end_play(), &room, players).await;
                if endgame {
                    room.ended = true;
                    room.update(&transaction)
                        .await
                        .map_err(|_| "Room update error".to_owned())?;
                    broadcast(MyMessage::end_game(), &room, players).await;
                };
                commit(transaction)
                    .await
                    .map_err(|_| "Error commiting transaction".to_owned())?;
                Ok(())
            }
        }
        _ => Err("Unknown message".to_owned()),
    }
}

async fn update_turn(
    transaction: &Transaction<'_>,
    room: &mut Room,
    players: &WsPlayers,
) -> Result<(), String> {
    room.increment_turn();
    if room.last_turn == room.turn {
        room.play = vec![];
        broadcast(MyMessage::table(vec![]), room, players).await;
    }
    room.update(transaction)
        .await
        .map_err(|_| "Error updating turn".to_owned())?;
    broadcast(MyMessage::turn(room.turn), room, players).await;
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

fn card_amount_to_points(cards: usize) -> i32 {
    if cards == 16 {
        return 5 * cards as i32;
    }
    if cards >= 14 {
        return 4 * cards as i32;
    }
    if cards >= 11 {
        return 3 * cards as i32;
    }
    if cards >= 8 {
        return 2 * cards as i32;
    }
    cards as i32
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
