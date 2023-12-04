use serde_json::Value;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use serenity::model::channel::{Attachment, Message};
use serenity::{client::Context, utils::Colour};
use std::collections::HashMap;

pub async fn log_embed_formatter(ctx: &Context, msg: &Message) -> Vec<CreateEmbed> {
    let mut create_embed_vec: Vec<CreateEmbed> = vec![];
    let hash_author = HashMap::from([
        (
            "icon_url",
            Value::String(msg.author.avatar_url().unwrap_or_default()),
        ),
        ("name", Value::String(msg.author.name.to_string())),
    ]);

    let attachment_vec: &Vec<Attachment> = &msg.attachments;

    let author = CreateEmbedAuthor(hash_author);

    let content: String = msg.content.to_string();
    let guild_id: String = msg.guild_id.unwrap_or_default().to_string();
    let channel_id: String = msg.channel_id.to_string();
    let channel_name = match msg.channel(&ctx.http).await {
        Ok(channel) => channel.to_string(),
        Err(_) => "Unknown-Channel".to_string(),
    };

    let id: String = msg.id.to_string();
    let link: String = format!(
        "[**Jump To Message**](https://discord.com/channels/{}/{}/{})",
        guild_id, channel_id, id
    );
    let msg_channel_link: String = format!("{}", channel_name);

    let formatted_desc: String = format!(
        "** Message sent in ** {}\n{}\n{}",
        msg_channel_link, link, content
    );

    let mut message_embed = CreateEmbed::default();
    message_embed
        .set_author(author)
        .colour(Colour::MAGENTA)
        .description(formatted_desc)
        .thumbnail(msg.author.avatar_url().unwrap_or_default())
        .footer(|footer| footer.text("User ID: ".to_string() + &msg.author.id.to_string()))
        .timestamp(msg.timestamp);

    let mut attachment_counter = 0;
    for attachment in attachment_vec {
        attachment_counter += 1;
        if attachment_counter == 1 {
            message_embed.image(&attachment.url);
            continue;
        }
        let mut image_embed = CreateEmbed::default();
        image_embed
            .title(&attachment.filename)
            .image(&attachment.url)
            .colour(Colour::MAGENTA);
        create_embed_vec.push(image_embed);
    }
    create_embed_vec.insert(0, message_embed);

    return create_embed_vec;
}
