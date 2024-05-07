use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "sqlite::memory:";
const DB_NAME: &str = "event-engine_db";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await?;

    Ok(())
}

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
}
