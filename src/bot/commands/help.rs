use super::prelude::*;

const HELP_MESSAGE: &str = concat!(
    "commands: help, join, leave, np, pause, ping, play, queue, resume, seek, skip, stop\n",
    std::env!("CARGO_PKG_NAME"),
    " ",
    std::env!("CARGO_PKG_REPOSITORY"),
    "\n"
);

#[command]
#[only_in(guilds)]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
        log::error!("{e}");
    }
    Ok(())
}
