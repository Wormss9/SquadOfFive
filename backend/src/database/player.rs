use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde_derive::{Serialize, Deserialize};
use tokio_postgres::{Row, Error};
use super::Database;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub steam_id: Option<i32>,
    pub nick: String,
    pub avatar: String,
}

impl From<Row> for Player{
    fn from(row: Row) -> Self {
        print!("{:?}", row);
        Self {
            id: row.get("id"),
            name: row.get("name"),
            password: row.get("password"),
            steam_id: row.get("steam_id"),
            nick: row.get("nick"),
            avatar: row.get("avatar"),
        }
    }
}

#[async_trait]
impl Database for Player {
    async fn create_table(pool: Pool) -> Result<(), Error> {
        let mut client = pool.get().await.unwrap();
        client.batch_execute(
            "CREATE TABLE IF NOT EXISTS player (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR UNIQUE NULLS NOT DISTINCT,
    password        VARCHAR,
    steam_id        INT UNIQUE NULLS NOT DISTINCT,
    nick            VARCHAR NOT NULL,
    avatar          VARCHAR NOT NULL DEFAULT ''
    );",
        ).await?;
        client.batch_execute("CREATE INDEX IF NOT EXISTS name_index ON player(name);").await?;
        client.batch_execute("CREATE INDEX IF NOT EXISTS steam_index ON player(steam_id);").await
    }
}


impl Player{
    pub async fn create(
        pool: Pool,
        name: &str,
        password: &str,
    ) -> Result<u64, Error> {
        let mut client = pool.get().await.unwrap();
        client.execute(
            "INSERT INTO player (name, nick, password) VALUES ($1, $1, $2);",
            &[&name, &password],
        ).await
    }
    pub async fn create_steam(
        pool: Pool,
        steam_id: &str,
        nick: &str,
        avatar: &str,
    ) -> Result<u64, Error> {
        let client = pool.get().await.unwrap();
        client.execute(
            "INSERT INTO player (steamId, nick,avatar) VALUES ($1, $2, $3)",
            &[&steam_id, &nick, &avatar],
        ).await
    }
    pub async fn get(pool: Pool, name: &str) -> Result<Option<Self>, Error> {
        let client = pool.get().await.unwrap();
        let row = client.query_opt("SELECT * FROM player WHERE name = ($1);", &[&name]).await?;
    
        Ok(match row{
            Some(row) => Some(Player::from(row)),
            None => None,
        })
    }
    pub async fn get_steam(pool: Pool, steam_id: &str) -> Result<Option<Self>, Error> {
        let mut client = pool.get().await.unwrap();
        let row = client.query_opt("SELECT * FROM player WHERE steamId = ($1);", &[&steam_id]).await?;
    
        Ok(match row{
            Some(row) => Some(Player::from(row)),
            None => None,
        })
    }
    
}