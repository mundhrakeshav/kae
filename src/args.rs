use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Initialize a new project.
    Init,
    /// List existing items
    List {
        /// Name of todos to filter by
        #[arg(short, long)]
        name: Option<String>,
        /// ID of todo to fetch
        #[arg(short, long)]
        id: Option<Uuid>,
        /// List all todos
        #[arg(short, long)]
        all: bool,
    },
    /// Add a new item
    Add { name: String, description: String },
    /// Remove an existing item.
    Remove {},
    /// Update an existing item.
    Update {
        /// ID of the todo to update
        #[arg(short, long)]
        id: Uuid,
        /// New name for the todo
        #[arg(long)]
        name: Option<String>,
        /// New description for the todo
        #[arg(long)]
        description: Option<String>,
        /// New status for the todo (Todo, InProgress, Done)
        #[arg(long)]
        status: Option<String>,
    },
}
