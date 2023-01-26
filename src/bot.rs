use lazy_static::lazy_static;
use poise::serenity_prelude::{self as serenity};
// use serenity::model::id::ChannelId;
use std::fs;
use std::sync::Mutex;

// * global variable CHANNEL_ID
lazy_static! {
    // static ref CHANNEL_ID: Mutex<String> = Mutex::new("".to_string());
    static ref GDRIVE_CHANNEL_ID: Mutex<u64> = Mutex::new(0);
}

fn set_gdrive_channel_id(channel_id: u64) {
    *GDRIVE_CHANNEL_ID
        .lock()
        .expect("Acquiring lock failed while setting channel_id") = channel_id;
}

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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

// ! doens't actually set the channel for now
#[poise::command(slash_command)]
async fn set_gdrive_channel(
    ctx: Context<'_>,
    #[description = "Pass the channel id here, to set the notification channel"] channel: u64,
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
    !todo!();

    ctx.say(format!(
        "Will send updates in the channel: {}",
        GDRIVE_CHANNEL_ID
            .lock()
            .expect("Failed to acquire lock on global GDRIVE_CHANNEL_ID")
    ))
    .await
    .expect("Error while trying to spawn the watcher");

    Ok(())
}

pub async fn bot() {
    let discord_token = fs::read_to_string("discordtoken.txt").expect("Issue with token");
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

    framework.run().await.unwrap();
}
