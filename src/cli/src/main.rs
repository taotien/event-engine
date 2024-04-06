use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
    },

    /// Update web crawler cache
    Update {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { radius, method }) => {
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
        }
        Some(Commands::Update {}) => todo!(),
        None => {}
    }

    /* TODO: call backend */
}
