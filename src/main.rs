use std::collections::HashMap;

use audiophile::{load_env_from_file, server_queries::ServersQueries};
mod commands;
mod config;
use config::Config;
use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    model::prelude::Ready,
    prelude::{Context, EventHandler},
    Client,
};

use songbird::SerenityInit;

use commands::join::*;
use commands::ping::*;
use commands::play::*;

#[group]
#[commands(ping, play, join)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(self: &Self, _ctx: Context, ready: Ready) {
        println!("✅ Bot is ready! Name is: {}", &ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Load enviromental variables from the `.env` file
    load_env_from_file();

    // Create new config
    let config = Config::new();

    // Initialize `serenity` framework
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.prefix))
        .group(&GENERAL_GROUP);

    // Initialize `serenity` discord client
    let mut client = Client::builder(config.token, config.intents)
        .framework(framework)
        .event_handler(Handler)
        .type_map_insert::<ServersQueries>(HashMap::default())
        .register_songbird()
        .await
        .expect("Error creating client!");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
}
