mod commands;
mod modules;

use crate::commands::user_commands::*;
use crate::commands::admin_commands::*;

use modules::channel_message_logger::channel_message_logger;
use modules::message_stalker::message_stalker;


use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::Message;
use tracing::error;




pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(ping, quit, mom)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        channel_message_logger(&ctx, &msg).await;
        message_stalker(&ctx, &msg).await;
        }
        
    }


#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let token = get_token_from_json();

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("!")).group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Config {
    discord_api_key: String,
}

fn get_token_from_json() -> String{
    let mut file = File::open("config/keys.json").expect("Unable to find keys.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read keys.json");
    let config: Config = serde_json::from_str(&contents).expect("Unable to parse keys.json");

    let token = config.discord_api_key;
    return token;
}

