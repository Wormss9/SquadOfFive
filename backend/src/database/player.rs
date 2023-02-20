use crate::{game::play::Card, rejection::MyRejection};

use super::{initialize_client, Database};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::{Error, Row};
use warp::Rejection;

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
    async fn create_table(pool: Pool) -> Result<(), Error> {
        let client = pool.get().await.expect("Failed to connect to db");
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS player (
                    id          SERIAL PRIMARY KEY,
                    game_user   INT REFERENCES game_user(id),
                    room        TEXT NOT NULL REFERENCES room(ulid),
                    cards       JSON[] DEFAULT array[]::JSON[],
                    points      INT DEFAULT 0,
                    turn        INT DEFAULT 0
                );
                CREATE INDEX IF NOT EXISTS game_user_index ON player(game_user);
                CREATE INDEX IF NOT EXISTS room_index ON player(room);",
            )
            .await
    }
}

impl Player {
    pub async fn create(
        pool: Pool,
        room: &str,
        cards: &Vec<Card>,
        turn: i32,
    ) -> Result<Player, Rejection> {
        let row = initialize_client(pool)
            .await?
            .query_one(
                "INSERT INTO player (room, cards, turn) VALUES ($1, $2, $3)  RETURNING *;",
                &[&room, cards, &turn],
            )
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(Player::from(row))
    }
    pub async fn get(pool: Pool, room: &str, user: &str) -> Result<Option<Self>, Rejection> {
        let row = initialize_client(pool)
            .await?
            .query_opt(
                "SELECT * FROM player WHERE room = ($1) AND game_user = ($2);",
                &[&room, &user],
            )
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(row.map(Player::from))
    }
    pub async fn get_all(pool: Pool, room: &str) -> Result<Vec<Self>, Rejection> {
        let rows = initialize_client(pool)
            .await?
            .query("SELECT * FROM player WHERE room = ($1);", &[&room])
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(rows.into_iter().map(Player::from).collect())
    }
    pub async fn set_user(&self, pool: Pool, user: i32) -> Result<(), Rejection> {
        initialize_client(pool)
            .await?
            .execute(
                "UPDATE player SET game_user = $1 WHERE id = $2 AND turn = $3",
                &[&user, &self.id, &self.turn],
            )
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(())
    }
}
