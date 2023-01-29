use super::prelude::*;

async fn handle_skip(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    let handler = manager.get(guild.id).into_anyhow_result("voice channel")?;

    let handler = handler.lock().await;
    handler.queue().skip()?;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = handle_skip(ctx, msg).await {
        log::error!("{e}");
    }
    Ok(())
}
