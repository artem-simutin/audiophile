use std::collections::HashMap;

use serenity::{model::prelude::ChannelId, prelude::TypeMapKey};

pub mod context;
pub struct ServerContexts;

impl TypeMapKey for ServerContexts {
    type Value = HashMap<ChannelId, context::ServerContext>;
}
