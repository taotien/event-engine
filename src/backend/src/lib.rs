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
    pub async fn add_event(&self, pool: &SqlitePool) -> anyhow::Result<()> {
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

        Ok(())
    }
}
