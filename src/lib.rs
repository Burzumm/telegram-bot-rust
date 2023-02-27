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
    pub entities: Vec<TelegramEntities>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramEntities{
    offset: i64,
    length: i64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    message_type: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: TelegramMessage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub chat_id: i64,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotCommand {
    command: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TelegramResponseResult<T> {
    ok: bool,
    result: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramBot {
    api_token: String,
    telegram_api_url: String,
    telegram_bot_api_url: String,
    last_update_id: i64,
}
