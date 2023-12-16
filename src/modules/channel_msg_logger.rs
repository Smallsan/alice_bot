use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;

use super::formatters::log_embed_formatter::log_embed_formatter;
use crate::ParsedConfig;

struct LogChannelConfig {
    channel_id: ChannelId,
    enabled: bool,
}

pub async fn channel_msg_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || !msg.sticker_items.is_empty(){
        return;
    }

    let config: LogChannelConfig = fetch_config(&ctx).await;

    if !config.enabled {
        return;
    }

    let embed_vec: Vec<serenity::builder::CreateEmbed> = log_embed_formatter(ctx, msg).await;

    match config
        .channel_id
        .send_message(&ctx.http, |message| message.add_embeds(embed_vec))
        .await
    {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending log message: {:?}", e);
        }
    };
}

async fn fetch_config(ctx: &Context) -> LogChannelConfig {
    let config_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<ParsedConfig>()
            .expect("Expected Parsed Config In TypeMap.")
            .clone()
    };

    let config_hashmap_locked = config_hashmap.lock().await;

    return LogChannelConfig {
        channel_id: ChannelId(config_hashmap_locked.log_channel_id),
        enabled: config_hashmap_locked.log_channel_enabled,
    };
}
