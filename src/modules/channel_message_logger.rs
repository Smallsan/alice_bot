use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;

use super::formatters::log_embed_formatter::log_embed_formatter;

pub async fn channel_message_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot{
        return;
    }

        let log_channel_id: ChannelId = ChannelId(967685973456609320);

        let embed: CreateEmbed = log_embed_formatter(ctx, msg).await;

        let _send_message = log_channel_id
                .send_message(&ctx.http, |message| 
                    message.set_embed(embed))
                .await;
            }





