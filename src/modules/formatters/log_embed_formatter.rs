use serenity::{client::Context, utils::Colour};
use serenity::model::channel::Message;
use std::collections::HashMap;
use serde_json::Value;
use serenity::builder::{CreateEmbedAuthor, CreateEmbed};


pub async fn log_embed_formatter(ctx: &Context, msg: &Message) -> CreateEmbed{
        let attachment: String = match msg.attachments.first()
        {
            Some(first_attachment) => first_attachment.url.to_owned(),
            None => "".to_owned(),

        };
        let hash_author = HashMap::from([
            ("icon_url", Value::String(msg.author.avatar_url().unwrap_or_default())),
            ("name", Value::String(msg.author.name.to_string())),
        ]);

        let author = CreateEmbedAuthor(hash_author);
        
        let message_content: String = msg.content.to_string();
        let message_guild_id: String = msg.guild_id.unwrap_or_default().to_string();
        let message_channel_id: u64 = *msg.channel_id.as_u64();
        let message_channel_name: String = ctx.http.get_channel(message_channel_id).await.unwrap().to_string();
        let message_id: String = msg.id.to_string();
        let message_link: String = format!("[**Jump To Message**](https://discord.com/channels/{}/{}/{})",message_guild_id,message_channel_id,message_id);
        let message_channel_link: String = format!("{}",message_channel_name);
        let formatted_description: String = format!("** Message sent in ** {}\n{}\n{}",message_channel_link,message_link,message_content);


        let mut embed = CreateEmbed::default();
        embed
        .set_author(author)
        .colour(Colour::MAGENTA)
        .description(formatted_description)
        .thumbnail(msg.author.avatar_url().unwrap_or_default())
        .footer(|footer| footer.text("User ID: ".to_string() + &msg.author.id.to_string() ))
        .timestamp(msg.timestamp)
        .attachment(attachment);
        

        embed


    }






            



