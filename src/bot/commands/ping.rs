use super::prelude::*;

#[command]
#[only_in(guilds)]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = msg.channel_id.say(&ctx.http, "pong!").await {
        log::error!("{e}");
    }
    Ok(())
}
