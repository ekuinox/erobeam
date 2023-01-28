use std::{fs::File, path::Path};

use tokio::io::{AsyncWriteExt, BufWriter};

use super::prelude::*;
use crate::chobit;

async fn handle_play(ctx: &Context, msg: &Message, mut args: Args) -> Result<()> {
    let Ok(url) = args.single::<String>() else {
        msg.channel_id.say(&ctx.http, "URL指定してね～").await?;
        return Ok(());
    };

    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;

    let product = chobit::VoiceProduct::from_url(url).await?;
    let sample = product
        .voice_sample_tracks
        .last()
        .into_anyhow_result("last voice sample track")?;

    let filename = sample
        .media_url
        .path()
        .split('/')
        .last()
        .map(|s| Path::new("./voices/").join(s))
        .into_anyhow_result("name")?;
    log::debug!("{filename:?}");
    if !filename.exists() {
        let bytes = reqwest::get(sample.media_url.clone())
            .await?
            .bytes()
            .await?;
        let file = File::create(&filename)?;
        let mut writer = BufWriter::new(tokio::fs::File::from_std(file));
        writer.write_all(&bytes).await?;
    }

    let source = songbird::ffmpeg(&filename).await?;
    let mut handler = handler.lock().await;
    handler.play_source(source);

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = handle_play(ctx, msg, args).await {
        log::error!("{e}");
    }
    Ok(())
}
