use rand::random;
use serenity::http::CacheHttp;
use serenity::{client::Context, http::Http};
use serenity::model::channel::Message;
use std::sync::Arc;

use crate::util::debug::send_error;

use crate::commands::moove::{moove, moove_check};

pub async fn handle(ctx: Context, msg: Message) {
    if msg.author.bot {
        return
    }
    
    let lower_case_content = msg.content.to_lowercase();

    let bot_id = ctx.cache.current_user().id;
    if msg.mentions_user_id(bot_id) || lower_case_content.contains("moover") {
        if !response(ctx.http.clone(), msg.clone()).await {
            // NOTE maybe should exit here instead since there is something very wrong if I can't reply
            return
        }
    }
    else if lower_case_content.contains("henlo") {
        if !henlo(ctx.http.clone(), msg.clone()).await {
            // NOTE same as above
            return
        }
    }
    
    if random::<u16>() % 1000 == 666 {
        match msg.reply(ctx.http(), "Povedz loď").await {
            Ok(_) => {},
            Err(e) => send_error(ctx.http.clone(), e.to_string()).await
        }
    }

    let channel_id = match moove_check(&msg).await {
        Some(val) => val,
        None => return
    };

    match moove(ctx.http, msg.clone(), channel_id).await {
        Ok(_) => return,
        Err(e) => println!("ERROR: {e}")
    };
}

async fn response(http: Arc<Http>, msg: Message) -> bool {
    // NOTE probably not clever to do it this way  
    const RESPONSES: [&str; 4] = [
        "To som jaaa",
        "Henloooo",
        "No čo je?",
        "Hm?"
    ];

    let num = random::<usize>() % RESPONSES.len();
    match msg.reply(http.clone(), RESPONSES[num]).await {
        Ok(_) => { return true }
        Err(e) => {
            send_error(http, e.to_string()).await;
            return false
        }
    };
}

async fn henlo(http: Arc<Http>, msg: Message) -> bool {
    const EMOJIS: [&str; 7] = ["🥰", "🐄", "🐮", "❤️", "👋", "🤠", "😊"];

    let num = random::<usize>() % EMOJIS.len();
    let response = format!("Henlooo {} {}", msg.author.name, EMOJIS[num]);

    
    match msg.reply(http.clone(), response).await {
        Ok(_) => { return true }
        Err(e) => {
            send_error(http, e.to_string()).await;
            return false
        }
    };
}