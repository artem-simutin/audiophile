use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::{Context, Mentionable},
};

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "Not in a voice channel").await.unwrap();
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        msg.channel_id
            .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
            .await
            .unwrap();

        let chan_id = msg.channel_id;

        let send_http = ctx.http.clone();

        let mut handle = handle_lock.lock().await;

        // handle.add_global_event(
        //     Event::Track(TrackEvent::End),
        //     TrackEndNotifier {
        //         chan_id,
        //         http: send_http,
        //     },
        // );

        // let send_http = ctx.http.clone();

        // handle.add_global_event(
        //     Event::Periodic(Duration::from_secs(60), None),
        //     ChannelDurationNotifier {
        //         chan_id,
        //         count: Default::default(),
        //         http: send_http,
        //     },
        // );
    } else {
        msg.channel_id
            .say(&ctx.http, "Error joining the channel")
            .await
            .unwrap();
    }

    Ok(())
}
