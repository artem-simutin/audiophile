use audiophile::load_env_from_file;
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

use commands::*;

#[group]
#[commands(ping)]
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
        .await
        .expect("Error creating client!");

    // If error occurs while starting the client => log the error
    if let Err(why) = client.start().await {
        println!("Something went wrong starging the client: {:?}", why)
    }
}
