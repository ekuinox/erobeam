use super::prelude::*;

async fn handle_np(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;

    let handler = handler.lock().await;
    let text = match handler.queue().current() {
        Some(current) => {
            let track = if let Some(track) = &current.metadata().track {
                track.clone()
            } else {
                "UNKNOWN".to_string()
            };
            let duration = if let Some(duration) = &current.metadata().duration {
                let m = duration.as_secs() / 60;
                let s = duration.as_secs() % 60;
                format!("{m:02}:{s:02}")
            } else {
                String::new()
            };
            format!("再生中 - {track} {duration} {{uuid={}}}", current.uuid())
        }
        None => "何も再生していないっぽいです".to_string(),
    };
    msg.channel_id.say(&ctx.http, text).await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn np(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = handle_np(ctx, msg).await {
        log::error!("{e}");
    }
    Ok(())
}
