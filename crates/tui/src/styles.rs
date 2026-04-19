use domain::entities::LogLevel;
use ratatui::style::{Color, Modifier, Style};

pub(crate) fn level_style(level: &LogLevel) -> Style {
    match level {
        LogLevel::Info => Style::default().fg(Color::Cyan),
        LogLevel::Warn => Style::default().fg(Color::Yellow),
        LogLevel::Error => Style::default().fg(Color::Red),
        LogLevel::Log => Style::default().fg(Color::LightGreen),
        LogLevel::None => Style::default(),
    }
}

pub(crate) fn title_style() -> Style {
    Style::default().bold().fg(Color::Indexed(214)) // 8-bit orange
}

pub(crate) fn command_line_style() -> Style {
    Style::default().fg(Color::Indexed(214)) // 8-bit orange
}

pub(crate) fn border_style() -> Style {
    Style::default().fg(Color::Blue).bold() // 8-bit blue
}

pub const SEARCH_RESULT_STYLE: Style = Style::new().bg(Color::Yellow);

pub(crate) fn pending_key_style() -> Style {
    Style::default().fg(Color::Blue)
}

pub const HELP_BLOCK_STYLE: Style = Style::new().bg(Color::DarkGray);

pub const HELP_TITLE_STYLE: Style = Style::new().bold();

pub const HELP_SELECTED_STYLE: Style = Style::new().fg(Color::White);

pub const HELP_HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::REVERSED);
