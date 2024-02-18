use serenity::async_trait;
use serenity::prelude::GatewayIntents;
use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::all::{Message, EventHandler};
use serenity::Client;

use tokio_cron_scheduler::{JobScheduler, Job};
use util::security::dotenv_var;

mod message_handler;
use message_handler::handle;

use std::future::Future;
use std::pin::Pin;

mod commands;
mod util;

mod other;
use other::notice;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        handle(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} v0.2 is connected!", ready.user.name);

        #[cfg(feature="RELEASE")] {
            use util::debug::hello;
            hello(ctx.http.clone()).await;
        }

        let sched = JobScheduler::new().await.unwrap();

        let job_closure = move |_, _| -> Pin<Box<dyn Future<Output = ()> + Send>> {
            let ctx_clone = ctx.clone();
            Box::pin( async move {
                notice::notice_wrapper(ctx_clone).await;
            })
        };

        sched.add(Job::new_async("0 0 13 * * *", job_closure).expect("Cron job not set up correctly")).await.unwrap();
        sched.start().await.unwrap();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anyhow::Context;

    let token_str = "TOKEN";
    
    #[cfg(feature="DEBUG")]
    let token_str = "DEBUGTOKEN";

    let token = dotenv_var(token_str).context("TOKEN not found in env")?;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .context("Failed to build client")?;

    client.start().await?;
    Ok(())
}
