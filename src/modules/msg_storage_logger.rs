use crate::MessageStorageContainer;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn msg_storage_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || msg.content.contains("!backtrack") {
        return;
    }

    let channel_id = msg.channel_id.as_u64();

    let msg_storage_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorageContainer>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };

    let mut msg_storage_hashmap_locked = msg_storage_hashmap.lock().await;

    if !msg_storage_hashmap_locked.contains_key(channel_id) {
        msg_storage_hashmap_locked.insert(*channel_id, Vec::new());
    }

    let mut formatted_msg = String::new();

    if let Some(msg_storage_vector) = msg_storage_hashmap_locked.get_mut(channel_id) {
        if msg_storage_vector.len() > 5 {
            msg_storage_vector.remove(0);
        }
        formatted_msg += &get_replied_msg(msg);
        formatted_msg += &get_author_msg(msg);
        msg_storage_vector.push(formatted_msg);
    }
}

fn get_replied_msg(msg: &Message) -> String {
    if let Some(replied_msg_box) = &msg.referenced_message {
        return format!(
            "┌── {:?} Said: {:?} \n  ",
            replied_msg_box.author.name.to_uppercase(),
            &replied_msg_box.content
        );
    }

    return String::new();
}

fn get_author_msg(msg: &Message) -> String {
    let author_msg = format!(
        "{:?} Said: {:?} \n  ",
        msg.author.name.to_string().to_uppercase(),
        &msg.content
    );

    return author_msg;
}
