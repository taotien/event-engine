use std::env;

use clap::Parser;
use sqlx::{Connection, SqliteConnection, SqlitePool};

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}
/*
Need a parser for json to sql (struct)

*/
#[derive(clap::Subcommand)]
enum Commands {
    Add { json: String },
    List,
    // Remove { id: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match &args.subcommand {
        Commands::Add { json } => {
            let parsed = serde_json::from_str(json)?;
        }
        Commands::List => {
            todo!()
        }
    }

    // Ok(())
}
