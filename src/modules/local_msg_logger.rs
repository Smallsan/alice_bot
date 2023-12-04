use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn log_message(ctx: &Context, msg: &Message) {
    if msg.author.bot{
        return
    }

    let msg_author = &msg.author.name;
    let msg_content: String = msg.content.to_string();
    let msg_channel_name = if let Ok(msg_channel) = msg.channel(&ctx.http).await {
        msg_channel.to_string()
    } else {
        "Unknown Channel".to_string()
    };
    let msg_time = msg.timestamp.date();
    let msg_attachments_url: Vec<String> = msg.attachments.iter().map(|attachment| attachment.url.clone()).collect();
    let msg_date = msg.timestamp.time();
    let formatted_message = format!("[{:?}][{:?}][{:?}] - {:?}", msg_channel_name, msg_author, msg_time, msg_content);

}