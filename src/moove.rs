use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::channel::Message;
use serenity::http::Http;
use serenity::model::channel::Attachment;
use anyhow;
use serenity::model::mention;

// use regex::Regex;

// Checks if the message should be mooved, if not returns Ok(0)
// If the message should be mooved, try to move it and return Ok(1) if mooved succesfully
// else returns Err()
pub async fn moove(http: Http, msg: Message) -> Result<u8, String>{
    let channel_mentions = msg.mention_channels;
    let words = msg.content.trim().split_whitespace().count();
    
    // let re = Regex::new(r"<#[0-9]*>$").unwrap();
    // if re.captures(content)

    if channel_mentions.len() != 1 || words != 1 {
        return Ok(0);
    }

    let mut msg_to_moove = match msg.referenced_message {
        Some(m) => m,
        None => return Ok(0)
    };

    let mentioned_channel = channel_mentions[0].id;

    let attachments = msg_to_moove.attachments;
    let embeds = msg_to_moove.embeds;

    let sent_by = format!("Sent by {}\n mooved {}\n", msg_to_moove.author, msg.author);

    let mut embeds_copy : Vec<CreateEmbed> = Vec::new();
    for embed in msg_to_moove.embeds {
        embeds_copy.push(CreateEmbed::from(embed));
    }

    // let mut attachment_links : Vec<String> = Vec::new();
    let attachment_link = msg_to_moove.attachments.pop();
    // for attachment in msg_to_moove.attachments {
    //     attachment_links.push(attachment.url);
    // }

    // if embeds_copy.len() > 0 || attachment_links.len() > 0 {
    if embeds_copy.len() > 0 {
        let mut new_content = "".to_string();
        if !msg_to_moove.content.is_empty() {
            new_content = format!("Message:\n{}", msg_to_moove.content);
        }
        sent_by.push_str(&new_content);
        match mentioned_channel.send_message(http, |m| {
            m.content(sent_by).add_embeds(embeds_copy).add_file(attachment_link)
        }).await {
            Ok(_) => return Ok(0),
            Err(e) => return Err(e.to_string())
        };
    }
    else if !msg_to_moove.content.is_empty() {
        match mentioned_channel.send_message(http, |m| {
            m.add_embed(|e| {
                e.title("MOO").field(sent_by, " ", false).field("Message:\n", msg_to_moove.content, false)
            })
        }).await {
            Ok(_) => return Ok(0),
            Err(e) => return Err(e.to_string())
        };
    }

    Err("Something went wrong while mooving the message".to_string())
}