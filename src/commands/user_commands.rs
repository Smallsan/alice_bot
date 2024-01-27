use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::modules::handlers::models::reputation::{self, Model as Reputation};

use crate::commands::command_functions::backtrack::get_backtrack;
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
    let backtrack_msg_embed = get_backtrack(&ctx, &msg).await;

    msg.channel_id
        .send_message(&ctx.http, |msg| msg.set_embed(backtrack_msg_embed))
        .await?;

    Ok(())
}

#[command]
async fn rep(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = args.single::<UserId>()?;
    let user_name = user_id.to_user(&ctx.http).await?.name;
    let reputation_model = Reputation::find_user_by_id(user_id.as_u64().clone() as i64).await;

    match reputation_model {
        Some(reputation_model) => {
            msg.channel_id
                .say(&ctx.http, format!("User {} has a reputation score of {}! Keep up the good work!", user_name, reputation_model.reputation))
                .await?;
        }
        None => {
            msg.channel_id
                .say(&ctx.http, format!("User {} is new around here and doesn't have a reputation score yet.", user_name))
                .await?;
        }
    }
    Ok(())
}

#[command]
async fn halal(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = args.single::<UserId>()?;
    let reputation_model = Reputation::find_user_by_id(user_id.as_u64().clone() as i64).await;
    let user_name = user_id.to_user(&ctx.http).await?.name;
    match reputation_model {
        Some(_) => {
            Reputation::increment_reputation(user_id.as_u64().clone() as i64).await?;
            let updated_reputation_model = Reputation::find_user_by_id(user_id.as_u64().clone() as i64).await.unwrap();
            msg.channel_id
                .say(&ctx.http, format!("User {}'s reputation has increased! Their new reputation score is {}.", user_name, updated_reputation_model.reputation))
                .await?;
        }
        None => {
            let mut reputation_model = Reputation::new(user_id.as_u64().clone() as i64, 0, user_name.clone());
            reputation_model.reputation += 1;
            msg.channel_id
                .say(&ctx.http, format!("User {} has earned their first reputation point! Their reputation score is now {}.", user_name, reputation_model.reputation))
                .await?;
            Reputation::create(reputation_model).await?;
        }
    }
    Ok(())
}

#[command]
async fn haram(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = args.single::<UserId>()?;
    let reputation_model = Reputation::find_user_by_id(user_id.as_u64().clone() as i64).await;
    let user_name = user_id.to_user(&ctx.http).await?.name;
    match reputation_model {
        Some(_) => {
            Reputation::decrement_reputation(user_id.as_u64().clone() as i64).await?;
            let updated_reputation_model = Reputation::find_user_by_id(user_id.as_u64().clone() as i64).await.unwrap();
            msg.channel_id
                .say(&ctx.http, format!("User {}'s reputation has decreased. Their new reputation score is {}.", user_name, updated_reputation_model.reputation))
                .await?;
        }
        None => {
            let mut reputation_model = Reputation::new(user_id.as_u64().clone() as i64, 0, user_name.clone());
            reputation_model.reputation -= 1;
            msg.channel_id
                .say(&ctx.http, format!("User {}'s reputation has decreased. Their reputation score is now {}.", user_name, reputation_model.reputation))
                .await?;
            Reputation::create(reputation_model).await?;
        }
    }
    Ok(())
}