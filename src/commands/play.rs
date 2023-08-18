use audiophile::server_queries::{ServerQuery, ServersQueries};
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Mentionable,
};

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let song_url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.reply(ctx, "Please provide YouTube url as first argument")
                .await
                .unwrap();
            return Ok(());
        }
    };

    if !song_url.starts_with("http") {
        msg.reply(ctx, "Provide valid audio resourse url!")
            .await
            .unwrap();
        return Ok(());
    };

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

    let mut data = ctx.data.write().await;
    let server_queries = data
        .get_mut::<ServersQueries>()
        .expect("The server queries must be presented in the client data!");
    let current_query = server_queries
        .entry(connect_to)
        .or_insert(ServerQuery::new());

    current_query.add_song(&song_url);

    msg.reply(ctx, format!("Song {} is added to the queue!", &song_url))
        .await
        .unwrap();

    if let Ok(_channel_connected) = success {
        msg.channel_id
            .say(&ctx.http, format!("Joined {}", connect_to.mention()))
            .await
            .unwrap();
        let mut handle = handle_lock.lock().await;
    } else {
        msg.channel_id
            .say(&ctx.http, "Something went wrong joining to the channel!")
            .await
            .unwrap();
    }

    Ok(())
}
