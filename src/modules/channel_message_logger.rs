use std::collections::HashMap;
use serde_json::Value;
use serenity::builder::CreateEmbedAuthor;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::utils::Colour;


pub async fn log_message(ctx: &Context, msg: &Message) {
    let hash_author = HashMap::from([
        ("icon_url", Value::String(msg.author.avatar_url().unwrap_or_default())),
        ("name", Value::String(msg.author.name.to_string())),
    ]);
    let author = CreateEmbedAuthor(hash_author);
    if !msg.author.bot {
        let _msg = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content(&msg.content)
                        .embed(|e| {
                            e.set_author(author)
                            .colour(Colour::MAGENTA)
                                .description("This is a description")
                                .thumbnail(&msg.author.avatar_url().unwrap_or_default())
                                .fields(vec![
                                    ("This is the first field", "This is a field body", true),
                                    ("This is the second field", "Both fields are inline", true),
                                ])
                                .field("This is the third field", "This is not an inline field", false)
                                .footer(|f| f.text("User ID: ".to_string() + &msg.author.id.to_string() ))
                                .timestamp(msg.timestamp)
                        })
                })
                .await;

}
}



