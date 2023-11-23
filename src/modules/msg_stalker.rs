use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::UserId;

use crate::ParsedConfig;

use super::formatters::log_embed_formatter::log_embed_formatter;

struct MessageStalkerConfig {
    msg_stalker_user_id: UserId,
    msg_stalker_receiver_id: UserId,
}

pub async fn msg_stalker(ctx: &Context, msg: &Message) {

    if msg.author.bot {
        return;
    }
    
    let msg_stalker_config : MessageStalkerConfig;

    let config_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
        .get::<ParsedConfig>()
        .expect("Expected Parsed Config In TypeMap")
        .clone()
    };

    {
        let locked_config_hashmap = config_hashmap.lock().await;
        msg_stalker_config = MessageStalkerConfig {
            msg_stalker_user_id: UserId(locked_config_hashmap.msg_stalker_user_id),
            msg_stalker_receiver_id: UserId(locked_config_hashmap.msg_stalker_receiver_id),
        }
    }

    if msg.author.id != msg_stalker_config.msg_stalker_user_id  {
        return;
    }

    let formatted_msg = log_embed_formatter(ctx, msg).await;
    msg_stalker_config.msg_stalker_receiver_id
    .to_user(&ctx.http).await.expect("Unable to fetch stalker_receiver_id")
    .create_dm_channel(&ctx.http).await.expect("Unable to direct message user")
    .send_message(&ctx.http, |msg| 
        msg.set_embed(formatted_msg)).await.expect("Unable to send direct message to user");


    
    



}
