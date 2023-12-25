use mini_redis::Error;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;

use crate::modules::formatters::log_embed_formatter::log_embed_formatter;
use crate::ParsedConfig;

struct LogChannelConfig {
    channel_id: ChannelId,
    enabled: bool,
}

pub async fn channel_msg_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || !msg.sticker_items.is_empty() {
        return;
    }

    let config: LogChannelConfig = fetch_config(&ctx).await;

    if !config.enabled {
        return;
    }

    let embed_vec: Vec<serenity::builder::CreateEmbed> = log_embed_formatter(ctx, msg).await;

    if let Err(err) = send_message(&ctx, config.channel_id, embed_vec).await {
        eprintln!("Error sending log message: {:?}", err);
    }
}

async fn send_message(
    ctx: &Context,
    channel_id: ChannelId,
    embed_vec: Vec<CreateEmbed>,
) -> Result<(), Error> {
    channel_id
        .send_message(&ctx.http, |message| message.add_embeds(embed_vec))
        .await?;
    return Ok(());
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
