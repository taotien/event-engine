use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List {
        /// Optional radius
        #[arg(short, long)]
        radius: Option<u32>,

        /// Optional distance
        #[arg(short, long)]
        distance: Option<u32>,
    },

    Update {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { radius, distance }) => {
            if let Some(radius) = radius {
                println!("radius: {radius}");
            }

            if let Some(distance) = distance {
                println!("distance: {distance}");
            }
        }
        Some(Commands::Update {}) => todo!(),
        None => {}
    }

    /* TODO: call backend */
}
