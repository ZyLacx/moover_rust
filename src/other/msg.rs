use serenity::http::Http;
use std::sync::Arc;

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

    let channel = http.get_channel(780439236867653635).await.unwrap();

    let num = rand::random::<usize>() % messages.len();

    if let Err(why) = channel.id().say(http, messages[num]).await {
        print!("Error sending message: {:?}", why);
    };
}