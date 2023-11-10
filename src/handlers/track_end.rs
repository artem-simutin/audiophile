use serenity::{
    async_trait,
    model::prelude::{ChannelId, GuildId, Message},
    prelude::Context,
};
use songbird::{Event, EventContext, EventHandler};

use crate::{
    functions::play_latest_song::play_latest_song, structs::queue::ContextData,
    utils::check_msg_err,
};

pub struct TrackEndHandler {
    pub ctx: Context,
    pub msg: Message,
    pub guild_id: GuildId,
    pub voice_channel_id: ChannelId,
}

#[async_trait]
impl EventHandler for TrackEndHandler {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        {
            let mut data = self.ctx.data.write().await;
            let data = data
                .get_mut::<ContextData>()
                .expect("There is not bot context data!");
            let server_data = match data.get_mut(&self.guild_id.into()) {
                Some(sd) => sd,
                None => return None,
            };
            let queue = match server_data.voice_queues.get_mut(&self.voice_channel_id) {
                Some(q) => q,
                None => return None,
            };

            queue.songs.pop_front();

            let songs_remaining = queue.songs.len();

            if songs_remaining == 0 {
                // No songs in the queue to play
                let sb = songbird::get(&self.ctx)
                    .await
                    .expect("Songbird is placed on initialization")
                    .clone();
                let handler_lock = match sb.get(self.guild_id) {
                    Some(vm) => vm,
                    None => return None,
                };
                let mut handler = handler_lock.lock().await;
                handler.stop();
                let _ = handler.leave().await;
                queue.set_playing(false);
                return None;
            };
        }

        match play_latest_song(&self.ctx, &self.guild_id.into(), &self.voice_channel_id).await {
            Ok(_s) => return None,
            Err(why) => {
                check_msg_err(self.msg.reply(&self.ctx.http, why.to_string()).await);
                return None;
            }
        };
    }
}
