use crate::{BotCommand, TelegramBot};
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_attributes::instrument;

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
struct InternalBotCommand {
    commands: Vec<BotCommand>,
}

impl TelegramBot {
    #[instrument]
    pub async fn set_commands(&self, commands: &Vec<BotCommand>) -> Result<Response, Error> {
        let com = InternalBotCommand {
            commands: commands.to_vec(),
        };
        let delete_response = &self.delete_all_commands().await?.text().await?;
        info!("delete telegram commands result: {}", delete_response);
        let client = Client::new();
        info!("set telegram commands commands started");
        return client
            .post(format!("{}{}", self.telegram_bot_api_url, "setMyCommands"))
            .json(&com)
            .send()
            .await;
    }

    #[instrument]
    async fn delete_all_commands(&self) -> Result<Response, Error> {
        info!("start delete all telegram commands");
        let client = Client::new();
        let response = client
            .post(format!(
                "{}{}",
                self.telegram_bot_api_url, "deleteMyCommands"
            ))
            .send()
            .await;
        return response;
    }
}
