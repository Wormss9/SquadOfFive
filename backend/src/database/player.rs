use super::{initialize_client, Database};
use crate::{game_logic::play::Card, utils::error::Error};
use async_trait::async_trait;
use deadpool_postgres::{Pool, Transaction};
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub id: i32,
    pub game_user: Option<i32>,
    pub room: String,
    pub cards: Vec<Card>,
    pub points: i32,
    pub turn: i32,
}

impl From<Row> for Player {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            game_user: row.get("game_user"),
            room: row.get("room"),
            cards: row.get("cards"),
            points: row.get("points"),
            turn: row.get("turn"),
        }
    }
}

#[async_trait]
impl Database for Player {
    async fn create_table(transaction: &Transaction<'_>) -> Result<(), Error> {
        transaction
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS player (
                    id          SERIAL PRIMARY KEY,
                    game_user   INT REFERENCES game_user(id) ON delete set null,
                    room        TEXT NOT NULL REFERENCES room(ulid) ON delete cascade,
                    cards       JSON[] DEFAULT array[]::JSON[],
                    points      INT DEFAULT 0,
                    turn        INT DEFAULT 0
                );
                CREATE INDEX IF NOT EXISTS game_user_index ON player(game_user);
                CREATE INDEX IF NOT EXISTS room_index ON player(room);",
            )
            .await
            .map_err(Error::from_db)
    }
}

impl Player {
    pub async fn create(
        transaction: &Transaction<'_>,
        room: &str,
        cards: &Vec<Card>,
        turn: i32,
    ) -> Result<Player, Error> {
        let row = transaction
            .query_one(
                "INSERT INTO player (room, cards, turn) VALUES ($1, $2, $3)  RETURNING *;",
                &[&room, cards, &turn],
            )
            .await
            .map_err(Error::from_db)?;
        Ok(Player::from(row))
    }
    pub async fn get(pool: &Pool, id: &i32) -> Result<Self, Error> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM player WHERE id = ($1);", &[&id])
            .await
            .map_err(Error::from_db)?;
        row.map(Player::from)
            .ok_or_else(|| Error::code(StatusCode::UNAUTHORIZED))
    }
    pub async fn update(&self, transaction: &Transaction<'_>) -> Result<(), Error> {
        let row = transaction
            .execute(
                "UPDATE player SET game_user = $1, cards = $2, points = $3, turn = $4 WHERE id = $5",
                &[&self.game_user,&self.cards,&self.points,&self.turn, &self.id],
            )
            .await
            .map_err( Error::from_db)?;
        if row != 1 {
            return Err(Error::message(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Updated {} rows", row),
            ));
        }
        Ok(())
    }
    pub fn public(&self) -> PublicPlayer {
        PublicPlayer {
            id: self.id,
            game_user: self.game_user,
            room: self.room.clone(),
            points: self.points,
            turn: self.turn,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq)]
pub struct PublicPlayer {
    pub id: i32,
    pub game_user: Option<i32>,
    pub room: String,
    pub points: i32,
    pub turn: i32,
}

impl PartialEq for PublicPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for PublicPlayer {
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.id.hash(state);
        }
    }

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.game_user.hash(state);
        self.room.hash(state);
        self.points.hash(state);
        self.turn.hash(state);
    }
}
