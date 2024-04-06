use std::env;

use clap::Parser;
use sqlx::SqlitePool;

use backend::Event;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Add { json: String },
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO http server receive requests

    let args = Cli::parse();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match &args.subcommand {
        Commands::Add { json } => {
            let parsed: Event = serde_json::from_str(json)?;
            parsed.add_event(&pool).await?;
        }
        Commands::List => {
            list_events(&pool).await?;
        }
    }

    Ok(())
}

async fn list_events(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
            SELECT id, name
            FROM events
            ORDER BY name
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!("[{}]: id: {}", rec.name, rec.id,);
    }

    Ok(())
}
