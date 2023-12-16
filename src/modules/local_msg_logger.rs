use chrono::Local;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use crate::ParsedConfig;
struct LocalLoggerConfig {
    enabled: bool,
}

pub async fn local_logger(ctx: &Context, msg: &Message) {
    if msg.author.bot || !msg.sticker_items.is_empty(){
        return;
    }

    let config: LocalLoggerConfig = fetch_config(&ctx).await;

    if !config.enabled {
        return;
    }

    download_attachments(&msg).await;
    log_message(&ctx, &msg).await;
}

async fn download_attachments(msg: &Message) {
    if msg.attachments.is_empty() {
        return
    }
    for attachment in &msg.attachments {
        if let Ok(attachment_raw) = attachment.download().await {
            let file_directory: String = format!("log/attachments/{}", attachment.filename);

            let mut file: File = File::create(&file_directory)
                .expect(&format!("Failed to create {:?}", &file_directory));

            file.write_all(&attachment_raw)
                .expect(&format!("Failed to create {:?}", file_directory));
        }
    }
}

async fn log_message(ctx: &Context, msg: &Message) {
    let author: &String = &msg.author.name;

    let channel_name: String = match msg.channel(&ctx.http).await {
        Ok(channel) => {
            if let Some(guild) = channel.guild() {
                guild.name
            } else {
                "Unknown-Channel".to_string()
            }
        }
        Err(_) => "Unknown-Channel".to_string(),
    };

    let guild_name: String = match msg.guild_id {
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

    let date: String = datetime.format("%m-%d-%Y").to_string();

    let time = datetime.format("%H:%M");

    let formatted_message: String =
        format!("[{}][{}][{}] - {}", channel_name, author, time, content);

    let directory: String = format!("log/{} [{}].txt", guild_name, date);

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

async fn fetch_config(ctx: &Context) -> LocalLoggerConfig {
    let config_hashmap = {
        let data_read = ctx.data.read().await;

        data_read
            .get::<ParsedConfig>()
            .expect("Expected Parsed Config In TypeMap.")
            .clone()
    };

    let config_hashmap_locked = config_hashmap.lock().await;

    return LocalLoggerConfig {
        enabled: config_hashmap_locked.local_logger_enabled,
    };
}
