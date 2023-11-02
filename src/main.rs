use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::prelude::Context;
use serenity::{async_trait, Client};
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    prelude::EventHandler,
};

mod config;
mod prelude;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅🌎 Bot is ready! - {}:{}", ready.user.name, ready.version)
    }
}

// static VIDEO_ID: &str = "https://www.youtube.com/watch?v=H2PYtvIYDHE";
#[group]
struct Songs;

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
        .group(&SONGS_GROUP);

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&config.token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("❌ Something went wrong building the client");

    if let Err(why) = client.start().await {
        println!("❌ Something went wrong starting the client: {:?}", why)
    }
}
