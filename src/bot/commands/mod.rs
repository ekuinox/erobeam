mod join;
mod leave;
mod np;
mod pause;
mod ping;
mod play;
mod queue;
mod resume;
mod seek;
mod skip;
mod stop;

pub(self) mod prelude {
    pub use crate::{anyhow_ext::IntoAnyhowResult, bot::Config};
    pub use anyhow::Result;
    pub use serenity::{
        framework::standard::{macros::command, Args, CommandResult},
        model::prelude::*,
        prelude::*,
    };
}

use serenity::framework::standard::macros::group;

use self::{
    join::*, leave::*, np::*, pause::*, ping::*, play::*, queue::*, resume::*, seek::*, skip::*,
    stop::*,
};

#[group]
#[commands(join, leave, np, pause, ping, play, queue, resume, seek, skip, stop)]
pub struct General;
