use serenity::client::Context;
use serenity::model::channel::Message;

use crate::MessageStorageContainer;

pub async fn msg_storage_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || msg.content.contains("!backtrack") || !msg.sticker_items.is_empty(){
        return;
    }

    let channel_id: &u64 = msg.channel_id.as_u64();

    let msg_storage = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorageContainer>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };

    let mut msg_storage_locked = msg_storage.lock().await;

    if !msg_storage_locked.contains_key(channel_id) {
        msg_storage_locked.insert(*channel_id, Vec::new());
    }

    let formatted_msg = fetch_formatted_message(&msg);

    if let Some(msg_vector) = msg_storage_locked.get_mut(channel_id) {
        if msg_vector.len() > 5 {
            msg_vector.remove(0);
        }
        msg_vector.push(formatted_msg);
    }
}

fn fetch_formatted_message(msg: &Message) -> String {
    let replied_msg = fetch_replied_msg(&msg);
    let author_msg = fetch_author_msg(&msg);

    return format!("┌── {}\n  {}", replied_msg, author_msg)
}

fn fetch_replied_msg(msg: &Message) -> String {
    if let Some(replied_msg_box) = &msg.referenced_message {
        return format!(
            "┌── {} Said: {}\n  ",
            replied_msg_box.author.name.to_uppercase(),
            &replied_msg_box.content
        )
    } else {
        return String::new()
    }
}

fn fetch_author_msg(msg: &Message) -> String {
    return format!(
        "{} Said: {}\n  ",
        msg.author.name.to_uppercase(),
        &msg.content
    )
}