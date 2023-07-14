use std::sync::Arc;

use anyhow::{self, Context};
use poise::serenity_prelude::AttachmentType;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::http::Http;
use serenity::model::channel::Attachment;
use serenity::model::channel::Message;
use serenity::model::mention;
use url::Url;

// use regex::Regex;

// Checks if the message should be mooved, if not returns Ok(0)
// If the message should be mooved, try to move it and return Ok(1) if mooved succesfully
// else returns Err()

pub async fn moove(http: Arc<Http>, msg: Message) -> anyhow::Result<()> {
    let channel_mentions = msg.mention_channels;
    let words = msg.content.trim().split_whitespace().count();
    // let re = Regex::new(r"<#[0-9]*>$").unwrap();
    // if re.captures(content)

    if channel_mentions.len() != 1 || words != 1 || msg.content.is_empty() {
        anyhow::bail!("no message worth processing");
    }

    let msg_to_moove = msg.referenced_message.context("no message present")?;

    let mentioned_channel = channel_mentions[0].id;

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
    new_content += format!("Message:\n{}", msg_to_moove.content).as_str();

    mentioned_channel
        .send_message(http, |m| {
            m.content(new_content)
                .add_embeds(embeds)
                .add_files(attachments)
        })
        .await?;
    Ok(())
}
