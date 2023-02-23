use crate::{
    database::{game_user::UserIdentification, player::PublicPlayer, Player, Room},
    filters::rejection::MyRejection,
    game_logic::play::deal_cards,
};
use deadpool_postgres::Pool;
use http::StatusCode;
use rand::seq::SliceRandom;
use warp::{reply::Json, Rejection};

pub async fn create(user: UserIdentification, pool: Pool) -> Result<Json, Rejection> {
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
    Ok(warp::reply::json(&room))
}

pub async fn get_owned(user: UserIdentification, pool: Pool) -> Result<Json, Rejection> {
    let room = Room::get_my(pool, user.id).await?;
    Ok(warp::reply::json(&room))
}

pub async fn get_joined(user: UserIdentification, pool: Pool) -> Result<Json, Rejection> {
    let room = Room::get_joined(pool, user.id).await?;
    Ok(warp::reply::json(&room))
}

pub async fn get_players(
    ulid: String,
    user: UserIdentification,
    pool: Pool,
) -> Result<Json, Rejection> {
    let room = Room::get(pool.clone(), &ulid).await?;
    let players = room.get_players(pool).await?;
    players
        .iter()
        .find(|p| p.game_user == Some(user.id))
        .ok_or(MyRejection::code(StatusCode::UNAUTHORIZED))?;
    let public_players: Vec<PublicPlayer> = players.iter().map(|p| p.public()).collect();
    Ok(warp::reply::json(&public_players))
}

pub async fn get_one(
    ulid: String,
    user: UserIdentification,
    pool: Pool,
) -> Result<Json, Rejection> {
    let room = Room::get(pool.clone(), &ulid).await?;
    user.is_part_of(pool, &room).await?;
    Ok(warp::reply::json(&room))
}

pub async fn join(ulid: String, user: UserIdentification, pool: Pool) -> Result<Json, Rejection> {
    let room = Room::get(pool.clone(), &ulid).await?;
    let players = room.get_players(pool.clone()).await?;
    let exists = players
        .iter()
        .find(|p| p.game_user == Some(user.id))
        .is_some();
    if exists {
        return Err(MyRejection::code(StatusCode::CONFLICT));
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
        .ok_or(MyRejection::code(StatusCode::CONFLICT))?;
    player.set_user(pool, user.id).await?;
    Ok(warp::reply::json(&room))
}
