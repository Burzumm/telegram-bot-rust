use crate::{Message, TelegramBot, TelegramMessage, TelegramResponseResult, UpdatedMessage};
use reqwest::Error;
use tracing::info;
use tracing_attributes::instrument;

impl Message {
    pub fn new(chat_id: i64, text: String) -> Self {
        Message { chat_id, text }
    }
}

impl UpdatedMessage {
    pub fn new(chat_id: i64, text: String, message_id: i64) -> Self {
        UpdatedMessage {
            chat_id,
            text,
            message_id,
        }
    }

    pub fn from(message: &TelegramMessage, new_text: String) -> Self {
        UpdatedMessage::new(message.chat.id, new_text, message.message_id)
    }
}

impl TelegramBot {
    #[instrument]
    pub async fn send_message(
        &self,
        message: &Message,
    ) -> Result<TelegramResponseResult<TelegramMessage>, Error> {
        info!(
            "send telegram message started to chat_id: {}",
            message.chat_id
        );
        return reqwest::Client::new()
            .post(format!("{}{}", self.telegram_bot_api_url, "sendMessage"))
            .json(message)
            .send()
            .await?
            .json::<TelegramResponseResult<TelegramMessage>>()
            .await;
    }

    #[instrument]
    pub async fn edit_message_text(
        &self,
        message: &UpdatedMessage,
    ) -> Result<TelegramResponseResult<TelegramMessage>, Error> {
        info!(
            "update telegram message started to chat_id: {}",
            message.chat_id
        );
        return reqwest::Client::new()
            .post(format!(
                "{}{}",
                self.telegram_bot_api_url, "editMessageText"
            ))
            .json(message)
            .send()
            .await?
            .json::<TelegramResponseResult<TelegramMessage>>()
            .await;
    }
}
