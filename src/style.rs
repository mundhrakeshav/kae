use ratatui::style::{
    palette::tailwind::{BLUE, FUCHSIA, GREEN, SLATE},
    Color, Modifier, Style,
};

pub const TODO_TEXT_FG_COLOR: Color = SLATE.c200;
pub const IN_PROGRESS_TEXT_FG_COLOR: Color = FUCHSIA.c300;
pub const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
pub const NORMAL_ROW_BG: Color = SLATE.c950;
pub const ALT_ROW_BG_COLOR: Color = SLATE.c900;
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
pub const TEXT_FG_COLOR: Color = SLATE.c200;
