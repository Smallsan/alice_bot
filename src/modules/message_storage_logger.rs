use serenity::model::channel::Message;
use serenity::client::Context;
use crate::MessageStorage;

pub async fn message_storage_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || msg.content.contains("!backtrack"){
        return;
    }

    let channel_id = msg.channel_id.as_u64();

    let message_storage_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorage>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };

    let formatted_message = msg.author.name.to_string().to_uppercase() + " Said: " + &msg.content + "\n";

    let mut message_storage_hashmap_locked = message_storage_hashmap.lock().await;

    if !message_storage_hashmap_locked.contains_key(channel_id) {
        message_storage_hashmap_locked.insert(*channel_id, Vec::new());
    }

    if let Some(message_storage_vector) = message_storage_hashmap_locked.get_mut(channel_id) {
        if message_storage_vector.len() > 5 {
            message_storage_vector.remove(0);
        }
        message_storage_vector.push(formatted_message);
    }

}
