use async_recursion::async_recursion;
use lazy_static::lazy_static;
use poise::serenity_prelude::{self as serenity, CacheAndHttp};
use serde_json::Value;
use std::fs;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

// User data, which is stored and accessible in all command invocations
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// * global variable
lazy_static! {
    static ref GDRIVE_CHANNEL_ID: Mutex<u128> = Mutex::new(0);
}

async fn set_gdrive_channel_id(channel_id: u128) {
    *GDRIVE_CHANNEL_ID.lock().await = channel_id;
}

#[poise::command(slash_command)]
async fn account_age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let discord_user = user.as_ref().unwrap_or_else(|| ctx.author());
    let res = format!(
        "{} stopped having a life since {}",
        discord_user.name,
        discord_user
            .created_at()
            .format("%b %e, %Y and it was a %A")
    );
    ctx.say(res).await.expect("Error in account_age()");
    Ok(())
}

#[poise::command(slash_command)]
async fn set_gdrive_channel(
    ctx: Context<'_>,
    #[description = "Pass the channel id here, to set the notification channel"] channel: u128,
) -> Result<(), Error> {
    set_gdrive_channel_id(channel.clone());
    ctx.say(format!(
        "Channel {} has been set for sending notifications",
        channel
    ))
    .await
    .expect("Error while trying to spawn the watcher");

    Ok(())
}

#[poise::command(slash_command)]
async fn spawn_watcher(ctx: Context<'_>) -> Result<(), Error> {
    // /set_gdrive_channel_id(1061996380865953792);
    ctx.say(format!(
        "Will send updates in the channel: {}",
        GDRIVE_CHANNEL_ID.lock().await
    ))
    .await
    .expect("Error while trying to spawn the watcher");

    Ok(())
}

// * Used recursion to keep listening to rx, instead of spawing a tokio task
#[async_recursion]
async fn send_changes_via_bot(ctx: &Arc<CacheAndHttp>, mut rx: mpsc::Receiver<Value>) {
    // loop {
    //     println!("{:?}", rx.recv().await);
    // }
    println!("From the bot thread {:?}", rx.recv().await);
    // ctx.say(rx.recv().await.unwrap().to_string());
    send_changes_via_bot(ctx, rx);
}

pub async fn bot(rx: mpsc::Receiver<Value>) {
    let discord_token = fs::read_to_string("discordtoken.txt")
        .expect("Canno't read the disccord token from the file");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![account_age(), set_gdrive_channel(), spawn_watcher()], // Macro takes care of ctx and user
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    let metacat = framework.build().await.expect("Failed to init metacat");

    metacat
        .client()
        .start()
        .await
        .expect("Failed to start metacat");

    let global_ctx = &metacat.client().cache_and_http;

    send_changes_via_bot(global_ctx, rx).await; // ! some kind of Send requried issue here
}
//    framework.run().await.expect("Failed to start metacat");
