use super::{initialize_client, Database, Player};
use crate::filters::rejection::MyRejection;
use crate::game_logic::play::Card;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::{Error, Row};
use ulid::Ulid;
use warp::Rejection;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Room {
    pub ulid: String,
    pub host: i32,
    pub play: Vec<Card>,
    pub turn: i32,
}

impl From<Row> for Room {
    fn from(row: Row) -> Self {
        Self {
            ulid: row.get("ulid"),
            host: row.get("host"),
            play: row.get("play"),
            turn: row.get("turn"),
        }
    }
}

#[async_trait]
impl Database for Room {
    async fn create_table(pool: Pool) -> Result<(), Error> {
        let client = pool.get().await.expect("Failed to connect to db");
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS room (
                    ulid            VARCHAR(26) PRIMARY KEY,
                    host            INT REFERENCES game_user(id),
                    play            JSON[] DEFAULT array[]::JSON[],
                    turn            INT DEFAULT 1
                );",
            )
            .await
    }
}

impl Room {
    pub async fn create(pool: Pool, host: i32) -> Result<Room, Rejection> {
        let id = Ulid::new().to_string();
        let row = initialize_client(pool)
            .await?
            .query_one(
                "INSERT INTO room (ulid, host) VALUES ($1, $2) RETURNING *;",
                &[&id, &host],
            )
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(Room::from(row))
    }
    pub async fn get(pool: Pool, id: &str) -> Result<Self, Rejection> {
        let row = initialize_client(pool)
            .await?
            .query_opt("SELECT * FROM room WHERE ulid = ($1);", &[&id])
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;
        row
            .map(Room::from)
            .ok_or(MyRejection::code(StatusCode::NOT_FOUND))
    }
    pub async fn get_my(pool: Pool, host: i32) -> Result<Vec<Self>, Rejection> {
        let rows = initialize_client(pool)
            .await?
            .query("SELECT * FROM room WHERE host = ($1);", &[&host])
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(rows.into_iter().map(Room::from).collect())
    }
    pub async fn get_joined(pool: Pool, host: i32) -> Result<Vec<Self>, Rejection> {
        let rows = initialize_client(pool)
            .await?
            .query("select * from room where ulid in (select room from player where game_user = ($1));", &[&host])
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(rows.into_iter().map(Room::from).collect())
    }
    pub async fn get_players(&self, pool: Pool) -> Result<Vec<Player>, Rejection> {
        let rows = initialize_client(pool)
            .await?
            .query("SELECT * FROM player WHERE room = ($1);", &[&self.ulid])
            .await
            .map_err(MyRejection::code_fn(StatusCode::INTERNAL_SERVER_ERROR))?;

        Ok(rows.into_iter().map(Player::from).collect())
    }
}
