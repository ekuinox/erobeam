use super::prelude::*;

async fn handle_queue(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;

    let queue = {
        let handler = handler.lock().await;
        handler.queue().to_owned()
    };
    let tracks = queue.current_queue().into_iter().enumerate().map(|(i, track)| {
        let n = i + 1;
        if let Some(track) = &track.metadata().track {
            format!("{n} - {track}")
        } else {
            format!("{n} - UNKNOWN")
        }
    }).collect::<Vec<_>>().join("\n");
    let text = if tracks.is_empty() {
        "キューには何もありません".to_string()
    } else {
        tracks
    };

    msg.channel_id.say(&ctx.http, text).await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = handle_queue(ctx, msg).await {
        log::error!("{e}");
    }
    Ok(())
}
