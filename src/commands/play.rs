use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::*,
    utils::Color,
};
use songbird::{Event, TrackEvent};
use youtube::YoutubeAPI;

use crate::{
    functions::play_latest_song::play_latest_song,
    handlers::track_end::TrackEndHandler,
    structs::{embed::AudiophileEmbeds, queue::ContextData, song::Song},
    utils::check_msg_err,
};

#[command]
#[aliases(p)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Check if the passed argument by the user exists
    let possible_song_url = match args.single::<String>() {
        Ok(arg) => arg,
        Err(_) => {
            check_msg_err(
                msg.reply(&ctx.http, "You must provide URL to a song!")
                    .await,
            );
            return Ok(());
        }
    };

    // Try to parse the given argument by the user
    let url = match youtube::url::Url::parse(&possible_song_url) {
        Ok(url) => url,
        Err(_) => {
            check_msg_err(msg.reply(&ctx.http, "Provide a valid URL").await);
            return Ok(());
        }
    };

    // Getting guild from the user's message
    let guild = match msg.guild(&ctx.cache) {
        Some(g) => g,
        None => {
            check_msg_err(
                msg.reply(&ctx.http, "Cannot get guild from your message!")
                    .await,
            );
            return Ok(());
        }
    };

    // Getting voice channel from the user's message
    let voice_channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id);

    // Checking if the voice channel has been got from the message
    let voice_channel_id = match voice_channel_id {
        Some(vs) => vs,
        None => {
            check_msg_err(
                msg.reply(&ctx.http, "You must be in a voice channel!")
                    .await,
            );
            return Ok(());
        }
    };

    // Trying to get the user's song from YouTube service

    let youtube_api = YoutubeAPI::new();
    let video = match youtube_api.video(url.as_str()) {
        Ok(vb) => vb,
        Err(why) => {
            check_msg_err(
                msg.reply(
                    &ctx.http,
                    format!(
                        "Something went wrong building video fetch struct: {}",
                        why.to_string()
                    ),
                )
                .await,
            );
            return Ok(());
        }
    };
    let result = match video.build().fetch().await {
        Ok(vr) => vr.items,
        Err(why) => {
            check_msg_err(
                msg.reply(
                    &ctx.http,
                    format!(
                        "Something went wrong fetching your song! {}",
                        why.to_string()
                    ),
                )
                .await,
            );
            return Ok(());
        }
    };
    let video = match result.get(0) {
        Some(vr) => vr,
        None => {
            check_msg_err(msg.reply(&ctx.http, "Your video is not found!").await);
            return Ok(());
        }
    };
    let song = match Song::try_from(video) {
        Ok(s) => s,
        Err(why) => {
            check_msg_err(
                msg.reply(
                    &ctx.http,
                    format!("Something went wrong converting video resourse into song struct! Creator of the bot is bimbo! {}", why.to_string()),
                )
                .await,
            );
            return Ok(());
        }
    };

    // Getting bot global voice manager (songbird)
    let voice_manager = songbird::get(ctx)
        .await
        .expect("Songbird placed in at initialization")
        .clone();

    // Joining channel or getting voice handler if already in the voice
    let (handler_lock, join_status) = voice_manager.join(guild.id, voice_channel_id).await;
    if let Err(why) = join_status {
        check_msg_err(
            msg.reply(
                &ctx.http,
                format!(
                    "Somethign went wrong connecting to the voice channel! {}",
                    why
                ),
            )
            .await,
        );
        return Ok(());
    }

    // Adding song to the queue
    {
        let mut data = ctx.data.write().await;
        let data = data.get_mut::<ContextData>().expect("Something went wrong adding song to the playlist! Tell to my creator that he is a bimbo 💀!");
        let server_context = data.entry(guild.id.into()).or_default();
        let voice_channel_context = server_context
            .voice_queues
            .entry(voice_channel_id)
            .or_default();
        voice_channel_context.songs.push_back(song.clone());
    }
    let is_playing = {
        // Get is song already playing or not
        let data = ctx.data.read().await;
        let context_data = data
        .get::<ContextData>()
        .expect("Something went wrong adding song to the playlist! Tell to my creator that he is a bimbo 💀!");
        let server_data = match context_data.get(&guild.id.into()) {
            Some(sd) => sd,
            None => {
                check_msg_err(
                    msg.reply(
                        &ctx.http,
                        "No this server data! Creator of this bot is bimbo!",
                    )
                    .await,
                );
                return Ok(());
            }
        };
        let queue = match server_data.voice_queues.get(&voice_channel_id) {
            Some(q) => q,
            None => {
                check_msg_err(
                    msg.reply(
                        &ctx.http,
                        "There is no queue for this voice channel! The bot creator is bimbo!",
                    )
                    .await,
                );
                return Ok(());
            }
        };

        let is_playing = queue.is_playing();

        is_playing
    };

    let author_avatar = match msg.author.avatar_url() {
        Some(aurl) => aurl,
        None => "https://logos-world.net/wp-content/uploads/2020/12/Discord-Emblem.png".to_string(),
    };
    let author_name = match msg.author_nick(&ctx.http).await {
        Some(an) => an,
        None => msg.author.name.clone(),
    };

    if is_playing {
        check_msg_err(
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title(&song.title)
                            .url(&song.url.as_str())
                            .thumbnail(&song.thumbnail_url)
                            .footer(AudiophileEmbeds::footer)
                            .color(Color::DARK_GREEN)
                            .author(|a| {
                                a.icon_url(&author_avatar)
                                    .name(format!("{} added to the queue!", author_name))
                            })
                    })
                })
                .await,
        );
        return Ok(());
    }

    // Add global event to the voice handler
    {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndHandler {
                ctx: ctx.clone(),
                msg: msg.clone(),
                guild_id: guild.id.clone(),
                voice_channel_id: voice_channel_id.clone(),
            },
        )
    }

    match play_latest_song(&ctx, &guild.id.into(), &voice_channel_id).await {
        Ok(s) => {
            check_msg_err(
                msg.channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.footer(AudiophileEmbeds::footer)
                                .title(&s.title)
                                .thumbnail(&s.thumbnail_url)
                                .url(s.url.as_str())
                                .color(Color::ORANGE)
                                .author(|a| {
                                    a.icon_url(&author_avatar)
                                        .name(format!("{} started playing a song!", author_name))
                                })
                        })
                    })
                    .await,
            );
        }
        Err(why) => check_msg_err(msg.reply(&ctx.http, why.to_string()).await),
    };

    Ok(())
}
