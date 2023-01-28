use std::{fs::File, io::Read, path::Path};

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BotCredential {
    pub token: String,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ErobeamConfig {
    pub bot: BotCredential,
}

impl ErobeamConfig {
    pub fn open(path: impl AsRef<Path>) -> Result<ErobeamConfig> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let config = toml::from_str(&buf)?;
        Ok(config)
    }
}
