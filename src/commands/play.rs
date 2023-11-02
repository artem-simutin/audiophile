use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::*,
};

use crate::utils::check_msg_err;

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
