use std::{env, sync::Arc};

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    start_time: Option<String>,
    end_time: Option<String>,
    location: Option<String>,
    description: Option<String>,
    price: Option<String>,
    tags: Option<Vec<String>>,
    source: Option<Url>,
    check_list: Vec<String>,
}

pub async fn init_pool() -> Arc<Pool<Sqlite>> {
    Arc::new(
        SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    )
}

impl Event {
    // pub async fn get_events(pool: &SqlitePool) -> anyhow::Result<Vec<Event>> {
    //     let recs = sqlx::query!(
    //         r#"
    //         SELECT id, name, start_time, end_time, location, description, price, tags, source
    //         FROM events
    //         ORDER BY name
    //     "#
    //     )
    //     .fetch_all(pool)
    //     .await?;

    //     let res = recs.iter().map(|r| {
    //         let name = r.name;
    //         let start_time = r.start_time;
    //         let end_time = r.end_time;
    //         let location = r.location;
    //         let description = r.description;
    //         let price = r.price.try_into().unwrap();
    //         // let tags = Some(r.tags.split(";").collect());
    //         // let tags = Some(r.tags.unwrap().collect());
    //         let tags = {
    //             let t = r.tags.split(";");
    //             let list = t.collect();
    //             Ok(list)
    //         };
    //         let source = r.source.parse().ok();
    //         Event {
    //             name,
    //             start_time,
    //             end_time,
    //             location,
    //             description,
    //             price,
    //             tags,
    //             source,
    //             check_list: r.check_list,
    //         }
    //     });

    //     Ok(res)
    // }

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
            source,
            check_list,
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
