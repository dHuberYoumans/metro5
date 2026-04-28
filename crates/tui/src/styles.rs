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

pub(crate) const TITLE_STYLE: Style = Style::new().bold().fg(Color::Indexed(214)); // 8-bit orange

pub(crate) const COMMAND_LINE_STYLE: Style = Style::new().fg(Color::Indexed(214)); // 8-bit orange

pub(crate) const BORDER_STYLE: Style = Style::new().bold().fg(Color::Blue); // 8-bit blue

pub const SEARCH_RESULT_STYLE: Style = Style::new().bg(Color::Yellow);

pub(crate) const PENDING_KEY_STYLE: Style = Style::new().fg(Color::Blue);

pub(crate) const ERROR_STYLE: Style = Style::new().fg(Color::LightRed);

pub(crate) const HELP_BLOCK_STYLE: Style = Style::new().bg(Color::DarkGray);

pub(crate) const HELP_TITLE_STYLE: Style = Style::new().bold();

pub(crate) const HELP_SELECTED_STYLE: Style = Style::new().fg(Color::White);

pub(crate) const HELP_HIGHLIGHT_STYLE: Style = Style::new().add_modifier(Modifier::REVERSED);
