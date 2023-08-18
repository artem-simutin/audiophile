use std::collections::HashMap;

use serenity::{model::prelude::ChannelId, prelude::TypeMapKey};

pub struct ServersQueries;

impl TypeMapKey for ServersQueries {
    type Value = HashMap<ChannelId, ServerQuery>;
}

pub struct ServerQuery {
    pub songs: Vec<String>,
    pub looped: bool,
}

impl ServerQuery {
    pub fn new() -> Self {
        ServerQuery {
            songs: vec![],
            looped: false,
        }
    }

    pub fn add_song(self: &mut Self, song_url: &String) {
        self.songs.push(song_url.clone());
    }
}
