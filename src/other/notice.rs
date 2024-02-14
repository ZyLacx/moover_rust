use serenity::prelude::*;
use chrono::{Local, Datelike};

// use async_sqlite::{rusqlite::{Rows, Statement}, ClientBuilder, JournalMode};

use crate::util::debug::send_error;

use anyhow::Result;

use async_rusqlite::Connection;

pub async fn notice_wrapper(ctx: Context) {
    match notice(ctx.clone()).await {
        Ok(_) => return,
        Err(e) => {
            send_error(ctx.http.clone(), e.to_string()).await;
            return
        }
    }
}

struct BirtdayRow {
    day: u8,
    month: u8,
    nick: String
}

async fn notice(ctx: Context) -> anyhow::Result<()> {
    let local = Local::now();
    let day = local.day();
    let month = local.month();


    let conn = Connection::open("my.db").await?;

    let stmt = conn.call(move |conn| {
        conn.prepare("SELECT * FROM birthdays WHERE day=6 AND month=3;")
    }).await?;

    // let result = Vec::new();
    // let stmt = client.conn(|conn| {
    //     // conn.prepare(format!("SELECT * FROM {db} WHERE day={day} AND month={month};").as_str());
    //     conn.prepare("SELECT * FROM birthdays WHERE day=6 AND month=3;")
    // }).await?;

    let rows = stmt.query([])?;

    let result: Vec<BirtdayRow> = Vec::new();
    while let Some(row) = rows.next()? {
        let bd = BirtdayRow {
            day: row.get(1)?,
            month: row.get(2)?,
            nick: row.get(3)?
        };
        result.push(bd);
    }

    println!("VALUE: {:?}", result[0].day);
    // for db in ["birthdays", "events"] {
        
    //     // let mut stmt = client.conn(|conn| {
    //     //     conn.
    //     // }).await;
    // }

    Ok(())
}