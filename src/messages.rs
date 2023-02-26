use crate::{Message, TelegramBot};
use reqwest::{Error, Response};

impl Message {
    pub fn new(chat_id: i64, text: String) -> Self {
        Message { chat_id, text }
    }
}

impl TelegramBot {
    pub async fn send_message(&self, message: Message) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        return client
            .post(format!("{}{}", self.telegram_bot_api_url, "sendMessage"))
            .json(&message)
            .send()
            .await;
    }
}
