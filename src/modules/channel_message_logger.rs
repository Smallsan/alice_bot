use std::collections::HashMap;
use serde_json::Value;
use serenity::builder::CreateEmbedAuthor;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;
use serenity::utils::Colour;




pub async fn log_message(ctx: &Context, msg: &Message){
    if msg.author.bot {
        return;
    }
        let log_channel_id: ChannelId = ChannelId(967685973456609320);
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

        let _msg = log_channel_id
                .send_message(&ctx.http, |m| {
                    m
                        .embed(|e| {
                            e.set_author(author)
                            .colour(Colour::MAGENTA)
                                .description(formatted_description)
                                .thumbnail(&msg.author.avatar_url().unwrap_or_default())
                                .footer(|f| f.text("User ID: ".to_string() + &msg.author.id.to_string() ))
                                .timestamp(msg.timestamp)
                        })
                })
                .await;

            }





