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
    price: u64,
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

        let id = sqlx::query!(
            r#"
            INSERT INTO events ( name )
            VALUES ( ?1 )
        "#,
            self.name
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
