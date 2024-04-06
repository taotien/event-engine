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
    },

    Update {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { radius }) => {
            if let Some(radius) = radius {
                println!("radius: {radius}");
            }
        }
        Some(Commands::Update {}) => todo!(),
        None => {}
    }

    /* TODO: call backend */
}
