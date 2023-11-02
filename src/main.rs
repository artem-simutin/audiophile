mod commands;
mod config;
mod prelude;
mod utils;

use commands::play::*;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::{async_trait, Client};
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    prelude::EventHandler,
};
use songbird::SerenityInit;
use utils::check_msg_err;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅🌎 Bot is ready! - {}:{}", ready.user.name, ready.version)
    }
}

// static VIDEO_ID: &str = "https://www.youtube.com/watch?v=H2PYtvIYDHE";
#[group]
#[commands(ping, play)]
struct General;

#[tokio::main]
async fn main() {
    if let Err(why) = dotenv::dotenv() {
        println!(
            "Something went wrong loading .env file into environment, {:?}",
            why
        );
        std::process::exit(1);
    };

    let config = config::Config::new();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix).ignore_bots(true))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::all()
        | GatewayIntents::default()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("❌ Something went wrong building the client");

    if let Err(why) = client.start().await {
        println!("❌ Something went wrong starting the client: {}", why)
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg_err(msg.reply(&ctx.http, "Pong! 😎").await);
    Ok(())
}
