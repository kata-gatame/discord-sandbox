pub mod owner {
    use crate::commands::owner::quit::*;
    use serenity::framework::standard::macros::group;

    #[group]
    #[commands(quit)]
    pub struct Owner;
}


pub mod misc {
    use crate::commands::misc::ping::*;
    use serenity::framework::standard::macros::group;

    #[group]
    #[commands(ping)]
    pub struct Misc;
}
