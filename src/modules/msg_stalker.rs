use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::UserId;

use crate::ParsedConfig;

use super::formatters::log_embed_formatter::log_embed_formatter;

struct MessageStalkerConfig {
    user_id: UserId,
    receiver_id: UserId,
    enabled: bool,
}

pub async fn msg_stalker(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }

    let stalker_config: MessageStalkerConfig;

    let config = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ParsedConfig>()
            .expect("Expected Parsed Config In TypeMap")
            .clone()
    };

    {
        let config_locked = config.lock().await;
        stalker_config = MessageStalkerConfig {
            user_id: UserId(config_locked.stalker_user_id),
            receiver_id: UserId(config_locked.stalker_receiver_id),
            enabled: config_locked.stalker_enabled,
        }
    }

    if msg.author.id != stalker_config.user_id || !stalker_config.enabled {
        return;
    }

    let embed_vec = log_embed_formatter(&ctx, msg).await;
    let stalker_receiver = &stalker_config
        .receiver_id
        .to_user(&ctx.http)
        .await
        .expect("Unable to get fetch from stalker user id");
    let stalker_private_channel_result = stalker_receiver.create_dm_channel(&ctx.http).await;
    match stalker_private_channel_result {
        Ok(_) => {
            if let Ok(stalker_private_channel) = stalker_private_channel_result {
                stalker_private_channel
                    .send_message(&ctx.http, |msg| msg.add_embeds(embed_vec))
                    .await
                    .expect("Unable to send direct message to user");
            }
        }
        Err(_) => {
            println!("Unable to create message channel to user, User might have their 'Direct message from server members' option disabled");
            return;
        }
    }
}
