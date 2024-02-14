use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateMessage};
use serenity::http::Http;
use serenity::model::channel::Message;
use tokio::time::sleep;
use regex::Regex;
use serenity::model::id::ChannelId;

// Checks if the message should be mooved
// If the message should be mooved, try to move it and return Ok if mooved succesfully
// else returns Err()

pub async fn moove_check(msg: &Message) -> Option<u64> {
    let word_count = msg.content.trim().split_whitespace().count();
    let re = Regex::new(r"<#[0-9]*>$").unwrap();

    if word_count != 1 || re.captures(&msg.content).is_none() {
        return None
    }

    let channel_id = match msg.content[2..msg.content.len() - 1].parse::<u64>() {
        Ok(val) => val,
        Err(_) => return None
    };

    return Some(channel_id);
}

pub async fn moove(http: Arc<Http>, msg: Message, m_channel_id: u64) -> anyhow::Result<()> {
    // this should be in moove_check, but I need to find a good way to return in with channel_id
    let msg_to_moove = msg.clone().referenced_message.context("Referenced message not found")?;

    //steals all attachments, but sets all of them as Image urls, so rip actual docs etc
    let mut attachments: Vec<CreateAttachment> = Vec::new();
    for attachment in msg_to_moove.attachments.clone() {
        let data = attachment.download().await?;
        attachments.push(CreateAttachment::bytes(data, attachment.filename));
    }

    //steals all the embeds
    let embeds: Vec<CreateEmbed> = msg_to_moove
        .embeds.clone()
        .into_iter()
        .map(| embed | CreateEmbed::from(embed))
        .collect();

    
    let mut new_content = format!("Sent by {}\n mooved {}\n", msg_to_moove.author, msg.author);
    let mut new_msg = CreateMessage::new();

    // Either copy all the attachments and embeds and create a new message that contains them all
    if attachments.len() > 0 || embeds.len() > 0 {
        new_content += format!("Message:\n{}", msg_to_moove.content).as_str();

        new_msg = new_msg.content(new_content)
            .add_embeds(embeds)
            .add_files(attachments);
    }
    // or create a new embed with the content of the mooved message as one of the fields
    else {
        let embed = CreateEmbed::new()
        .field("MOO", new_content, false)
        .field("Message:\n", msg_to_moove.content.clone(), false);
    
        new_msg = new_msg.add_embed(embed);
    }

    ChannelId::new(m_channel_id).send_message(http.clone(), new_msg).await?;

    sleep(Duration::from_secs(2)).await;

    msg_to_moove.delete(http.clone()).await?;
    msg.delete(http).await?;
    Ok(())
}
