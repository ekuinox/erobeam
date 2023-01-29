use std::sync::Arc;

use uuid::Uuid;

use super::prelude::*;

async fn handle_queue(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;

    let queue = {
        let handler = handler.lock().await;
        handler.queue().to_owned()
    };
    let tracks = {
        async fn get(ctx: Context, uuid: Uuid) -> Result<Arc<TrackDetail>> {
            let details = ctx.data.read().await;
            let details = details
                .get::<TrackDetailsKey>()
                .into_anyhow_result("details")?;
            details.get(&uuid).await
        }

        let tracks = futures::future::join_all(
            queue
                .current_queue()
                .into_iter()
                .map(|track| get(ctx.clone(), track.uuid())),
        )
        .await;
        tracks
    };
    let tracks = tracks
        .into_iter()
        .enumerate()
        .map(|(i, track)| {
            if let Ok(track) = track {
                format!("{i} - {}\n", track.title)
            } else {
                format!("{i} - UNKNOWN\n")
            }
        })
        .collect::<String>();
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
