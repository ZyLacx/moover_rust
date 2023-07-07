use std::sync::Arc;

use poise::serenity_prelude::GuildChannel;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use util::security::dotenv_var;
use serenity::http::{self, Http};

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
        let messages = ["AAAAAAAAAAAAAAAAAAAA", "Henlooo", "Good day y'all!",
                        "May have crashed...", "MOOOooo", "Heyyyyy!", "I'm baaaaack!",
                        "Whom'st have summoned the ancient one?"];
        
        // }
        let opt_token = dotenv_var("TOKEN");
        if opt_token.is_none() {
            return;
        }

        let token = opt_token.unwrap();
        let channel_result = ctx.http.get_channel(780439236867653635).await;
        let channel = channel_result.unwrap();
        // let channel = await http.get_channel(780439236867653635);
        // GuildChannel::say(&self, http, content)
        // self.message(ctx, new_message)
        // C = ;
        
        // const debug_channel = 
    }
}



#[tokio::main]
async fn main() {
    let opt_token = dotenv_var("TOKEN");
    if opt_token.is_none() {
        return;
    }

    let token = opt_token.unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(&token, intents)
                                .event_handler(Handler)
                                .await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
