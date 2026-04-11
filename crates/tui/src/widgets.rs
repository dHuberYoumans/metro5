use app::app::{App, Mode};
use ratatui::{
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::styles::{border_style, command_line_style, title_style};

pub fn main_window(log_lines: Vec<Line>) -> Paragraph {
    let title = Line::from(Span::styled(" Metro 5 ", title_style()));
    let block = Block::bordered()
        .title(title.right_aligned())
        .border_set(border::ROUNDED)
        .border_style(border_style());
    Paragraph::new(log_lines).block(block)
}

pub fn commandline<'a>(app: &'a App) -> Paragraph<'a> {
    let title = match app.mode {
        Mode::Command => Line::from(" cmd "),
        Mode::Search => Line::from(" search "),
        _ => Line::from(""),
    };
    let block = Block::bordered()
        .title(title.centered())
        .border_set(border::ROUNDED)
        .border_style(command_line_style());
    let text = if app.mode == Mode::Search {
        Line::from(app.query.get_raw())
    } else {
        Line::from(app.command.get_raw())
    };
    Paragraph::new(text).block(block)
}
