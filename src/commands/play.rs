use audiophile::check_message_send_status;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::{MembershipState, Message},
    utils::Color,
};
use url::Url;

use crate::{
    server::{context::ServerContext, ServerContexts},
    song::Song,
};

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
    if let Err(_) = Url::parse(&value) {
        check_message_send_status(msg.reply(ctx, "Invalid url!").await);
        return Ok(());
    };
    let song = match Song::new_by_yt_url(&value).await {
        Ok(s) => s,
        Err(why) => {
            check_message_send_status(msg.reply(ctx, why.to_string()).await);
            return Ok(());
        }
    };

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

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_message_send_status(msg.reply(ctx, "Not in a voice channel").await);
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    let mut data = ctx.data.write().await;
    let server_queries = data
        .get_mut::<ServerContexts>()
        .expect("The server queries must be presented in the client data!");
    let current_query = server_queries
        .entry(connect_to)
        .or_insert(ServerContext::new());

    current_query.add_song(song.clone());

    let author_nickname = match msg.author_nick(&ctx.http).await {
        Some(n) => n,
        None => msg.author.name.clone(),
    };

    check_message_send_status(
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(format!("From {}", author_nickname))
                            .icon_url(&msg.author.avatar_url().get_or_insert("".to_string()))
                    })
                    .color(Color::ORANGE)
                    .url(&song.url)
                    .title(&song.title)
                })
            })
            .await,
    );

    if let Ok(_channel_connected) = success {
        let mut handle = handle_lock.lock().await;

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
        let player = handle.play_source(source.into());
    } else {
        check_message_send_status(
            msg.channel_id
                .say(&ctx.http, "Something went wrong joining to the channel!")
                .await,
        );
    }

    Ok(())
}
