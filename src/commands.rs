use crate::{BotCommand, TelegramBot};
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct BotCommandScope {}

impl BotCommand {
    pub fn new(command: String, description: String) -> Self {
        BotCommand {
            command,
            description,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct InternalBotCommand{
    commands: Vec<BotCommand>
}

impl TelegramBot {
    pub async fn set_commands(&self, commands: Vec<BotCommand>) -> Result<Response, Error> {
        let com = InternalBotCommand{
            commands
        };
        let _ = &self.delete_all_commands().await?;
        let client = Client::new();
        return client
            .post(format!("{}{}", self.telegram_bot_api_url, "setMyCommands"))
            .json(&com)
            .send()
            .await;
    }

    async fn delete_all_commands(&self) -> Result<Response, Error> {
        let client = Client::new();
        return client
            .post(format!(
                "{}{}",
                self.telegram_bot_api_url, "deleteMyCommands"
            ))
            .send()
            .await;
    }
}
