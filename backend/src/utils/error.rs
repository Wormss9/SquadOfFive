use axum::response::IntoResponse;
use http::StatusCode;
use tokio_postgres::error::SqlState;

#[derive(Debug)]
pub struct Error {
    code: StatusCode,
    message: Option<String>,
}

impl Error {
    pub fn code(code: StatusCode) -> Self {
        Self {
            code,
            message: None,
        }
    }
    pub fn message(code: StatusCode, message: String) -> Self {
        Self {
            code,
            message: Some(message),
        }
    }
    pub fn code_fn<T: std::fmt::Debug>(code: StatusCode) -> impl Fn(T) -> Self {
        move |_| -> Self { Self::code(code) }
    }
    pub fn message_fn<T: std::fmt::Debug>(code: StatusCode, message: String) -> impl Fn(T) -> Self {
        move |_| -> Self { Self::message(code, message.clone()) }
    }
    pub fn from_db(e: tokio_postgres::Error) -> Self {
        let code = StatusCode::INTERNAL_SERVER_ERROR;
        let state = match e.code() {
            Some(s) => s,
            None => return Self::code(code),
        };
        let message = match state {
            &SqlState::UNIQUE_VIOLATION => Some("Already exists".to_owned()),
            _ => None,
        };
        Self { code, message }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            match self.message {
                Some(x) => x,
                None => self.code.canonical_reason().unwrap_or("").to_owned(),
            },
        )
            .into_response()
    }
}
