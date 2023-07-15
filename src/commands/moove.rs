use std::sync::Arc;

use anyhow::{self, Context};
use poise::serenity_prelude::AttachmentType;
use serenity::builder::CreateEmbed;
use serenity::http::Http;
use serenity::model::channel::Message;
use url::Url;
use regex::Regex;

// Checks if the message should be mooved
// If the message should be mooved, try to move it and return Ok if mooved succesfully
// else returns Err()

pub enum MooveResult {
    Mooved,
    NotMooveRequest
}

pub async fn moove(http: Arc<Http>, msg: Message) -> anyhow::Result<MooveResult> {
    let word_count = msg.content.trim().split_whitespace().count();

    let re = Regex::new(r"<#[0-9]*>$").unwrap();

    if word_count != 1 || re.captures(&msg.content).is_none() {
        return Ok(MooveResult::NotMooveRequest);
    }

    let msg_to_moove = msg.referenced_message.context("no message present")?;
    
    let mentioned_channel = http.get_channel(
                                        msg.content[2..msg.content.len() - 1].parse::<u64>()
                                        .unwrap()).await?.id();

    //steals all attachments, but sets all of them as Image urls, so rip actual docs etc
    let attachments = msg_to_moove
        .attachments
        .into_iter()
        .map(|att| AttachmentType::Image(Url::parse(att.url.as_str()).unwrap()));

    //steals all the embeds
    let embeds: Vec<CreateEmbed> = msg_to_moove
        .embeds
        .into_iter()
        .map(|em| CreateEmbed::from(em))
        .collect();

    let mut new_content = format!("Sent by {}\n mooved {}\n", msg_to_moove.author, msg.author);

    if !msg_to_moove.content.is_empty() {
        mentioned_channel.send_message(http, |m| {
            m.add_embed(|e| {
                e.field("MOO", new_content, false)
                .field("Message:\n", msg_to_moove.content.clone(), false)  
            })
        }).await?;
    }
    else if attachments.len() > 0 || embeds.len() > 0 {
        new_content += format!("Message:\n{}", msg_to_moove.content).as_str();
        mentioned_channel.send_message(http, |m| {
            m.content(new_content)
                .add_embeds(embeds)
                .add_files(attachments)
        }).await?;
    }
    Ok(MooveResult::Mooved)
}
