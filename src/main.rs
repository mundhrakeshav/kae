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
            let tasks = Task::from_file(utils::CONFIG_PATH)?;
            ui::UI::new(tasks)
                .run(ratatui::init())
                .expect("Failed to run UI");

            Ok(())
        }
        args::Commands::Update {
            id,
            name,
            description,
            status,
        } => {
            let mut tasks = Task::from_file(utils::CONFIG_PATH)?;
            if let Some(task_to_update) = tasks.iter_mut().find(|t| t.id == id) {
                if let Some(n) = name {
                    task_to_update.name = n;
                }
                if let Some(d) = description {
                    task_to_update.description = d;
                }
                if let Some(s) = status {
                    match s.to_lowercase().as_str() {
                        "todo" => task_to_update.status = task::TaskStatus::Todo,
                        "inprogress" => task_to_update.status = task::TaskStatus::InProgress,
                        "done" => task_to_update.status = task::TaskStatus::Done,
                        _ => {
                            return Err(anyhow::anyhow!(
                                "Invalid status: {}. Must be one of Todo, InProgress, Done",
                                s
                            ));
                        }
                    }
                }
            } else {
                return Err(anyhow::anyhow!("Task with ID {} not found", id));
            }

            let updated_tasks_json = serde_json::to_string_pretty(&tasks)?;
            utils::write_todos_to(utils::CONFIG_PATH, updated_tasks_json)?;
            tracing::info!("Task {} updated successfully.", id);
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
