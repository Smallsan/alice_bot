use serenity::model::channel::Message;
use serenity::client::Context;


pub async fn message_throwback(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }

    if msg.content.starts_with("!throwback") {
        let mut splited_message = msg.content.splitn(2, ' ');

        splited_message.next();

        if let Some(argument) = splited_message.next() {
            msg.channel_id.send_message(&ctx.http, |message| message.content("Throwback")).await.unwrap();
        } else {
            msg.channel_id.send_message(&ctx.http, |message| message.content("Throwback")).await.unwrap();
        }
    }
}