use std::sync::atomic::AtomicUsize;

use chrono::Utc;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::client::Context;

// use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};
use tokio_schedule::{every, EveryDay, Job};
use util::security::dotenv_var;

use std::sync::Arc;

mod message_handler;
use message_handler::handle;

mod commands;
mod util;
use util::debug::send_error;

mod other;
use other::notice;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        handle(ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        #[cfg(feature="RELEASE")] {
            use util::debug::hello;
            hello(ctx.http.clone()).await;
        }

        // notice::notice_wrapper(ctx).await;

        // let scheduler = every(1).day().at(13, 30, 0)
        //     .perform(|| async {
        //         notice::notice_wrapper(ctx.clone()).await
        //     }).await;

        // let mut scheduler = JobScheduler::new().await;
        // scheduler.
        // scheduler.add(match Job::new_async("5 * * * * * *", |uuid, mut l| Box::pin( async {
        //     notice::notice(ctx.clone()).await;
        // })) {
        //     Ok(_) => {}
        //     Err(e) => {
        //         send_error(ctx.http.clone(), e.to_string());
        //         panic!()
        //     }
        // });
        // scheduler.add(Job::new(daily("22"), move || {
        //     notice::notice(ctx.clone())
        // }));
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
