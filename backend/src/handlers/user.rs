use crate::{database::game_user::UserIdentification, error::Error};
use axum::{debug_handler, extract::State, response::IntoResponse, Json};
use deadpool_postgres::Pool;

#[debug_handler]
pub async fn get(
    user: UserIdentification,
    State(pool): State<Pool>,
) -> Result<impl IntoResponse, Error> {
    let player = user.get_game_user(pool).await?.get_public();
    Ok(Json(player))
}
