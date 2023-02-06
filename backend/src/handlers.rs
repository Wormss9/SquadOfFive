use crate::{authorization::{Login, self}, database::Player};
use deadpool_postgres::Pool;
use http::StatusCode;
use warp::{reply::WithStatus, Rejection};

pub async fn register(login: Login, pool: Pool) -> Result<WithStatus<String>, Rejection> {
    Player::create(pool.clone(), &login.name, &login.password).await?;
    Ok(warp::reply::with_status(
        "CREATED".to_owned(),
        StatusCode::OK,
    ))
    // authorization::login(login,pool).await
}
