use serenity::client::Context;
use serenity::model::channel::Message;

use crate::commands::moove::moove;

pub async fn handle(ctx: Context, msg: Message) {
    println!("In handler");
    match moove(ctx.http, msg).await {
        Ok(_) => return,
        Err(e) => println!("ERROR: {e}")
    };
}