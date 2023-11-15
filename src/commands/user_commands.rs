use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use super::command_functions::bubble_wrap::generate_bubble_wrap;


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn bubble(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, generate_bubble_wrap()).await?;
    Ok(())
}
