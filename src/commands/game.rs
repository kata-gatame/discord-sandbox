use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[command]
async fn guess(ctx: &Context, msg: &Message) -> CommandResult {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    msg.channel_id.say(&ctx.http, "A game of guess the number is starting!").await?;
    
    loop {
        msg.channel_id.say(&ctx.http, "Please input your guess...").await?;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => { msg.channel_id.say(&ctx.http, "Too small!").await?; },
            Ordering::Greater => { msg.channel_id.say(&ctx.http, "Too big!").await?; },
            Ordering::Equal => {
                msg.channel_id.say(&ctx.http, "You win!").await?;
                break;
            }
        }
    }

    Ok(())
}