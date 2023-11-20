use serenity::model::channel::Message;
use serenity::client::Context;
use crate::MessageStorageContainer;

pub async fn message_storage_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || msg.content.contains("!backtrack"){
        return;
    }

    let channel_id = msg.channel_id.as_u64();

    let message_storage_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorageContainer>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };

    let mut message_storage_hashmap_locked = message_storage_hashmap.lock().await;

    if !message_storage_hashmap_locked.contains_key(channel_id) {
        message_storage_hashmap_locked.insert(*channel_id, Vec::new());
    }

    let mut formatted_message = String::new();

    if let Some(message_storage_vector) = message_storage_hashmap_locked.get_mut(channel_id) {
        if message_storage_vector.len() > 5 {
            message_storage_vector.remove(0);
        }
        formatted_message += &get_replied_message(msg);
        formatted_message += &get_author_message(msg);
        message_storage_vector.push(formatted_message);
    }

}

fn get_replied_message(msg: &Message) -> String {

    if let Some(replied_message_box) = &msg.referenced_message {
        return format!("┌── {:?} Said: {:?} \n  ", replied_message_box.author.name.to_uppercase(), &replied_message_box.content)
    }

    return String::new();
}

fn get_author_message(msg: &Message) -> String {
    let author_message = format!("{:?} Said: {:?} \n  ", msg.author.name.to_string().to_uppercase(), &msg.content);
    
    return author_message;
}