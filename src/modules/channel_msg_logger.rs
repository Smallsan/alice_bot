use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;

use crate::ParsedConfig;

use super::formatters::log_embed_formatter::log_embed_formatter;

struct LogChannelConfig{
    channel_id: ChannelId,
    enabled: bool,
}

pub async fn channel_msg_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }
    let config_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ParsedConfig>()
            .expect("Expected Parsed Config In TypeMap.")
            .clone()
    };

    let log_channel_config : LogChannelConfig;
    {
        let config_hashmap_locked = config_hashmap.lock().await;
        log_channel_config = LogChannelConfig{
            channel_id: ChannelId(config_hashmap_locked.log_channel_id),
            enabled: config_hashmap_locked.log_channel_enabled,
        }
    }
    
    if !log_channel_config.enabled {
        return;
    }

    let embed_vec = log_embed_formatter(ctx, msg).await;

    let _send_message = log_channel_config.channel_id
        .send_message(&ctx.http, |message| message.add_embeds(embed_vec))
        .await;
}
