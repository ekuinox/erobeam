mod commands;
mod handler;

use anyhow::Result;
use serenity::{framework::standard::StandardFramework, prelude::*};
use songbird::SerenityInit;

use self::{commands::*, handler::Handler};
use crate::config::ErobeamConfig;

pub struct ErobeamBot {
    #[allow(unused)]
    config: ErobeamConfig,
    client: Client,
}

impl ErobeamBot {
    pub async fn new(config: ErobeamConfig) -> Result<ErobeamBot> {
        let framework = StandardFramework::new()
            .configure(|c| c.prefix(">>"))
            .group(&GENERAL_GROUP);
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        let client = Client::builder(config.bot.token.clone(), intents)
            .framework(framework)
            .event_handler(Handler)
            .register_songbird()
            .await?;
        Ok(ErobeamBot { config, client })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.client.start().await?;
        Ok(())
    }
}
