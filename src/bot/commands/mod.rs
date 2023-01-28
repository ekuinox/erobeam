mod join;

use serenity::framework::standard::macros::group;

use self::join::*;

#[group]
#[commands(join)]
pub struct General;
