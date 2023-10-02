mod config;
use config::Config;

use dotenv::dotenv;
use serenity::{builder::CreateEmbed, model::prelude::Message, Error};

// Loads environment variables from .env file
pub fn load_env_from_file() {
    dotenv().expect("Failed to load .env file");
}

pub fn check_message_send_status(status: Result<Message, Error>) {
    if let Err(why) = status {
        println!(
            "WARNING: Error sending message because of Discrod API.\n{}",
            why.to_string()
        )
    }
}
