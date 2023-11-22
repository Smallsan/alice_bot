use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::command_functions::backtrack::get_backtracked_message;

use crate::commands::command_functions::bubble_wrap::generate_bubble_wrap;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn bubble(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, generate_bubble_wrap())
        .await?;

    Ok(())
}

#[command]
async fn backtrack(ctx: &Context, msg: &Message) -> CommandResult {
    let backtrack_message_embed = get_backtracked_message(&ctx, &msg).await;
    msg.channel_id
        .send_message(&ctx.http, |message| {
            message.set_embed(backtrack_message_embed)
        })
        .await?;

    Ok(())
}
