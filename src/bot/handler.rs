use serenity::{async_trait, model::gateway::Ready, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, _ready: Ready) {
        println!("ready!");
    }
}
