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
            if let Some(max_radius) = radius {
                println!("radius: {max_radius}");
            }

            if let Some(transit_method) = method {
                if transit_method == "walk" {
                    println!("Transit method: walking");
                    /* TODO */
                } else if transit_method == "car" {
                    println!("Transit method: car");
                    /* TODO */
                } else if transit_method == "transit" {
                    println!("Transit method: public transit");
                    /* TODO */
                } else {
                    println!("Error: unsupported transit method");
                    exit(-1);
                }
            }

            if let Some(travel_time) = time.clone() {
                max_travel_time = TimeDelta::minutes(travel_time);
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
