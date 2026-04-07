use clap::{Parser, Subcommand};
use git_repo_analyzer::git;
use git_repo_analyzer::analytics;

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

            for (index, stats) in stats.iter().enumerate() {
                println!("{}. {}: {} commits", index + 1, stats.name, stats.commit_count);
            }
        }
    }
}

