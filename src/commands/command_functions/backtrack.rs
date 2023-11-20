use serenity::builder::CreateEmbed;
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::utils::Colour;

use crate::MessageStorageContainer;


pub async fn get_backtracked_message(ctx: &Context, msg: &Message) -> CreateEmbed {

    let channel_id = msg.channel_id.as_u64();

    let message_storage_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorageContainer>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };
    
    let mut formatted_message = String::new();

    let message_storage_hashmap_locked = message_storage_hashmap.lock().await;

    match message_storage_hashmap_locked.get(channel_id) { 
        Some(message_storage_vector) =>
        for message in message_storage_vector.iter() {

            formatted_message += message;

        },
        None => formatted_message += "Cannot Retrieve Old Messages",
    }

    let mut embed = CreateEmbed::default();
        embed
        .colour(Colour::RED)
        .description(formatted_message)
        .timestamp(msg.timestamp);
        

    return embed


}

