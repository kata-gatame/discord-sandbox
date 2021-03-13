use rand::Rng;
use std::{time::Duration};
use serenity::{prelude::*};
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
async fn guess(ctx: &Context, msg: &Message) -> CommandResult {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    let _ =  msg.reply(ctx, "Please provide your guess...").await;

    if let Some(answer) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(10)).await {
        
        if answer.content.parse::<u32>().unwrap() == secret_number {
            let _ = answer.reply(ctx, "That's correct!").await;
        } else {
            if answer.content.parse::<u32>().unwrap() > secret_number {
                let _ = answer.reply(ctx, "Wrong, your guess was too big!").await;
            } else {
                let _ = answer.reply(ctx, "Wrong, your guess was too small!").await;
            }
        }
    } else {
        let _ =  msg.reply(ctx, "No answer within 10 seconds.").await;
    };

    Ok(())
}