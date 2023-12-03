mod commands;
mod modules;

use crate::commands::admin_commands::*;
use crate::commands::user_commands::*;

use modules::channel_msg_logger::channel_msg_logger;
use modules::msg_stalker::msg_stalker;
use modules::msg_storage_logger::msg_storage_logger;

use sea_orm::Database;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
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
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::error;

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

pub struct ParsedConfig {
    log_channel_id: u64,
    log_channel_enabled: bool,
    stalker_user_id: u64,
    stalker_receiver_id: u64,
    stalker_enabled: bool,
}

impl TypeMapKey for ParsedConfig {
    type Value = Arc<Mutex<ParsedConfig>>;
}

#[group]
#[commands(quit, bubble, backtrack)]
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
        channel_msg_logger(&ctx, &msg).await;
        msg_storage_logger(&ctx, &msg).await;
        msg_stalker(&ctx, &msg).await;
    }
}

#[tokio::main]
async fn main() {
    let database_connection = connect_database().await;

    tracing_subscriber::fmt::init();

    let token = get_key_from_json();

    let http = Http::new(&token);

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
        data.insert::<ParsedConfig>(Arc::new(config_parser(load_config()).into()));
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

#[derive(Serialize, Deserialize)]
struct Keys {
    discord_api_key: String,
}
#[derive(Serialize, Deserialize)]
struct Config {
    log_channel_id: String,
    log_channel_enabled: String,
    stalker_user_id: String,
    stalker_receiver_id: String,
    stalker_enabled: String,
}

impl TypeMapKey for Config {
    type Value = Arc<Mutex<Config>>;
}

/// Gets The Discord Bot Token From The Config File.
fn get_key_from_json() -> String {
    create_directory("config/keys.json");
    let mut key_file = File::open("config/keys.json").expect("Unable to find keys.json");
    let mut contents = String::new();
    key_file
        .read_to_string(&mut contents)
        .expect("Unable to read keys.json");
    let keys: Keys = serde_json::from_str(&contents).expect("Unable to parse keys.json");
    let token = keys.discord_api_key;
    return token;
}

/// Loads Config From Config File And Returns It.
fn load_config() -> Config {
    create_directory("config/config.json");
    let mut config_file = File::open("config/config.json").expect("Unable to find config.json");
    let mut contents = String::new();
    config_file
        .read_to_string(&mut contents)
        .expect("Unable to read config.json");
    let config: Config = serde_json::from_str(&contents).expect("Unable to parse config.json");
    return config;
}

fn config_parser(config: Config) -> ParsedConfig {
    ParsedConfig {
        log_channel_id: (config
            .log_channel_id
            .parse::<u64>()
            .expect("Unable to parse log_channel_id")),
        log_channel_enabled: (config
            .log_channel_enabled
            .parse::<bool>()
            .expect("Unable to parse log_channel_enabled")),
        stalker_user_id: (config
            .stalker_user_id
            .parse::<u64>()
            .expect("Unable to parse stalker_user_id")),
        stalker_receiver_id: (config
            .stalker_receiver_id
            .parse::<u64>()
            .expect("Unable to parse stalker_receiver_id")),
        stalker_enabled: (config
            .stalker_enabled
            .parse::<bool>()
            .expect("Unable to parse log_channel_enabled")),
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

/// Creates a Directory & Returns if The Directory Already Exists.
fn create_directory(directory_name: &str) {
    if fs::metadata(directory_name).is_ok() {
        println!("Directory {} Already Exists", directory_name);
        return;
    }
    fs::create_dir(directory_name).expect("Error creating directory");
    println!("Directory {} Has Been Created", directory_name);
}
