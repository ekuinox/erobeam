mod join;
mod leave;
mod ping;
mod play;

pub(self) mod prelude {
    pub use crate::anyhow_ext::IntoAnyhowResult;
    pub use anyhow::Result;
    pub use serenity::{
        framework::standard::{macros::command, CommandResult, Args},
        model::prelude::*,
        prelude::*,
    };
}

use serenity::framework::standard::macros::group;

use self::{join::*, leave::*, ping::*, play::*};

#[group]
#[commands(join, leave, ping, play)]
pub struct General;
