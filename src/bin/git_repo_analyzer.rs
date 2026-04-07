use clap::{Parser, Subcommand};
use git_repo_analyzer::git;
use git_repo_anazyler::analytics;

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
            let authors = git::get_authors();
            let stats = analytics::count_authors(authors);
            println!("Authors command wird ausgeführt");
        }
    }
}