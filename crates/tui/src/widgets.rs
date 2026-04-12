use app::app::{App, Mode};
use ratatui::{
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::styles::{
    border_style, command_line_style, help_style, help_title_style, man_style, man_title_style,
    title_style,
};

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

pub fn help_block() -> Paragraph<'static> {
    let title = Line::from("Help").style(help_title_style());
    let block = Block::default()
        .title(title.centered())
        .borders(Borders::NONE)
        .style(help_style());
    Paragraph::new(help_text()).block(block)
}

fn help_text() -> String {
    String::from(
        r#"
    - ':'   -- enter command mode

    - '/'   -- search

    - 'esc' -- clear all 

    - ':man' -- show commands
    "#,
    )
}

pub fn man_block() -> Paragraph<'static> {
    let title = Line::from("Commands").style(man_title_style());
    let block = Block::default()
        .title(title.centered())
        .borders(Borders::NONE)
        .style(man_style());
    Paragraph::new(man_text()).block(block)
}

fn man_text() -> String {
    String::from(
        r#"
    - ':help: | :h'         -- show help

    - ':quit | :q'          -- quit

    - ':search <pattern>'    -- search log message for <pattern>

    - ':filter <log level>' -- filter for <log level> (info/log/warn/error)

    "#,
    )
}
