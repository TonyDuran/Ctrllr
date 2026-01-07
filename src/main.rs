use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ctrllr")]
#[command(author = "TonyDuran")]
#[command(version = "0.1.0")]
#[command(about = "A better bash history tool - version controlled, taggable, and shareable", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize ctrllr in the current directory
    Init,
    /// Search through command history
    Search {
        /// Search query
        query: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            println!("Initializing ctrllr...");
            // TODO: Initialize git repository for history storage
        }
        Some(Commands::Search { query }) => {
            match query {
                Some(q) => println!("Searching for: {}", q),
                None => println!("Opening interactive search..."),
            }
            // TODO: Implement ratatui-based interactive search
        }
        None => {
            println!("Welcome to ctrllr!");
            println!("Use 'ctrllr --help' to see available commands.");
        }
    }
}
