use crate::{
    database::{game_user::UserIdentification, GameUser},
    error::Error,
};
use axum::{
    debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use deadpool_postgres::Pool;

#[debug_handler]
pub async fn get_self(
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let player = user.get_game_user(pool).await?.get_public();
    Ok(Json(player))
}

#[debug_handler]
pub async fn get(
    Path(id): Path<i32>,
    _: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let player = GameUser::get_by_id(pool, id).await?.get_public();
    Ok(Json(player))
}
