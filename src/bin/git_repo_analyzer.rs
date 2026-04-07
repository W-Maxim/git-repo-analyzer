use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-repo-analyzer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Authors,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Authors => {
            println!("Authors command wird ausgeführt");
        }
    }
}