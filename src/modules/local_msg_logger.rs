use chrono::Local;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::guild::Guild;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub async fn local_message_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }
    download_attachments(&msg).await;
    log_message(&ctx, &msg).await;
}

async fn download_attachments(msg: &Message) {
    for attachment in &msg.attachments {
        if let Ok(attachment_raw) = attachment.download().await {
            let file_directory = format!("log/attachments/{}", attachment.filename);
            println!("{}", attachment.filename);
            println!("{}", file_directory);
            let mut file = File::create(&file_directory)
                .expect(&format!("Failed to create {:?}", &file_directory));
            file.write_all(&attachment_raw)
                .expect(&format!("Failed to create {:?}", file_directory));
        }
    }
}

async fn log_message(ctx: &Context, msg: &Message) {
    let author = &msg.author.name;

    let channel_name = match msg.channel(&ctx.http).await {
        Ok(channel) => {
            if let Some(guild) = channel.guild() {
                guild.name
            } else {
                "Unknown-Channel".to_string()
            }
        }
        Err(_) => "Unknown-Channel".to_string(),
    };

    let guild_name = match msg.guild_id {
        Some(guild_id) => {
            if let Ok(guild) = ctx.http.get_guild(guild_id.as_u64().clone()).await {
                guild.name.to_string()
            } else {
                "Unknown-Guild".to_string()
            }
        }
        None => "Unknown-Guild".to_string(),
    };

    let content: String = msg.content.to_string();

    let datetime = Local::now();

    let date = datetime.format("%m-%d-%Y").to_string();

    let time = datetime.format("%H:%M");

    let formatted_message = format!("[{}][{}][{}] - {}", channel_name, author, time, content);

    let directory = format!("log/{} [{}].txt", guild_name, date);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&directory)
    {
        if let Err(_) = writeln!(file, "{}", formatted_message) {
            println!("Couldn't write to file: {:?}", &directory);
        }
    }
}
