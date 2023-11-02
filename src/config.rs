use std::env;

pub struct Config {
    pub token: String,
    pub prefix: String,
}

impl Config {
    pub fn new() -> Self {
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        // if let Err(why) = validate_token(&token) {
        //     println!("❌ Your discord token is probably invalid: {:?}", why);
        //     panic!();
        // };
        Self {
            token,
            prefix: ">>".to_string(),
        }
    }
}
