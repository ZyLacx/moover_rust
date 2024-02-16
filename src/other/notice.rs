use chrono::{Datelike, Local};

use serenity::{all::{GuildId, UserId}, builder::{CreateEmbed, CreateMessage}, client::Context, http::Http, model::Colour};

use anyhow::Ok;

use sqlx::{Connection, FromRow, SqliteConnection};

use crate::util::security::dotenv_var;
use crate::util::debug::send_error;
use crate::util::utilities;

use std::sync::Arc;

// pub async fn notice_wrapper(http: Arc<Http>) {
pub async fn notice_wrapper(ctx: Context) {
    match notice(ctx.http.clone()).await {
        Err(e) => {
            send_error(ctx.http.clone(), e.to_string()).await;
            return;
        },
        Result::Ok(_) => return
    }
}

async fn announce_event(guild_id: GuildId, name: &str, special_message: &str, http: Arc<Http>) -> anyhow::Result<()> {
    let mut event_embed = CreateEmbed::new()
        .color(Colour::new(rand::random::<u32>() % 0xFFFFFF))
        .title("Today's event is:");

    let system_channel = utilities::get_system_channel(guild_id, http.clone()).await?;
    
    if special_message.contains("http") {
        event_embed = event_embed.description(name);
        system_channel.send_message(http.clone(),
             CreateMessage::new().add_embed(event_embed.clone()).content(special_message)).await?;
        // Ok(());
    }

    event_embed = event_embed.field(name, special_message, true);
    system_channel.send_message(http.clone(),
        CreateMessage::new().add_embed(event_embed)).await?;

    Ok(())
}

async fn celebrate_birthday(guild_id: GuildId, user_id: UserId, nick: &str, http: Arc<Http>) -> anyhow::Result<()> {
    let system_channel = utilities::get_system_channel(guild_id, http.clone()).await?;

    let embed = CreateEmbed::new()
        .color(Colour::new(rand::random::<u32>() % 0xFFFFFF))
        .title(format!("HAPPY BIRTHDAY {}!", nick))
        .description(format!("<@{}>'s birthday is today!!! Yay!", user_id.get()));

    system_channel.send_message(http.clone(), CreateMessage::new().add_embed(embed.clone())).await?;

    Ok(())
}

#[derive(Clone, FromRow, Debug)]
struct BirthdayRow {
    #[sqlx(try_from="i64")]
    id: u64,
    nick: String,
}

#[derive(Clone, FromRow, Debug)]
struct EventRow {
    id: u32,
    #[sqlx(try_from="i64")]
    guild: u64,
    name: String,
    year: i32,
    special_message: String,
}

async fn notice(http: Arc<Http>) -> anyhow::Result<()> {
    use anyhow::Context;

    let local = Local::now();
    let day = local.day();
    let month = local.month();
    let year = local.year();

    let db_path = dotenv_var("DATABASE_URL").context("DATABASE_URL not found in env")?;

    let mut db = SqliteConnection::connect(db_path.as_str()).await?;

    let birtdays = sqlx::query_as::<_, BirthdayRow>(
        "SELECT id, nick FROM birthdays
        WHERE day=? AND month=?;"
    )
    .bind(day)
    .bind(month)
    .fetch_all(&mut db)
    .await?;

    let global_events = sqlx::query_as::<_, EventRow>(
        "SELECT id, guild, name, year, special_message from events
        WHERE day=? AND month=? AND guild=0;"
    )
    .bind(day)
    .bind(month)
    .fetch_all(&mut db)
    .await?;

    let guilds = http.get_guilds(None, None).await?;

    for guild in guilds {
        let guild_id = guild.id;

        for bd in &birtdays {
            let user_id = UserId::new(bd.id);
            guild_id.member(http.clone(), user_id).await?;

            celebrate_birthday(guild_id, user_id, bd.nick.as_str(), http.clone()).await?;
        }

        // TODO if has year delete it from announce and delete

        for e in &global_events {
            if e.year != 0 && e.year != year {
                continue;
            }

            announce_event(guild_id, e.name.as_str(), e.special_message.as_str(), http.clone()).await?;
        }
    }

    let global_events = sqlx::query_as::<_, EventRow>(
        "SELECT id, guild, name, year, special_message from events
        WHERE day=? AND month=? AND guild!=0;"
    )
    .bind(day)
    .bind(month)
    .fetch_all(&mut db)
    .await?;

    // TODO if has year delete it from announce and delete

    for e in &global_events {
        announce_event(GuildId::new(e.guild), e.name.as_str(), e.special_message.as_str(), http.clone()).await?;
    }

    Ok(())
}

