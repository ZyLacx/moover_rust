use std::sync::Arc;

use serenity::http::Http;

pub async fn send_error(_http: Arc<Http>, msg: String) {
    println!("ERROR: {msg}");
    
    #[cfg(feature="RELEASE")] {
        use serenity::all::ChannelId;
        use serenity::all::CreateMessage;
        use std::process::exit;

        match ChannelId::new(1199495008416440491)
            .send_message(_http, CreateMessage::new().content(msg)).await {
                Ok(_) => { return; }
                Err(_) => { exit(-1) }
        };
    }
}
    
#[cfg(feature="RELEASE")]
pub async fn hello(http: Arc<Http>) {
    use serenity::all::ChannelId;

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