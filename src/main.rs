use songbird::SerenityInit;
use std::collections::HashMap;

use audiophile::load_env_from_file;
mod commands;
mod config;
mod server;
use config::Config;
use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    model::prelude::{Activity, Ready},
    prelude::{Context, EventHandler},
    Client,
};
mod song;
use commands::ping::*;
use commands::play::*;
use server::ServerContexts;

#[group]
#[commands(ping, play)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(self: &Self, ctx: Context, ready: Ready) {
        let app_config = Config::new();
        ctx.set_activity(Activity::listening(format!(
            "you. Prefix: {}",
            app_config.prefix
        )))
        .await;
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
        .type_map_insert::<ServerContexts>(HashMap::default())
        .register_songbird()
        .await
        .expect("Error creating client!");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
}
