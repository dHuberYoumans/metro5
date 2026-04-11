use domain::entities::LogLevel;
use ratatui::style::{Color, Style};

pub fn level_style(level: &LogLevel) -> Style {
    match level {
        LogLevel::Info => Style::default().fg(Color::Cyan),
        LogLevel::Warn => Style::default().fg(Color::Yellow),
        LogLevel::Error => Style::default().fg(Color::Red),
        LogLevel::Log => Style::default().fg(Color::LightGreen),
        LogLevel::None => Style::default(),
    }
}

pub fn title_style() -> Style {
    Style::default().bold().fg(Color::Indexed(214)) // 8-bit orange
}

pub fn command_line_style() -> Style {
    Style::default().fg(Color::Indexed(214)) // 8-bit orange
}

pub fn border_style() -> Style {
    Style::default().fg(Color::Blue).bold() // 8-bit blue
}

pub fn search_result_style() -> Style {
    Style::default().bg(Color::Yellow)
}
