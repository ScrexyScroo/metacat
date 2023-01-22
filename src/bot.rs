use poise::serenity_prelude as serenity;
use std::fs;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn account_age(ctx: Context<'_>, user: Option<serenity::User>) -> Result<(), Error> {
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

pub async fn bot() {
    let discord_token = fs::read_to_string("discordtoken.txt").expect("Issue with token");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![account_age()], // Macro takes care of ctx and user
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
