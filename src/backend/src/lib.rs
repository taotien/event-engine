use std::{env, fs, time::SystemTime};

use anyhow::Context;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    location: Option<String>,
    desc: Option<String>,
    price: u32,
    tags: Option<Vec<String>>,
    source: Option<Url>,
}

impl Event {
    pub async fn list_events(pool: &SqlitePool) -> anyhow::Result<()> {
        let recs = sqlx::query!(
            r#"
            SELECT id, name, start_time, end_time, location, description, price, tags, source
            FROM events
            ORDER BY name
        "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            println!(
                "[{}]: 
                    id: {}
                    start: {}
                    end: {}
                    loc: {}
                    desc: {}
                    price: {}
                    tags: {}
                    source: {}
                    ",
                rec.name,
                rec.id,
                rec.start_time,
                rec.end_time,
                rec.location,
                rec.description,
                rec.price,
                rec.tags,
                rec.source
            );
        }

        Ok(())
    }
    pub async fn add_event(&self, pool: &SqlitePool) -> anyhow::Result<i64> {
        let mut conn = pool.acquire().await?;

        let start = self.start_time.to_string();
        let end = self.end_time.to_string();
        let tags : Option<String> = match self.tags {
            Some(t) => Some(t.join(";")),
            None => None
        }

        // let tags: Option<String> = self.tags.
        let id = sqlx::query!(
            r#"
            INSERT INTO events ( name, start_time, end_time, location, description, price )
            VALUES ( ?1, ?2, ?3, ?4, ?5, ?6 ) 
        "#,
            self.name,
            start,
            end,
            self.location,
            self.desc,
            self.price,
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}

pub fn check_db_freshness() -> anyhow::Result<SystemTime> {
    let mut db_var = env::var("DATABASE_URL").to_owned()?;
    let db_path = db_var.split_off(db_var.find(':').unwrap());

    let meta = fs::metadata(db_path)?;
    Ok(meta.modified()?)
}
