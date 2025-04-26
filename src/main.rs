mod args;
mod style;
mod task;
mod ui;
mod utils;
use anyhow::Result;
use args::AppArgs;
use clap::Parser;
use task::Task;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use ui::UI;

fn main() {
    set_tracing();
    tracing::debug!("arigato!!");

    let args = AppArgs::parse();

    match handle_command(args.command) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("error: {:?}", e)
        }
    }
}

fn handle_command(cmd: args::Commands) -> Result<()> {
    match cmd {
        args::Commands::Init => {
            return utils::init_todos();
        }
        args::Commands::Add { name, description } => Ok(()),
        args::Commands::List { name, all, id } => {
            print!("{:?}, {}, {:?}", name, all, id);
            let tasks = Task::from_file(utils::CONFIG_PATH)?;
            ui::UI::new(tasks)
                .run(ratatui::init())
                .expect("Failed to run UI");

            Ok(())
        }
        _ => Ok(()),
    }
}

fn set_tracing() {
    let filter = EnvFilter::from_default_env()
        .add_directive("kae=debug".parse().unwrap())
        .add_directive("kae::task=info".parse().unwrap());

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        // .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
}
