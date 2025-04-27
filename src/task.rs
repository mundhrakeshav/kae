use anyhow::{Ok, Result};
use ratatui::{
    text::Line,
    widgets::{ListItem, ListState},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{style, utils};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
}

pub enum ListMode {
    View,
    Modify,
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub mode: ListMode,
}

impl Task {
    pub fn new(name: String, description: String) -> Task {
        return Task {
            id: Uuid::new_v4(),
            name,
            description,
            status: TaskStatus::Todo,
        };
    }

    pub fn from_file(path: &str) -> Result<Vec<Task>> {
        let json_todos = utils::read_todos_from(path)?;
        let tasks: Vec<Task> = serde_json::from_str(json_todos.as_str())?;
        Ok(tasks)
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(task: &Task) -> Self {
        let line = match task.status {
            TaskStatus::Todo => {
                Line::styled(format!(" ☐ {}", task.name), style::TODO_TEXT_FG_COLOR)
            }
            TaskStatus::InProgress => Line::styled(
                format!(" ◌ {}", task.name),
                style::IN_PROGRESS_TEXT_FG_COLOR,
            ),
            TaskStatus::Done => {
                Line::styled(format!(" ✓ {}", task.name), style::COMPLETED_TEXT_FG_COLOR)
            }
        };

        ListItem::new(line)
    }
}
