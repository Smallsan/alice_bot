mod commands;
mod modules;

use crate::commands::user_commands::*;
use crate::commands::admin_commands::*;

use modules::channel_message_logger::channel_message_logger;
use modules::message_stalker::message_stalker;
use modules::message_throwback::message_throwback;

use sea_orm::Database;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fs;
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
pub struct MessageStorage {
    messages: Vec<String>,
}
impl MessageStorage {
    pub fn new() -> Self {
        MessageStorage {
            messages: Vec::with_capacity(5),
        }
    }

    pub fn add_message(&mut self, message: &str) {
        if self.messages.len() >= 5 {
            self.messages.remove(0);
        }
        self.messages.push(message.to_string());
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(quit, bubble)]
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
        message_throwback(&ctx, &msg).await;
        message_stalker(&msg).await;
        }
    }


#[tokio::main]
async fn main() {

    let database_connection = connect_database().await;

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


#[derive(Serialize, Deserialize)]
struct Keys {
    discord_api_key: String,
    discord_test_api_key: String,
}

/// Gets The Discord Bot Token From The Config File
fn get_token_from_json() -> String{
    create_directory("config/keys.json");
    let mut file = File::open("config/keys.json").expect("Unable to find keys.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read keys.json");
    let keys: Keys = serde_json::from_str(&contents).expect("Unable to parse keys.json");
    let token = keys.discord_test_api_key;
    return token;
}

/// Sets Up a Database Connection
async fn connect_database() -> DatabaseConnection{
    create_directory("database");
    let database: DatabaseConnection = Database::connect("sqlite://database/database.sqlite?mode=rwc").await.expect("Unable to connect to database");
    return database;

}

/// Creates a Directory & Returns if The Directory Already Exists
fn create_directory(directory_name: &str) {
    if fs::metadata(directory_name).is_ok() {
        println!("Directory {} Already Exists", directory_name);
        return;
    }
    fs::create_dir(directory_name).expect("Error creating directory");
    println!("Directory {} Has Been Created", directory_name);
}



