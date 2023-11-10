use serenity::{model::prelude::ChannelId, prelude::Context};
use songbird::id::GuildId;
use youtube::prelude::AudiophileError;

use crate::structs::{queue::ContextData, song::Song};

pub async fn play_latest_song(
    ctx: &Context,
    guild_id: &GuildId,
    voice_channel_id: &ChannelId,
) -> Result<Song, AudiophileError> {
    let latest_song = {
        let data = ctx.data.read().await;
        let data = data.get::<ContextData>().ok_or(AudiophileError {
            location: "play_latest_song",
            message: "Cannot get bot context data",
        })?;
        let server_data = data.get(guild_id).ok_or(AudiophileError {
            location: "play_latest_song",
            message: "Cannot get the server context with this guild id",
        })?;
        let queue = server_data
            .voice_queues
            .get(voice_channel_id)
            .ok_or(AudiophileError {
                location: "play_latest_song",
                message: "Cannot get current voice channel data context",
            })?;

        let latest_song = queue.songs.front().ok_or(AudiophileError {
            location: "play_latest_song",
            message: "There is no song to play in the queue",
        })?;
        latest_song.clone()
    };

    let sb = songbird::get(ctx)
        .await
        .expect("Sonbird is placed on initialization")
        .clone();

    let handler = sb.get(*guild_id).ok_or(AudiophileError {
        location: "play_latest_song",
        message: "There is no voice handler initialized on this server",
    })?;

    let source = songbird::ytdl(latest_song.url.as_str())
        .await
        .map_err(|_why| AudiophileError {
            location: "play_latest_song::songbird::ytdl",
            message: "Something went wrong generatin source from the url",
        })?;

    {
        let mut data = ctx.data.write().await;
        let data = data
            .get_mut::<ContextData>()
            .expect("There is no bot context! Bimbo!");
        let server_data = data.get_mut(&guild_id).ok_or(AudiophileError {
            location: "play_latest_song::write_playing",
            message: "There is not server data for this server!",
        })?;
        let queue = server_data
            .voice_queues
            .get_mut(&voice_channel_id)
            .ok_or(AudiophileError {
                location: "play_latest_song::write_playing",
                message: "There is no queue for this voice channel!",
            })?;
        queue.set_playing(true)
    }

    {
        let mut handler = handler.lock().await;
        handler.play_source(source);
    }

    Ok(latest_song)
}
