use crate::{
    database::{
        commit, game_user::UserIdentification, initialize_client, initialize_transaction,
        player::PublicPlayer, Player, Room,
    },
    game_logic::play::deal_cards,
    utils::error::Error,
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
    let mut client = initialize_client(&pool).await?;
    let transaction = initialize_transaction(&mut client).await?;

    let room = Room::create(&transaction, user.id).await?;
    let deck = deal_cards();
    let mut players: Vec<Player> = Vec::new();
    for (x, cards) in deck.iter().enumerate() {
        players.push(Player::create(&transaction, &room.ulid, cards, x as i32).await?);
    }
    let mut player = players
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect::<Vec<_>>()[0]
        .to_owned();
    player.game_user = Some(user.id);
    player.update(&transaction).await?;
    commit(transaction).await?;
    Ok(Json(room))
}

pub async fn delete(
    Path(ulid): Path<String>,
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let room = Room::get(&pool, &ulid).await?;
    if room.host != user.id {
        return Err(Error::code(StatusCode::FORBIDDEN));
    }
    let mut client = initialize_client(&pool).await?;
    let transaction = initialize_transaction(&mut client).await?;
    room.delete(&transaction).await?;
    commit(transaction).await?;
    Ok(())
}

pub async fn get_owned(
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let rooms = Room::get_owned(&pool, user.id).await?;
    Ok(Json(rooms))
}

pub async fn get_joined(
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let rooms = Room::get_joined(&pool, user.id).await?;
    Ok(Json(rooms))
}

pub async fn join(
    Path(ulid): Path<String>,
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let room = Room::get(&pool, &ulid).await?;
    let players = room.get_players(&pool).await?;
    let exists = players.iter().any(|p| p.game_user == Some(user.id));
    if exists {
        return Ok(Json(room));
    }
    let players: Vec<Player> = players
        .into_iter()
        .filter(|p| p.game_user.is_none())
        .collect();
    let rand = players
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect::<Vec<_>>();
    let mut player = rand
        .first()
        .ok_or_else(|| Error::code(StatusCode::CONFLICT))?
        .to_owned()
        .to_owned();
    player.game_user = Some(user.id);
    let mut client = initialize_client(&pool).await?;
    let transaction = initialize_transaction(&mut client).await?;
    player.update(&transaction).await?;
    commit(transaction).await?;
    Ok(Json(room))
}

pub async fn get_players(
    Path(ulid): Path<String>,
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let room = Room::get(&pool, &ulid).await?;
    let players = room.get_players(&pool).await?;
    players
        .iter()
        .find(|p| p.game_user == Some(user.id))
        .ok_or_else(|| Error::code(StatusCode::UNAUTHORIZED))?;
    let public_players: Vec<PublicPlayer> = players.iter().map(|p| p.public()).collect();
    Ok(Json(public_players))
}
