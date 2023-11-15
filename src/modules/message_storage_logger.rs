use serenity::model::channel::Message;
use serenity::client::Context;
use crate::MessageStorage;

pub async fn message_storage_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || msg.content.contains("!backtrack"){
        return;
    }

    let message_storage = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorage>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };

    let formatted_message = msg.author.name.to_string().to_uppercase() + " Said: " + &msg.content + "\n";

    let mut message_storage_locked = message_storage.lock().await;
    if message_storage_locked.len() > 5 {
        message_storage_locked.remove(0);
    }

    message_storage_locked.push(formatted_message);
}
