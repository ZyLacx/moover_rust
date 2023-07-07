use std::sync::Arc;

use poise::serenity_prelude::GuildChannel;
use serenity::async_trait;
use serenity::http::{self, Http};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use util::security::dotenv_var;

mod util;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // if (ready.user.name != "MOOver Debug") {
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

        // }
        let channel_result = ctx.http.get_channel(780439236867653635).await;
        let channel = channel_result.unwrap();
        // let channel = await http.get_channel(780439236867653635);
        // GuildChannel::say(&self, http, content)
        // self.message(ctx, new_message)
        // C = ;

        // const debug_channel =
    }
}
struct MyStruct {
    niečo: String,
}
impl MyStruct {
    fn add(&mut self) {
        self.niečo.push(char::from_digit(2, 2).unwrap());
    }
}
trait Countable {
    fn count(&self) -> usize;
}
impl Countable for MyStruct {
    fn count(&self) -> usize {
        self.niečo.len()
    }
}
fn smt(var: Box<dyn Countable>) {
    var.count();
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut n = MyStruct {
        niečo: "aaa".to_string(),
    };
    n.add();
    loop {
        //Keeps trying to reconnect, if errors occur print to console and retry
        match connect().await {
            Ok(r) => return Ok(()),
            Err(e) => println!("FAILED TO CONNECT!!! {e}\nRetrying soon..."),
        }
    }
}

async fn connect() -> anyhow::Result<()> {
    use anyhow::Context;
    let token = dotenv_var("TOKEN").context("No TOKEN in env")?;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .context("Failed to build client")?;

    client.start().await?;
    Ok(())
}
