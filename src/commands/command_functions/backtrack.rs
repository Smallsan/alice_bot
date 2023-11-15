use serenity::builder::CreateEmbed;
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::utils::Colour;

use crate::MessageStorage;

pub async fn get_backtracked_message(ctx: &Context, msg: &Message) -> CreateEmbed {
    let message_storage = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorage>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };

    let mut formatted_message = String::new();

    for messages in message_storage.lock().await.iter() {
        formatted_message += messages;
    }

    let mut embed = CreateEmbed::default();
        embed
        .colour(Colour::RED)
        .description(formatted_message)
        .timestamp(msg.timestamp);
        

    return embed

}