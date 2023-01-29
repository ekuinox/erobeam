use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

use reqwest::Url;
use songbird::Call;
use tokio::io::{AsyncWriteExt, BufWriter};

use super::prelude::*;
use crate::chobit;

async fn handle_play(ctx: &Context, msg: &Message, mut args: Args) -> Result<()> {
    let Ok(url) = args.single::<String>() else {
        msg.channel_id.say(&ctx.http, "URL指定してね～").await?;
        return Ok(());
    };
    let url = Url::parse(&url)?;

    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;

    let cache_dir = {
        let data = ctx.data.read().await;
        data.get::<Config>()
            .into_anyhow_result("config")
            .map(|c| &c.voice.cache_dir)?
            .clone()
    };

    play_chobit_product(&cache_dir, url, handler).await?;

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

async fn play_chobit_voice(
    cache_dir: &Path,
    chobit::VoiceSampleTrack {
        media_url,
        title,
        playtime,
    }: chobit::VoiceSampleTrack,
    handler: Arc<Mutex<Call>>,
) -> Result<VoiceSampleTrack> {
    let filename = media_url
        .path()
        .split('/')
        .last()
        .into_anyhow_result("name")?;
    let filename = cache_dir.join(filename);
    if filename.exists() {
        log::info!("Cache exists ... {filename:?}");
    } else {
        log::info!("Download ... {filename:?}");
        let bytes = reqwest::get(media_url.clone()).await?.bytes().await?;
        let file = File::create(&filename)?;
        let mut writer = BufWriter::new(tokio::fs::File::from_std(file));
        writer.write_all(&bytes).await?;
    }

    let source = songbird::ffmpeg(&filename).await?;
    let mut handler = handler.lock().await;
    let track = handler.enqueue_source(source);
    log::info!("queued ... {:?}", track.metadata().track);

    Ok(VoiceSampleTrack {
        media_url,
        title,
        playtime,
        path: filename,
    })
}

async fn play_chobit_product(
    cache_dir: &Path,
    url: Url,
    handler: Arc<Mutex<Call>>,
) -> Result<VoiceProduct> {
    let chobit::VoiceProduct {
        voice_sample_tracks,
        product_id,
        title,
        circle_name,
        cover_url,
    } = chobit::VoiceProduct::from_url(url).await?;
    let voice_sample_tracks = futures::future::join_all(
        voice_sample_tracks
            .into_iter()
            .map(|track| play_chobit_voice(cache_dir, track, handler.clone())),
    )
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    Ok(VoiceProduct {
        voice_sample_tracks,
        product_id,
        title,
        circle_name,
        cover_url,
    })
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VoiceSampleTrack {
    pub media_url: Url,
    pub title: String,
    pub playtime: String, // Todo to Duration
    pub path: PathBuf,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VoiceProduct {
    pub product_id: String,
    pub title: String,
    pub circle_name: String,
    pub cover_url: Url,
    pub voice_sample_tracks: Vec<VoiceSampleTrack>,
}
