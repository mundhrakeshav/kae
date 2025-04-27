use crate::style;
use crate::task::{ListMode, Task, TaskList, TaskStatus};
use anyhow::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListState, Padding, Paragraph, StatefulWidget, Widget,
    Wrap,
};
use ratatui::{symbols, DefaultTerminal};
use ratatui::{text::Line, widgets::ListItem};

#[derive(Debug)]
enum UIMode {
    View,
    Insert,
}

pub struct UI {
    task_list: TaskList,
    should_exit: bool,
    mode: UIMode,
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
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Char('i') => self.mode = UIMode::Insert,
            KeyCode::Esc => self.mode = UIMode::View,
            KeyCode::Left => self.select_none(),
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Home => self.select_first(),
            KeyCode::End => self.select_last(),
            KeyCode::Tab => self.toggle_status(),
            _ => {}
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

    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        if let Some(i) = self.task_list.state.selected() {
            self.task_list.tasks[i].status = match self.task_list.tasks[i].status {
                TaskStatus::Todo => TaskStatus::InProgress,
                TaskStatus::InProgress => TaskStatus::Done,
                TaskStatus::Done => TaskStatus::Todo,
            }
        }
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("KAE").bold().centered().render(area, buf);
    }

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let s = "Use ↓↑ to move, ← to unselect, 'TAB' to change status, 'q' to exit";
        let text = match self.mode {
            UIMode::View => format!("{} | mode: {} ", s, "VIEW"),
            UIMode::Insert => format!("{} | mode: {} ", s, "INSERT"),
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

        // Iterate through all elements in the `items` and stylize them.
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

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(style::SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.task_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.task_list.state.selected() {
            match self.task_list.tasks[i].status {
                TaskStatus::Done => format!("✓ DONE: {}", self.task_list.tasks[i].description),
                TaskStatus::Todo => format!("☐ TODO: {}", self.task_list.tasks[i].description),
                TaskStatus::InProgress => {
                    format!("◌ IN PROGRESS: {}", self.task_list.tasks[i].description)
                }
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(style::TODO_HEADER_STYLE)
            .bg(style::NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(style::TEXT_FG_COLOR)
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
