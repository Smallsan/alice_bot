use mini_redis::Error;
use serenity::client::Context;
use serenity::model::channel::{Message, PrivateChannel};
use serenity::model::id::UserId;
use serenity::builder::CreateEmbed;

use super::formatters::log_embed_formatter::log_embed_formatter;
use crate::ParsedConfig;

struct MessageStalkerConfig {
    user_id: UserId,
    receiver_id: UserId,
    enabled: bool,
}

pub async fn msg_stalker(ctx: &Context, msg: &Message) {
    if msg.author.bot || !msg.sticker_items.is_empty(){
        return;
    }

    let config: MessageStalkerConfig = fetch_config(&ctx).await;

    if msg.author.id != config.user_id || !config.enabled {
        return;
    }

    let embed_vec: Vec<CreateEmbed> = log_embed_formatter(&ctx, msg).await;

    if let Err(_) = send_private_message(&ctx, &config.receiver_id, embed_vec).await{
        eprintln!("Unable to create a message channel to the user. The user might have 'Direct message from server members' option disabled");
    }

}
async fn send_private_message(ctx: &Context, receiver_id: &UserId, embed_vec: Vec<CreateEmbed>) -> Result<(), Error> {
    let private_channel = fetch_private_channel(&ctx, &receiver_id).await?;
    private_channel.send_message(&ctx.http, |msg| msg.add_embeds(embed_vec)).await?;

    return Ok(());
}

async fn fetch_private_channel(ctx: &Context, receiver_id: &UserId) -> Result<PrivateChannel, Error> {
    let stalker_receiver = receiver_id.to_user(&ctx.http).await?;
    let stalker_private_channel = stalker_receiver.create_dm_channel(&ctx.http).await?;

    return Ok(stalker_private_channel);

}

async fn fetch_config(ctx: &Context) -> MessageStalkerConfig {
    let config = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ParsedConfig>()
            .expect("Expected Parsed Config In TypeMap")
            .clone()
    };

    {
        let config_locked = config.lock().await;

        return MessageStalkerConfig {
            user_id: UserId(config_locked.stalker_user_id),
            receiver_id: UserId(config_locked.stalker_receiver_id),
            enabled: config_locked.stalker_enabled,
        };
    }
}
