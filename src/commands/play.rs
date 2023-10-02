use audiophile::check_message_send_status;
use serenity::{
    async_trait,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
};
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use url::Url;

use crate::{config::Config, song::Song};

struct TrackEndEventHandler {
    command_ctx: Context,
    msg: Message,
}

#[async_trait]
impl VoiceEventHandler for TrackEndEventHandler {
    async fn act(&self, _: &EventContext<'_>) -> Option<Event> {
        let songbird = songbird::get(&self.command_ctx)
            .await
            .expect("Songbird initialization")
            .clone();

        let guild_id = match self.msg.guild(&self.command_ctx.cache) {
            Some(v) => v,
            None => {
                println!("Something went wrong getting guild id from the message in `TrackEndEventHandler`");
                return None;
            }
        };

        let manager = songbird.get(guild_id.id);

        if let Some(locker) = manager {
            let handler = locker.lock().await;
        }

        None
    }
}

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Try to get some provided value
    let value = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_message_send_status(
                msg.reply(ctx, "Please provide YouTube url as first argument")
                    .await,
            );
            return Ok(());
        }
    };

    // Try to parse provided value as URL
    if let Err(_) = Url::parse(&value) {
        check_message_send_status(msg.reply(ctx, "Invalid url!").await);
        return Ok(());
    };

    // Create a song from the value
    // TODO: Implement auth for users for YT
    let song = match Song::new_by_yt_url(&value).await {
        Ok(s) => s,
        Err(why) => {
            check_message_send_status(msg.reply(ctx, why.to_string()).await);
            return Ok(());
        }
    };

    // Get guild from the sent message
    let guild = match msg.guild(&ctx.cache) {
        Some(g) => g,
        None => {
            check_message_send_status(
                msg.reply(
                    ctx,
                    "Something went wrong extracting guild from the message!",
                )
                .await,
            );
            return Ok(());
        }
    };

    let guild_id = guild.id;

    // Try to get voice channel from the message
    let voice_channel_id = match guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id)
    {
        Some(channel) => channel,
        None => {
            check_message_send_status(msg.reply(ctx, "Not in a voice channel").await);
            return Ok(());
        }
    };

    // Get songbird manager client
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    // Try to connect to the voice channel
    let (handle_lock, success) = manager.join(guild_id, voice_channel_id).await;

    // Get the message author server nickname or get the discord global name
    let author_nickname = match msg.author_nick(&ctx.http).await {
        Some(n) => n,
        None => msg.author.name.clone(),
    };

    let config = Config::new();

    // if current_query.songs.len() == 1 {
    //     check_message_send_status(
    //         msg.channel_id
    //             .send_message(&ctx.http, |m| {
    //                 m.embed(|e| {
    //                     e.author(|a| {
    //                         a.name(format!("From {}", &author_nickname))
    //                             .icon_url(&msg.author.avatar_url().get_or_insert("".to_string()))
    //                     })
    //                     .title("Started playing this song right away!")
    //                     .color(Color::GOLD)
    //                     .image(&song.thumbnail_url.unwrap_or_default())
    //                     .field(":timer: Song duration", &song.length.as_secs(), true)
    //                     .field(":calendar_spiral: Publish date", &song.upload_date, true)
    //                     .field(":thumbsup::skin-tone-2: Likes", &song.likes_count, true)
    //                     .footer(|f| {
    //                         f.text(format!("Powered by {}", &config.name))
    //                             .icon_url(&config.avatar_url)
    //                     })
    //                 })
    //             })
    //             .await,
    //     )
    // } else {
    //     check_message_send_status(
    //         msg.channel_id
    //             .send_message(&ctx.http, |m| {
    //                 m.embed(|e| {
    //                     e.author(|a| {
    //                         a.name(format!("From {}", author_nickname))
    //                             .icon_url(&msg.author.avatar_url().get_or_insert("".to_string()))
    //                     })
    //                     .field(":timer: Song duration", &song.length.as_secs(), true)
    //                     .field(
    //                         "Queue duration",
    //                         current_query
    //                             .songs
    //                             .iter()
    //                             .map(|i| i.length.as_secs())
    //                             .reduce(|acc, i| acc + i)
    //                             .unwrap_or_default(),
    //                         true,
    //                     )
    //                     .field("Songs in queue", current_query.songs.len(), true)
    //                     .color(Color::ORANGE)
    //                     .url(&song.url)
    //                     .description("Song is added to the queue!")
    //                     .title(&song.title)
    //                     .footer(|f| {
    //                         f.text(format!("Powered by {}", &config.name))
    //                             .icon_url(&config.avatar_url)
    //                     })
    //                 })
    //             })
    //             .await,
    //     );
    // }

    // If error occured while connecting to the voice channel,
    // finish the function and send an error message
    if let Err(why) = success {
        check_message_send_status(
            msg.channel_id
                .say(&ctx.http, "Something went wrong joining to the channel!")
                .await,
        );
        return Ok(());
    }

    // Take control of the voice channel
    let mut handle = handle_lock.lock().await;

    // Create buffer from the song's url by `yt-dlp`
    let source = match songbird::ytdl(song.url).await {
        Ok(s) => s,
        Err(why) => {
            check_message_send_status(
                msg.channel_id
                    .say(
                        ctx,
                        format!("Something went wrong fetching the song!\n{:?}", why),
                    )
                    .await,
            );
            return Ok(());
        }
    };

    // Start playing the song through `songbird` player
    let player = handle.play_source(source.into());

    // Add handler on track `END` event
    handle.add_global_event(
        Event::Track(TrackEvent::End),
        TrackEndEventHandler {
            command_ctx: ctx.clone(),
            msg: msg.clone(),
        },
    );

    Ok(())
}
