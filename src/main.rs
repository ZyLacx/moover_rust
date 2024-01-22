use rand::random;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use util::security::dotenv_var;
use other::msg::hello;

mod message_handler;
use message_handler::handle;

mod commands;
mod util;
mod other;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        handle(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let debug = match dotenv_var("DEBUG") {
            Some(v) => v,
            None => "OFF".to_string(),
        };
        if debug != "ON" {
            let messages = [
                "AAAAAAAAAAAAAAAAAAAA",
                "Henlooo",
                "Good day y'all!",
                "May have crashed...",
                "MOOOooo",
                "Heyyyyy!",
                "I'm baaaaack!",
                "Whom'st have summoned the ancient one?",
            ];

            let rand_num = random::<usize>() % messages.len();
            let channel = ctx.http.get_channel(780439236867653635).await.unwrap().id();
            match channel.say(&ctx.http, messages[rand_num]).await {
                Err(e) => println!("Something went wrong: {e}"),
                Ok(_) => return,
            };
        }
        // if ready.user.name != "MOOver Debug" {
            hello(ctx.http).await;
        // }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    let token = dotenv_var("TOKEN").context("No TOKEN in env")?;
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .context("Failed to build client")?;

    client.start().await?;
    Ok(())
}
