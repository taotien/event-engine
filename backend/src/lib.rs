use std::{env, sync::Arc};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    /// time in format YYYY,DD,MM,HH,MM,SS
    pub start_time: String,
    /// time in format YYYY,DD,MM,HH,MM,SS
    pub end_time: String,
    /// address that google maps can understand
    pub location: String,
    pub description: String,
    pub price: String,
    pub tags: Vec<String>,
    /// url of the source we scraped from
    pub source: String,
    /// list of things you should prepare before you go
    pub check_list: Vec<String>,
}

/// initialize a connection pool to the sqlite database
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
            // let tags = if !tags.is_empty() { Some(tags) } else { None };

            let mut check_list: Vec<String> = Vec::new();
            if let Some(c) = &r.tags {
                check_list = c.split(";").map(|s| s.to_owned()).collect();
            };
            // let check_list = if !check_list.is_empty() {
            //     Some(check_list)
            // } else {
            //     None
            // };

            Event {
                name: r.name.clone(),
                start_time: r.start_time.clone().unwrap(),
                end_time: r.end_time.clone().unwrap(),
                location: r.location.clone().unwrap(),
                description: r.description.clone().unwrap(),
                price: r.price.clone().unwrap(),
                tags,
                source: r.source.clone().unwrap(),
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

        // let tags: Option<String> = match &self.tags {
        //     Some(t) => Some(t.join(";")),
        //     None => None,
        // };
        let tags = self.tags.join(";");
        // let source = self.source.clone().map(|u| u.to_string());
        // let check_list = self.check_list.clone().unwrap();
        let check_list = self.check_list.join(";");
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
            self.source,
            check_list,
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
