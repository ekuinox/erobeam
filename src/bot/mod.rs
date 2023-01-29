mod commands;
mod handler;

use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, Result};
use reqwest::Url;
use serenity::{framework::standard::StandardFramework, prelude::*};
use songbird::SerenityInit;
use uuid::Uuid;

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
            data.insert::<TrackDetailsKey>(TrackDetails::new());
        }

        Ok(ErobeamBot { client })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.client.start().await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TrackDetail {
    #[allow(unused)]
    pub url: Url,
    #[allow(unused)]
    pub author: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct TrackDetails(Arc<Mutex<HashMap<Uuid, Arc<TrackDetail>>>>);

pub struct TrackDetailsKey;

impl TypeMapKey for TrackDetailsKey {
    type Value = TrackDetails;
}

impl TrackDetails {
    pub fn new() -> TrackDetails {
        TrackDetails(Arc::new(Mutex::new(HashMap::new())))
    }

    pub async fn insert(&self, uuid: Uuid, track: TrackDetail) -> Result<()> {
        let mut inner = self.0.lock().await;
        let _ = inner.insert(uuid, Arc::new(track));

        Ok(())
    }

    pub async fn get(&self, uuid: &Uuid) -> Result<Arc<TrackDetail>> {
        let inner = self.0.lock().await;
        let Some(value) = inner.get(uuid) else {
            bail!("uuid not found");
        };
        Ok(value.clone())
    }
}
