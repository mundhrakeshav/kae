use crate::style;
use crate::task::{ListMode, Task, TaskList, TaskStatus};
use anyhow::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListState, Padding, Paragraph, StatefulWidget, Widget,
    Wrap,
};
use ratatui::{symbols, DefaultTerminal};
use ratatui::{text::Line, widgets::ListItem};
use serde_json;

#[derive(Debug, PartialEq)]
enum UIMode {
    View,
    Insert,
    EditName,
    EditDescription,
}

#[derive(Debug, PartialEq)]
enum ActiveEditField {
    None,
    Name,
    Description,
}

pub struct UI {
    task_list: TaskList,
    should_exit: bool,
    mode: UIMode,
    input_buffer: String,
    active_edit_field: ActiveEditField,
}

impl UI {
    pub fn new(tasks: Vec<Task>) -> Self {
        let task_list = TaskList {
            tasks: tasks,
            state: ListState::default(),
            mode: ListMode::View,
        };
        Self {
            task_list,
            should_exit: false,
            mode: UIMode::View,
            input_buffer: String::new(),
            active_edit_field: ActiveEditField::None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.mode {
            UIMode::View => match key.code {
                KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Char('i') => self.mode = UIMode::Insert,
                KeyCode::Char('e') => {
                    if let Some(selected_index) = self.task_list.state.selected() {
                        self.mode = UIMode::EditName;
                        self.active_edit_field = ActiveEditField::Name;
                        self.input_buffer = self.task_list.tasks[selected_index].name.clone();
                    }
                }
                KeyCode::Char('d') => {
                    if let Some(selected_index) = self.task_list.state.selected() {
                        self.mode = UIMode::EditDescription;
                        self.active_edit_field = ActiveEditField::Description;
                        self.input_buffer = self.task_list.tasks[selected_index].description.clone();
                    }
                }
                KeyCode::Esc => self.mode = UIMode::View,
                KeyCode::Left => self.select_none(),
                KeyCode::Down => self.select_next(),
                KeyCode::Up => self.select_previous(),
                KeyCode::Home => self.select_first(),
                KeyCode::End => self.select_last(),
                KeyCode::Tab => self.toggle_status_and_save(),
                _ => {}
            },
            UIMode::Insert => match key.code {
                KeyCode::Esc => self.mode = UIMode::View,
                _ => {}
            },
            UIMode::EditName | UIMode::EditDescription => match key.code {
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Enter => {
                    self.confirm_edit_and_save();
                    self.mode = UIMode::View;
                    self.active_edit_field = ActiveEditField::None;
                    self.input_buffer.clear();
                }
                KeyCode::Esc => {
                    self.mode = UIMode::View;
                    self.active_edit_field = ActiveEditField::None;
                    self.input_buffer.clear();
                }
                _ => {}
            },
        }
    }

    fn select_none(&mut self) {
        self.task_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.task_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.task_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.task_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.task_list.state.select_last();
    }

    fn toggle_status(&mut self) {
        if let Some(i) = self.task_list.state.selected() {
            self.task_list.tasks[i].status = match self.task_list.tasks[i].status {
                TaskStatus::Todo => TaskStatus::InProgress,
                TaskStatus::InProgress => TaskStatus::Done,
                TaskStatus::Done => TaskStatus::Todo,
            };
        }
    }

    fn toggle_status_and_save(&mut self) {
        self.toggle_status();
        self.save_tasks();
    }

    fn confirm_edit_and_save(&mut self) {
        if let Some(selected_index) = self.task_list.state.selected() {
            match self.active_edit_field {
                ActiveEditField::Name => {
                    self.task_list.tasks[selected_index].name = self.input_buffer.clone();
                }
                ActiveEditField::Description => {
                    self.task_list.tasks[selected_index].description = self.input_buffer.clone();
                }
                ActiveEditField::None => {}
            }
            self.save_tasks();
        }
    }

    fn save_tasks(&self) -> Result<()> {
        let tasks_json = serde_json::to_string_pretty(&self.task_list.tasks)?;
        crate::utils::write_todos_to(crate::utils::CONFIG_PATH, tasks_json)?;
        Ok(())
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("KAE").bold().centered().render(area, buf);
    }

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let view_s = "Use ↓↑ to move, ← to unselect, TAB to change status, 'e' to edit name, 'd' to edit description, 'q' to exit";
        let edit_s = "Type to edit, Enter to save, Esc to cancel";

        let text = match self.mode {
            UIMode::View => format!("{} | mode: {} ", view_s, "VIEW"),
            UIMode::Insert => format!("{} | mode: {} ", view_s, "INSERT"),
            UIMode::EditName => format!("{} | mode: {} ", edit_s, "EDIT NAME"),
            UIMode::EditDescription => format!("{} | mode: {} ", edit_s, "EDIT DESCRIPTION"),
        };

        Paragraph::new(text).centered().render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let [list_area, item_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]).areas(area);
        self.render_list_titles(list_area, buf);
        self.render_selected_item(item_area, buf);
    }

    fn render_list_titles(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(style::TODO_HEADER_STYLE)
            .bg(style::NORMAL_ROW_BG);

        let items: Vec<ListItem> = self
            .task_list
            .tasks
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(style::SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.task_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let block_title = Line::raw("TODO Info").centered();
        let mut current_name = "Nothing selected...".to_string();
        let mut current_description = "".to_string();
        let mut current_status_prefix = "".to_string();

        if let Some(i) = self.task_list.state.selected() {
            let task = &self.task_list.tasks[i];
            current_name = task.name.clone();
            current_description = task.description.clone();
            current_status_prefix = match task.status {
                TaskStatus::Done => "✓ DONE".to_string(),
                TaskStatus::Todo => "☐ TODO".to_string(),
                TaskStatus::InProgress => "◌ IN PROGRESS".to_string(),
            };
        }

        let mut text_to_display = vec![];

        // Name display/edit
        if self.mode == UIMode::EditName && self.active_edit_field == ActiveEditField::Name {
            text_to_display.push(Line::styled(
                format!("Name: {}_", self.input_buffer),
                Style::default().fg(style::TEXT_FG_COLOR).add_modifier(Modifier::BOLD),
            ));
        } else {
            text_to_display.push(Line::styled(
                format!("Name: {}", current_name),
                Style::default().fg(style::TEXT_FG_COLOR),
            ));
        }

        // Description display/edit
        if self.mode == UIMode::EditDescription && self.active_edit_field == ActiveEditField::Description {
            text_to_display.push(Line::styled(
                format!("Description: {}_", self.input_buffer),
                Style::default().fg(style::TEXT_FG_COLOR).add_modifier(Modifier::BOLD),
            ));
        } else {
            text_to_display.push(Line::styled(
                format!("Description: {}", current_description),
                Style::default().fg(style::TEXT_FG_COLOR),
            ));
        }
        
        // Status (always view only in this pane)
        if self.task_list.state.selected().is_some() {
             text_to_display.push(Line::styled(
                format!("Status: {}", current_status_prefix),
                Style::default().fg(style::TEXT_FG_COLOR),
            ));
        }


        let block = Block::new()
            .title(block_title)
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(style::TODO_HEADER_STYLE)
            .bg(style::NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        Paragraph::new(text_to_display)
            .block(block)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl Widget for &mut UI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        UI::render_header(header_area, buf);
        self.render_footer(footer_area, buf);
        self.render_list(main_area, buf);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        style::NORMAL_ROW_BG
    } else {
        style::ALT_ROW_BG_COLOR
    }
}
