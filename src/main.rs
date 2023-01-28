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

    env_logger::init();

    let mut bot = ErobeamBot::new(config).await?;

    tokio::select! {
        r = bot.run() => {
            if let Err(e) = r {
                log::error!("{e}");
            }
        },
        r = tokio::signal::ctrl_c() => {
            if let Err(e) = r {
                log::error!("{e}");
            }
            println!("shutdown");
        }
    }

    Ok(())
}
