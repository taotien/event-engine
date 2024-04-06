use chrono::TimeDelta;
use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List events
    List {
        /// Maximum allowed straight line distance
        #[arg(short, long)]
        radius: Option<u32>,

        /// Transit methods: {walk|car|transit}
        #[arg(short, long)]
        method: Option<String>,

        /// Maximum allowed travel time in minutes
        #[arg(short, long)]
        time: Option<i64>,
    },

    /// Check status
    Status {
        /// View the values of the API keys
        #[arg(long, short, action = clap::ArgAction::SetTrue)]
        verbose: Option<bool>,
    },

    /// Update web crawler cache
    Update {},
}

fn main() {
    let max_travel_time: TimeDelta;

    let cli = Cli::parse();
    match &cli.command {
        Commands::List {
            radius,
            method,
            time,
        } => {
            if let Some(radius) = radius {
                println!("radius: {radius}");
            }

            if let Some(method) = method {
                if method == "walk" {
                    println!("Transit method: walking");
                    /* TODO */
                } else if method == "car" {
                    println!("Transit method: car");
                    /* TODO */
                } else if method == "transit" {
                    println!("Transit method: public transit");
                    /* TODO */
                } else {
                    println!("Error: unsupported transit method");
                    exit(-1);
                }
            }

            if let Some(time) = time.clone() {
                max_travel_time = TimeDelta::minutes(time);
                println!("Maximum travel time: {}", max_travel_time);
                /* TODO */
            }
        }

        Commands::Status { verbose } => {
            if let Some(verbose) = verbose {
                println!("verbose: {verbose}");
                /* TODO */
            }
        }

        Commands::Update {} => todo!(),
    }

    /* TODO: call backend */
}
