use std::env;

use serenity::prelude::GatewayIntents;

pub struct Config {
    pub name: String,
    pub url: String,
    pub token: String,
    pub prefix: String,
    pub intents: GatewayIntents,
}

impl Config {
    pub fn new() -> Self {
        let token =
            env::var("BOT_TOKEN").expect("BOT_TOKEN variable must be presented in `process` scope");
        let prefix = ">>".to_string();
        let intents = GatewayIntents::non_privileged()
            | GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILD_VOICE_STATES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::DIRECT_MESSAGES;
        return Config {
            name: "Audiophile".to_string(),
            url: "https://github.com/artem-simutin/audiophile".to_string(),
            prefix,
            token,
            intents,
        };
    }
}
