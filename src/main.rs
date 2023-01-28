mod anyhow_ext;
mod bot;
mod chobit;
mod config;
mod scraper_ext;

use anyhow::Result;
use bot::ErobeamBot;
use config::ErobeamConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let path =
        std::env::var("EROBEAM_CONFIG_PATH").unwrap_or_else(|_| "/etc/erobeam.toml".to_string());
    let config = ErobeamConfig::open(path)?;

    let mut bot = ErobeamBot::new(config).await?;
    bot.run().await?;

    Ok(())
}
