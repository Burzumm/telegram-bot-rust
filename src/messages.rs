use crate::{Message, TelegramBot};
use reqwest::{Error, Response};
use tracing::info;
use tracing_attributes::instrument;

impl Message {
    pub fn new(chat_id: i64, text: String) -> Self {
        Message { chat_id, text }
    }
}

impl TelegramBot {
    #[instrument]
    pub async fn send_message(&self, message: &Message) -> Result<Response, Error> {
        info!(
            "send telegram message started to chat_id: {}",
            message.chat_id
        );
        return reqwest::Client::new()
            .post(format!("{}{}", self.telegram_bot_api_url, "sendMessage"))
            .json(message)
            .send()
            .await;
    }
}
