use async_recursion::async_recursion;
use lazy_static::lazy_static;
use poise::serenity_prelude::{self as serenity, CacheAndHttp};
use poise::serenity_prelude::{CacheHttp, ChannelId, GuildChannel};
use std::fs;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::utils::Root;
use crate::utils::{CAT_ICON, FLAC_ICON, MKV_ICON, MP4_ICON, OPUS_ICON};
// User data, which is stored and accessible in all command invocations
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// * global variable
lazy_static! {
    static ref GDRIVE_CHANNEL_ID: Mutex<u64> = Mutex::new(0);
}

async fn set_gdrive_channel_id(channel_id: u64) {
    println!("Channel Id set {}", channel_id);
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
    #[description = "Pass the channel id here, to set the notification channel"]
    channel: GuildChannel,
) -> Result<(), Error> {
    set_gdrive_channel_id(channel.id.0).await;

    ctx.say(format!(
        "Channel {} has been set for sending notifications",
        channel.clone()
    ))
    .await
    .expect("Error while trying to spawn the watcher");

    Ok(())
}

#[poise::command(slash_command)]
async fn spawn_watcher(ctx: Context<'_>) -> Result<(), Error> {
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
async fn send_changes_via_bot(ctx: Arc<CacheAndHttp>, mut rx: mpsc::Receiver<Root>) {
    let channel = ChannelId(*GDRIVE_CHANNEL_ID.lock().await);
    let change = rx.recv().await;

    for chg in change
        .expect("Failed to get info from the gdrive thread")
        .changes
    {
        if chg.get_file_name().is_some() {
            let logo = match chg.get_mime_type() {
                "audio/x-flac" => FLAC_ICON,
                "video/x-matroska" => MKV_ICON,
                "video/mp4" => MP4_ICON,
                "audio/ogg" => OPUS_ICON,
                _ => CAT_ICON,
            };

            channel
                .send_message(&ctx.http(), |m| {
                    m.embed(|e| {
                        e.title(chg.get_file_name().unwrap())
                            // .description("")
                            .field("File Type", chg.get_mime_type(), true)
                            .field("File Id", chg.get_file_id(), true)
                            .thumbnail(logo)
                    })
                })
                // .say(&ctx.http(), change.unwrap().kind)
                .await
                .expect("Error while sending the changes via discord API to the set channel");
        };
    }

    send_changes_via_bot(ctx, rx).await;
}

pub async fn bot(rx: mpsc::Receiver<Root>) {
    let discord_token = fs::read_to_string("discordtoken.txt")
        .expect("Canno't read the disccord token from the file");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![account_age(), set_gdrive_channel(), spawn_watcher()],
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
    let global_ctx = metacat.client().cache_and_http.clone();

    set_gdrive_channel_id(1073888120434602075).await;

    // * spawning a seperate tokio task so it doesn't block the parent thread.
    // * this allows bot to recieve commands in parallel and not get stuck checking for gdrive changes
    tokio::task::spawn(send_changes_via_bot(global_ctx, rx));

    metacat.start().await.expect("Failed to start framework");
}

//    framework.run().await.expect("Failed to start metacat");
