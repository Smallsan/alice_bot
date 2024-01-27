use sea_orm::Database;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serenity::model::Timestamp;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::error;

mod commands;
mod modules;

use crate::commands::admin_commands::*;
use crate::commands::user_commands::*;

use modules::channel_msg_logger::channel_msg_logger;
use modules::local_msg_logger::local_logger;
use modules::msg_stalker::msg_stalker;
use modules::msg_storage_logger::msg_storage_logger;
use modules::tools::config_manager::load_config;
use modules::tools::create_directory::create_directory;
use modules::tools::key_manager::get_key_from_json;

pub struct UserCooldowns {
    user_cooldowns: HashMap<u64, Timestamp>
    
}
pub struct CommandCooldownContainer {
    command_cooldowns: HashMap<u64, UserCooldowns>
}

pub struct LogMutex;

impl TypeMapKey for LogMutex {
    type Value = Arc<Mutex<()>>;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct MessageStorageContainer;

impl TypeMapKey for MessageStorageContainer {
    type Value = Arc<Mutex<HashMap<u64, Vec<String>>>>;
}

pub struct DatabaseConnectionContainer;

impl TypeMapKey for DatabaseConnectionContainer {
    type Value = Arc<Mutex<DatabaseConnection>>;
}

#[derive(Serialize, Deserialize)]
pub struct Keys {
    discord_api_key: String,
}

pub struct ParsedConfig {
    log_channel_id: u64,
    log_channel_enabled: bool,
    local_logger_enabled: bool,
    stalker_user_id: u64,
    stalker_receiver_id: u64,
    stalker_enabled: bool,
}

impl TypeMapKey for ParsedConfig {
    type Value = Arc<Mutex<ParsedConfig>>;
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    log_channel_id: String,
    log_channel_enabled: String,
    local_logger_enabled: String,
    stalker_user_id: String,
    stalker_receiver_id: String,
    stalker_enabled: String,
}

#[group]
#[commands(quit, restart, bubble, backtrack, halal, haram, rep)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        println!("Resumed");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        channel_msg_logger(&ctx, &msg).await;
        local_logger(&ctx, &msg).await;
        msg_storage_logger(&ctx, &msg).await;
        msg_stalker(&ctx, &msg).await;
    }
}

#[tokio::main]
async fn main() {

    let database_connection = connect_database().await;

    tracing_subscriber::fmt::init();

    let token = get_key_from_json().discord_api_key;

    let http = Http::new(&token);

    create_directory("log");

    create_directory("log/attachments");

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        // Intitiating the values that are accessible throughout the project
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<MessageStorageContainer>(Arc::new(Mutex::new(HashMap::new())));
        data.insert::<DatabaseConnectionContainer>(Arc::new(database_connection.into()));
        data.insert::<ParsedConfig>(Arc::new(load_config().into()));
        data.insert::<LogMutex>(Arc::new(Mutex::new(())));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

/// Sets Up a Database Connection.
async fn connect_database() -> DatabaseConnection {
    create_directory("database");
    let database: DatabaseConnection =
        Database::connect("sqlite://database/database.sqlite?mode=rwc")
            .await
            .expect("Unable to connect to database");
    return database;
}
