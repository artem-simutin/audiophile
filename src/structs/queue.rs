use std::collections::{HashMap, VecDeque};

use serenity::{model::prelude::ChannelId, prelude::TypeMapKey};
use songbird::id::GuildId;

use super::song::Song;

pub struct ContextData {}

type ContextDataValue = HashMap<GuildId, ServerData>;

impl TypeMapKey for ContextData {
    type Value = ContextDataValue;
}

impl ContextData {
    pub fn new() -> ContextDataValue {
        HashMap::new()
    }
}

pub struct ServerData {
    pub voice_queues: HashMap<ChannelId, Queue>,
}

impl Default for ServerData {
    fn default() -> Self {
        Self {
            voice_queues: HashMap::new(),
        }
    }
}

pub struct Queue {
    playing: bool,
    pub songs: VecDeque<Song>,
}

impl Queue {
    pub fn is_playing(&self) -> bool {
        self.playing
    }
    pub fn set_playing(&mut self, v: bool) -> () {
        self.playing = v
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            playing: false,
            songs: VecDeque::new(),
        }
    }
}
