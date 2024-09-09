pub mod system;
pub mod task;
pub mod tui_me;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliTracker {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Add a new task
    Add {
        // Task description
        description: String,
    },

    // List tasks
    List {
        // List completed, in-progress, or to_do
        #[arg(short, long, default_value = "all")]
        status: String,
    },
    Done {
        id: String,
    },
    Update {
        id: String,

        // Optional description update
        description: String,
    },
    Delete {
        id: String,
    },
    Stage {
        id: String,
    },
    Search {
        // Does searching and filtering
        search_key: String,

        // If a user specifies space to search
        #[arg(short, long, default_value = "all")]
        space: String,
    },
    Helps,
    Save,
    Exit,
    Cpu {
        usage: Option<String>,
    },
    Hello,
}
