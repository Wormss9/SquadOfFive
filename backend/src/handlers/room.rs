use crate::{
    database::{game_user::UserIdentification, Player, Room},
    error::Error,
    game_logic::play::deal_cards,
};
use axum::{
    debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use deadpool_postgres::Pool;
use http::StatusCode;
use rand::seq::SliceRandom;

#[debug_handler]
pub async fn create(
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let room = Room::create(pool.clone(), user.id).await?;
    let deck = deal_cards();
    let mut players: Vec<Player> = Vec::new();
    for (x, cards) in deck.iter().enumerate() {
        players.push(Player::create(pool.clone(), &room.ulid, cards, x as i32).await?);
    }
    let player = players
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect::<Vec<_>>()[0];
    player.set_user(pool, user.id).await?;
    Ok(Json(room))
}

pub async fn get_owned(
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let rooms = Room::get_my(pool, user.id).await?;
    Ok(Json(rooms))
}

pub async fn get_joined(
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let rooms = Room::get_joined(pool, user.id).await?;
    Ok(Json(rooms))
}
pub async fn join(
    Path(ulid): Path<String>,
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let room = Room::get(pool.clone(), &ulid).await?;
    let players = room.get_players(pool.clone()).await?;
    let exists = players.iter().any(|p| p.game_user == Some(user.id));
    if exists {
        return Err(Error::code(StatusCode::CONFLICT));
    }
    let players: Vec<Player> = players
        .into_iter()
        .filter(|p| p.game_user.is_none())
        .collect();
    let rand = players
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect::<Vec<_>>();
    let player = rand
        .first()
        .ok_or_else(|| Error::code(StatusCode::CONFLICT))?;
    player.set_user(pool, user.id).await?;
    Ok(Json(room))
}

// pub async fn get_players(
//     ulid: String,
//     user: UserIdentification,
//     pool: Pool,
// ) -> Result<impl IntoResponse, Error> {
//     let room = Room::get(pool.clone(), &ulid).await?;
//     let players = room.get_players(pool).await?;
//     players
//         .iter()
//         .find(|p| p.game_user == Some(user.id))
//         .ok_or(Error::code(StatusCode::UNAUTHORIZED))?;
//     let public_players: Vec<PublicPlayer> = players.iter().map(|p| p.public()).collect();
//     Ok(warp::reply::json(&public_players))
// }

// pub async fn get_one(ulid: String, user: UserIdentification, pool: Pool) -> Result<impl IntoResponse, Error> {
//     let room = Room::get(pool.clone(), &ulid).await?;
//     user.is_part_of(pool, &room).await?;
//     Ok(warp::reply::json(&room))
// }
