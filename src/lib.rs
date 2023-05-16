mod bot;
mod commands;
mod messages;
mod updates;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub text: Option<String>,
    pub date: i64,
    pub chat: TelegramChat,
    pub entities: Option<Vec<TelegramEntities>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct TelegramChat {
    pub id: i64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub chat_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramEntities {
    pub offset: i64,
    pub length: i64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub message_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: TelegramMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatedMessage {
    pub message_id: i64,
    pub chat_id: i64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub chat_id: i64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramResponseResult<T> {
    pub ok: bool,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramBot {
    api_token: String,
    telegram_api_url: String,
    telegram_bot_api_url: String,
    last_update_id: i64,
}
