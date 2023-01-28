mod join;
mod leave;
mod ping;
mod play;
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

use self::{join::*, leave::*, ping::*, play::*, stop::*};

#[group]
#[commands(join, leave, ping, play, stop)]
pub struct General;
