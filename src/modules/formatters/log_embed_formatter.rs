use serde_json::Value;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use serenity::model::channel::{Attachment, Message};
use serenity::{client::Context, utils::Colour};
use std::collections::HashMap;

pub async fn log_embed_formatter(ctx: &Context, msg: &Message) -> Vec<CreateEmbed> {
    let mut create_embed_vec: Vec<CreateEmbed> = Vec::new();

    let hash_author = HashMap::from([
        (
            "icon_url",
            Value::String(msg.author.avatar_url().unwrap_or_default()),
        ),
        ("name", Value::String(msg.author.name.to_owned())),
    ]);

    let author = CreateEmbedAuthor(hash_author);

    let content = msg.content.to_owned();

    let guild_id = msg
        .guild_id
        .map_or_else(|| "Unknown-Guild".to_owned(), |id| id.to_string());

    let channel_id = msg.channel_id.to_string();

    let channel_name = msg.channel(&ctx.http).await.map_or_else(
        |_| "Unknown-Channel".to_owned(),
        |channel| channel.to_string(),
    );

    let id = msg.id.to_string();

    let link = format!(
        "[**Jump To Message**](https://discord.com/channels/{}/{}/{})",
        guild_id, channel_id, id
    );

    let formatted_desc = format!(
        "** Message sent in ** {}\n{}\n{}",
        channel_name, link, content
    );

    let mut message_embed = CreateEmbed::default();

    message_embed
        .set_author(author)
        .colour(Colour::MAGENTA)
        .description(formatted_desc)
        .thumbnail(msg.author.avatar_url().unwrap_or_default())
        .footer(|footer| footer.text(format!("User ID: {}", msg.author.id)))
        .timestamp(msg.timestamp);

    for (i, attachment) in msg.attachments.iter().enumerate() {
        if i == 0 {
            message_embed.image(&attachment.url);
        } else {
            let mut image_embed = CreateEmbed::default();
            image_embed
                .title(&attachment.filename)
                .image(&attachment.url)
                .colour(Colour::MAGENTA);
            create_embed_vec.push(image_embed);
        }
    }

    create_embed_vec.push(message_embed);

    create_embed_vec
}
