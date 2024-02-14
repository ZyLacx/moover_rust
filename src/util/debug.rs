use serenity::{all::ChannelId, builder::CreateMessage, http::Http};
use std::{process::exit, sync::Arc};

pub async fn send_error(http: Arc<Http>, msg: String) {
    println!("ERROR: {msg}");

    #[cfg(feature="RELEASE")]
    match ChannelId::new(1199495008416440491)
        .send_message(http, CreateMessage::new().content(msg)).await {
            Ok(_) => { return; }
            Err(_) => { exit(-1) }
        };
}

pub async fn hello(http: Arc<Http>) {
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

    let num = rand::random::<usize>() % messages.len();
    
    let channel = ChannelId::new(780439236867653635);
    if let Err(why) = channel.say(http, messages[num]).await {
        print!("Error sending message: {:?}", why);
    };
}