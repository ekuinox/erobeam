mod join;
mod leave;
mod ping;

pub(self) mod prelude {
    pub use crate::anyhow_ext::IntoAnyhowResult;
    pub use anyhow::Result;
    pub use serenity::{
        framework::standard::{macros::command, CommandResult},
        model::prelude::*,
        prelude::*,
    };
}

use serenity::framework::standard::macros::group;

use self::{join::*, leave::*, ping::*};

#[group]
#[commands(join, leave, ping)]
pub struct General;
