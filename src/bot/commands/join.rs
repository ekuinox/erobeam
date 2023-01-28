use anyhow::Result;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::anyhow_ext::IntoAnyhowResult;

async fn handle_join(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).into_anyhow_result("guild")?;
    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id)
        .into_anyhow_result("channel_id")?;
    let manager = songbird::get(ctx).await.into_anyhow_result("songbird")?;

    let (_, join_result) = manager.join(guild.id, channel_id).await;
    join_result?;

    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = handle_join(ctx, msg).await {
        eprintln!("{e}");
    }

    Ok(())
}
