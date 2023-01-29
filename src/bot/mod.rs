mod commands;
mod handler;

use std::sync::Arc;

use anyhow::Result;
use serenity::{framework::standard::StandardFramework, prelude::*};
use songbird::SerenityInit;

use self::{commands::*, handler::Handler};
use crate::config::ErobeamConfig;

pub struct Config;

impl TypeMapKey for Config {
    type Value = Arc<ErobeamConfig>;
}

pub struct ErobeamBot {
    client: Client,
}

impl ErobeamBot {
    pub async fn new(config: ErobeamConfig) -> Result<ErobeamBot> {
        tokio::fs::create_dir_all(&config.voice.cache_dir).await?;

        let framework = StandardFramework::new()
            .configure(|c| c.prefix(&config.bot.prefix))
            .group(&GENERAL_GROUP);
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        let client = Client::builder(config.bot.token.clone(), intents)
            .framework(framework)
            .event_handler(Handler)
            .register_songbird()
            .await?;
        {
            let mut data = client.data.write().await;
            data.insert::<Config>(Arc::new(config));
        }

        Ok(ErobeamBot { client })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.client.start().await?;
        Ok(())
    }
}
