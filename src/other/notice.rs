use chrono::{Datelike, Local};
use serenity::{all::{GuildId, UserId}, http::Http, prelude::*};

use crate::util::debug::send_error;

use anyhow::Ok;

use std::sync::Arc;

use crate::util::security::dotenv_var;

use sqlx::{FromRow, Connection, SqliteConnection};

pub async fn notice_wrapper(ctx: Context) {
    match notice(ctx.clone()).await {
        Err(e) => {
            send_error(ctx.http.clone(), e.to_string()).await;
            return;
        },
        Result::Ok(_) => return
    }
}

async fn announce_event(guild_id: GuildId, name: &str, special_message: &str, http: Arc<Http>) {
}

async fn celebrate_birthday(guild_id: GuildId, user_id: UserId, nick: &str, http: Arc<Http>) {
}

#[derive(Clone, FromRow, Debug)]
struct BirthdayRow {
    #[sqlx(try_from="i64")]
    id: u64,
    day: u8,
    month: u8,
    nick: String,
}

#[derive(Clone, FromRow, Debug)]
struct EventRow {
    #[sqlx(try_from="i64")]
    guild: u64,
    name: String,
    day: u8,
    month: u8,
    special_message: String,
}

async fn notice(ctx: serenity::client::Context) -> anyhow::Result<()> {
    use anyhow::Context;

    let local = Local::now();
    let day = local.day();
    let month = local.month();

    let db_path = dotenv_var("DATABASE_URL").context("DATABASE_URL not found in env")?;

    let mut db = SqliteConnection::connect(db_path.as_str()).await?;

    let birtdays = sqlx::query_as::<_, BirthdayRow>(
        "SELECT * FROM birthdays
        WHERE day=? AND month=?;"
    )
    .bind(day)
    .bind(month)
    .fetch_all(&mut db)
    .await?;

    let global_events = sqlx::query_as::<_, EventRow>(
        "SELECT guild, name, day, month, specialMessage from events
        WHERE day=? AND month=? AND guild=0;"
    )
    .bind(day)
    .bind(month)
    .fetch_all(&mut db)
    .await?;

    let guilds = ctx.http.get_guilds(None, None).await?;

    for guild in guilds {
        let guild_id = guild.id;

        for bd in &birtdays {
            let user_id = UserId::new(bd.id);
            guild_id.member(ctx.http(), user_id).await?;

            celebrate_birthday(guild_id, user_id, bd.nick.as_str(), ctx.http.clone()).await;
        }

        for e in &global_events {
            announce_event(guild_id, e.name.as_str(), e.special_message.as_str(), ctx.http.clone()).await;
        }
    }

    let global_events = sqlx::query_as::<_, EventRow>(
        "SELECT guild, name, day, month, specialMessage from events
        WHERE day=? AND month=? AND guild!=0;"
    )
    .bind(day)
    .bind(month)
    .fetch_all(&mut db)
    .await?;

    for e in &global_events {
        announce_event(GuildId::new(e.guild), e.name.as_str(), e.special_message.as_str(), ctx.http.clone()).await;
    }

    Ok(())
}

