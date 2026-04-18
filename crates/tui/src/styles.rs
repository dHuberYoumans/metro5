use domain::entities::LogLevel;
use ratatui::style::{Color, Style};

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

pub(crate) fn search_result_style() -> Style {
    Style::default().bg(Color::Yellow)
}

pub(crate) fn help_style() -> Style {
    Style::default().bg(Color::DarkGray)
}

pub(crate) fn help_title_style() -> Style {
    Style::default().bold()
}

pub(crate) fn pending_key_style() -> Style {
    Style::default().fg(Color::Blue)
}
