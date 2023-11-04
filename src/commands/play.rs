use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::*,
};
use youtube::YoutubeAPI;

use crate::{
    structs::{
        queue::{ContextData},
        song::Song,
    },
    utils::check_msg_err,
};

#[command]
#[aliases(p)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let url = match youtube::url::Url::parse(&possible_song_url) {
        Ok(url) => url,
        Err(_) => {
            check_msg_err(msg.reply(&ctx.http, "Provide a valid URL").await);
            return Ok(());
        }
    };

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

    let voice_channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id);

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

    let voice_manager = songbird::get(ctx)
        .await
        .expect("Songbird placed in at initialization")
        .clone();

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

    {
        let mut data = ctx.data.write().await;
        let data = data.get_mut::<ContextData>();
        let data = match data {
            Some(d) => d,
            None => {
                check_msg_err(
                    msg.reply(
                        &ctx.http,
                        "Something went wrong adding song to the playlist! Tell to my creator that he is a bimbo 💀!",
                    )
                    .await,
                );
                return Ok(());
            }
        };
        let server_context = data.entry(guild.id).or_default();
        let voice_channel_context = server_context
            .voice_queues
            .entry(voice_channel_id)
            .or_default();

        voice_channel_context.songs.push_back(song);
        println!("Queue: {:?}", voice_channel_context.songs);
    }

    let (voice_handler, join_status) = voice_manager.join(guild.id, voice_channel_id).await;

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

    let mut voice_handler = voice_handler.lock().await;

    let song_source = match songbird::ytdl(url).await {
        Ok(s) => s,
        Err(why) => {
            check_msg_err(
                msg.reply(
                    &ctx.http,
                    format!("Somethign went wrong creating source from URL: {}", why),
                )
                .await,
            );
            return Ok(());
        }
    };

    voice_handler.play_source(song_source);

    Ok(())
}
