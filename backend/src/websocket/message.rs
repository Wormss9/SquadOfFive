use axum::extract::ws::Message;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{utils::error::Error, game_logic::play::Card};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyMessage {
    #[serde(rename = "type")]
    kind: String,
    message: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum MessageType {
    Number(i32),
    Numbers(Vec<i32>),
    String(String),
    Vec(Vec<String>),
    Cards(Vec<Card>),
}

impl MyMessage {
    pub fn as_string(&self) -> String {
        to_string(self).unwrap()
    }
    pub fn as_message(&self) -> Message {
        Message::Text(self.as_string())
    }
    pub fn joined(id: i32) -> Self {
        Self {
            kind: "joined".to_owned(),
            message: MessageType::Number(id),
        }
    }
    pub fn disconnect(id: i32) -> Self {
        Self {
            kind: "joined".to_owned(),
            message: MessageType::Number(id),
        }
    }
    pub fn online(id: Vec<i32>) -> Self {
        Self {
            kind: "online".to_owned(),
            message: MessageType::Numbers(id),
        }
    }
    pub fn cards(id: Vec<Card>) -> Self {
        Self {
            kind: "cards".to_owned(),
            message: MessageType::Cards(id),
        }
    }
}

impl TryFrom<Message> for MyMessage {
    fn try_from(message: Message) -> Result<Self, Error> {
        serde_json::from_str(
            message
                .to_text()
                .map_err(Error::code_fn(StatusCode::IM_A_TEAPOT))?,
        )
        .map_err(Error::code_fn(StatusCode::IM_A_TEAPOT))
    }

    type Error = Error;
}
