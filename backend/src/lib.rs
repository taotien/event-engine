use std::{env, sync::Arc};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub price: Option<String>,
    pub tags: Option<Vec<String>>,
    pub source: Option<String>,
    pub check_list: Option<Vec<String>>,
}

pub async fn init_pool() -> Arc<Pool<Sqlite>> {
    Arc::new(
        SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    )
}

impl Event {
    pub async fn get_events(pool: &SqlitePool) -> anyhow::Result<Vec<Event>> {
        let recs = sqlx::query!(
            r#"
            SELECT id, name, start_time, end_time, location, description, price, tags, source
            FROM events
            ORDER BY name
        "#
        )
        .fetch_all(pool)
        .await?;

        let res = recs.iter().map(|r| {
            let mut tags: Vec<String> = Vec::new();
            if let Some(t) = &r.tags {
                tags = t.split(";").map(|s| s.to_owned()).collect();
            };
            let tags = if !tags.is_empty() { Some(tags) } else { None };

            let mut check_list: Vec<String> = Vec::new();
            if let Some(c) = &r.tags {
                check_list = c.split(";").map(|s| s.to_owned()).collect();
            };
            let check_list = if !check_list.is_empty() {
                Some(check_list)
            } else {
                None
            };

            Event {
                name: r.name.clone(),
                start_time: r.start_time.clone(),
                end_time: r.end_time.clone(),
                location: r.location.clone(),
                description: r.description.clone(),
                price: r.price.clone(),
                tags,
                source: r.source.clone(),
                check_list,
            }
        });

        Ok(res.collect())
    }

    pub async fn print_events(pool: &SqlitePool) -> anyhow::Result<()> {
        let recs = sqlx::query!(
            r#"
            SELECT id, name, start_time, end_time, location, description, price, tags, source, check_list
            FROM events
            ORDER BY name
        "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            println!(
                "[{}]:
                    id: {:?}
                    start: {:?}
                    end: {:?}
                    loc: {:?}
                    desc: {:?}
                    price: {:?}
                    tags: {:?}
                    source: {:?}
                    check_list: {:?}
                    ",
                rec.name,
                rec.id,
                rec.start_time,
                rec.end_time,
                rec.location,
                rec.description,
                rec.price,
                rec.tags,
                rec.source,
                rec.check_list
            );
        }

        Ok(())
    }

    pub async fn add_event(&self, pool: &SqlitePool) -> anyhow::Result<i64> {
        let mut conn = pool.acquire().await?;

        let tags: Option<String> = match &self.tags {
            Some(t) => Some(t.join(";")),
            None => None,
        };
        let source = self.source.clone().map(|u| u.to_string());
        let check_list = self.check_list.clone().unwrap();
        let check_list = check_list.join(";");
        let id = sqlx::query!(
            r#"
             INSERT INTO events ( name, start_time, end_time, location, description, price, tags, source, check_list )
             VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9 ) 
        "#,
            self.name,
            self.start_time,
            self.end_time,
            self.location,
            self.description,
            self.price,
            tags,
            source,
            check_list,
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
