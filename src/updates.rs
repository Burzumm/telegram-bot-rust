use std::io::stderr;
use crate::{TelegramBot, TelegramResponseResult, TelegramUpdate};
use reqwest::{Client, Error, Response};

use std::time::Duration;
use tracing::info;
use tracing_attributes::instrument;

impl TelegramBot {
    #[instrument]
    pub async fn get_updates(
        &mut self,
        update_timeout_secs: u64,
        last_update_id: Option<i64>,
    ) -> Result<Vec<TelegramUpdate>, Error> {
        match last_update_id {
            None => {}
            Some(update_id) => self.last_update_id = update_id,
        }
        info!("current telegram update id: {}", self.last_update_id);
        let client = Client::new();
        let res = self
            .get_updates_internal(&client, update_timeout_secs)
            .await?;
        let update_result = res
            .json::<TelegramResponseResult<Vec<TelegramUpdate>>>()
            .await?;
        match update_result.result {
            None => panic!("result empty"),
            Some(result) => return Ok(result)

        }
    }

    #[instrument]
    async fn get_updates_internal(
        &mut self,
        client: &Client,
        update_timeout_secs: u64,
    ) -> Result<Response, Error> {
        return client
            .get(format!(
                "{}{}{}{}{}{}",
                self.telegram_bot_api_url,
                "getUpdates?",
                "offset=",
                &self.last_update_id + 1,
                "&timeout=",
                update_timeout_secs
            ))
            .timeout(Duration::from_secs(update_timeout_secs + 10))
            .send()
            .await;
    }

    // async fn update_callback<TFunc, TTelegramUpdate>(
    //     &mut self,
    //     callback: fn(&TFunc, Vec<TelegramUpdate>),
    //     update_timeout_secs: u64,
    //     func_param: TFunc,
    // ) {
    //     let client = Client::new();
    //     let res = self
    //         .get_updates_internal(&client, update_timeout_secs)
    //         .await;
    //     let update_result = res
    //         .unwrap()
    //         .json::<TelegramResponseResult<Vec<TelegramUpdate>>>()
    //         .await
    //         .unwrap();
    //     if !update_result.result.is_empty() {
    //         for i in &update_result.result {
    //             if i.update_id > self.last_update_id {
    //                 self.last_update_id = i.update_id;
    //             }
    //         }
    //         callback(&func_param, update_result.result);
    //     }
    // }
}
