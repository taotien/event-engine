use std::env;
use std::process::exit;

use chrono::TimeDelta;
use clap::{Parser, Subcommand};
use google_maps::directions::TravelMode;

use backend::{init_pool, Event};
use cli::config;
use filters::filter::filter_events;

use cli::serialize::cnvt_event_to_json;
use std::process::Command;

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
        verbose: bool,
    },

    /// Update web crawler cache
    Update {},
}

#[tokio::main]
async fn main() {
    let db_conn_pool = init_pool().await;

    let max_travel_time: TimeDelta;
    let mut config_place: config::Place = config::get_config();

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

            let mut transit;
            if config_place.transit == "walk" {
                transit = Some(TravelMode::Walking);
            } else if config_place.transit == "car" {
                transit = Some(TravelMode::Driving);
            } else if config_place.transit == "transit" {
                transit = Some(TravelMode::Transit);
            } else {
                println!("Error: unsupported transit method");
                exit(-1);
            }

            if config_place.transit == "walk" {
                transit = Some(TravelMode::Walking);
            } else if config_place.transit == "car" {
                transit = Some(TravelMode::Driving);
            } else if config_place.transit == "transit" {
                transit = Some(TravelMode::Transit);
            } else {
                println!("Error: unsupported transit method");
                exit(-1);
            }

            if let Some(travel_time) = time.clone() {
                max_travel_time = TimeDelta::minutes(travel_time);
            }
        }

        Commands::Status { verbose } => {
            /* TODO: database cache timestamp */
            let openai_api_key = env::var("OPENAI_API_KEY");
            let google_maps_api_key = env::var("GOOGLE_MAPS_API_KEY");

            let mut err: bool = false;
            if openai_api_key.is_ok() {
                if *verbose {
                    println!("OPENAI_API_KEY: {:?}", openai_api_key.unwrap());
                } else {
                    println!("OpenAI API key is valid");
                }
            } else {
                println!("Error: OpenAI API key not found");
                err = true;
            }

            if google_maps_api_key.is_ok() {
                if *verbose {
                    println!("GOOGLE_MAPS_API_KEY: {:?}", google_maps_api_key.unwrap());
                } else {
                    println!("Google Maps API key is valid");
                }
            } else {
                println!("Error: Google Maps key not found");
                err = true;
            }

            /* One or more missing keys */
            if err {
                exit(-1);
            }
        }

        Commands::Update {} => todo!(),
    }
}
