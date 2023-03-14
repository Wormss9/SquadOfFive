use crate::{
    database::game_user::UserIdentification,
    utils::{
        authorization::{authorize_token, Key},
        error::Error,
    },
};
use axum::{
    async_trait,
    extract::{Extension, FromRequestParts, Query, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[async_trait]
impl<S> FromRequestParts<S> for UserIdentification
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(token)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| err.into_response())?;

        use axum::RequestPartsExt;
        let Extension(key) = parts
            .extract::<Extension<Key>>()
            .await
            .map_err(|err| err.into_response())?;

        authorize_token(&key, token.token().to_owned())
            .map_err(|_| Error::code(StatusCode::UNAUTHORIZED).into_response())
    }
}

pub struct WsUserIdentification(UserIdentification);

impl WsUserIdentification {
    pub fn inner(self) -> UserIdentification {
        self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for WsUserIdentification
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(token) = Query::<Token>::from_request_parts(parts, state)
            .await
            .map_err(|err| err.into_response())?;

        use axum::RequestPartsExt;
        let Extension(key) = parts
            .extract::<Extension<Key>>()
            .await
            .map_err(|err| err.into_response())?;

        Ok(WsUserIdentification(
            authorize_token(&key, token.token)
                .map_err(|_| Error::code(StatusCode::UNAUTHORIZED).into_response())?,
        ))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Token {
    token: String,
}
