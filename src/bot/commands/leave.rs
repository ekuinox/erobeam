use super::prelude::*;

async fn handle_leave(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("manager")?;
    if manager.get(guild.id).is_none() {
        msg.channel_id.say(&ctx.http, "ボイスチャンネルにいないよ！").await?;
        return Ok(())
    }
    manager.remove(guild.id).await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = handle_leave(ctx, msg).await {
        log::error!("{e}");
    }
    Ok(())
}
