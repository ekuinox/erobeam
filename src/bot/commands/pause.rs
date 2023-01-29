use super::prelude::*;

async fn handle_pause(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;

    let handler = handler.lock().await;
    handler.queue().pause()?;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = handle_pause(ctx, msg).await {
        log::error!("{e}");
    }
    Ok(())
}
