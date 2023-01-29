use std::time::Duration;

use super::prelude::*;

async fn handle_seek(ctx: &Context, msg: &Message, mut args: Args) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;
    let duration = args.single::<u64>()?;
    let duration = Duration::from_secs(duration);

    let queue = {
        let handler = handler.lock().await;
        handler.queue().clone()
    };
    let Some(current) = queue.current() else {
        msg.channel_id.say(&ctx.http, "再生されていません").await?;
        return Ok(())
    };
    if !current.is_seekable() {
        msg.channel_id.say(&ctx.http, "再生中のトラックはシークに対応していません").await?;
        return Ok(())
    }
    current.seek_time(duration)?;
    log::info!("seek {duration:?}");

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn seek(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = handle_seek(ctx, msg, args).await {
        log::error!("{e}");
    }
    Ok(())
}
