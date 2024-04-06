use std::{env, sync::Arc};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use clap::Parser;
use sqlx::{Pool, Sqlite, SqlitePool};

use backend::Event;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Serve,
    Add { json: String },
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    let pool = Arc::new(SqlitePool::connect(&env::var("DATABASE_URL")?).await?);

    let app = Router::new()
        .route(
            "/list",
            get({
                let pool = Arc::clone(&pool);
                move || list(pool)
            }),
        )
        .route(
            "/add",
            post({
                let pool = Arc::clone(&pool);
                move |body| add(body, pool)
            }),
        );

    match &args.subcommand {
        Commands::Add { json } => {
            let parsed: Event = serde_json::from_str(json)?;
            parsed.add_event(&pool).await?;
        }
        Commands::List => {
            Event::list_events(&pool).await?;
        }
        Commands::Serve => {
            let listener = tokio::net::TcpListener::bind("0.0.0.0:8787").await?;
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}

async fn list(pool: Arc<Pool<Sqlite>>) {
    Event::list_events(&pool).await.unwrap();
}

async fn add(axum::Json(payload): axum::Json<Event>, pool: Arc<Pool<Sqlite>>) -> StatusCode {
    // let parsed: Event = serde_json::from_str(payload).unwrap();
    payload.add_event(&pool).await.unwrap();

    // (StatusCode::CREATED, axum::Json(event))
    StatusCode::CREATED
}
