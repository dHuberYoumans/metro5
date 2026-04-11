use crate::styles::{level_style, search_result_style};
use app::app::{App, Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Position,
    text::{Line, Span},
    widgets::Clear,
};
use strip_ansi_escapes::strip;

use crate::widgets::*;

pub fn render(frame: &mut Frame, app: &App) {
    let logs = get_logs(app);
    let paragraph = main_window(logs);
    if app.mode == Mode::Command {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(frame.area());
        frame.render_widget(Clear, frame.area());
        frame.render_widget(paragraph, chunks[0]);
        frame.render_widget(commandline(app), chunks[1]);
        frame.set_cursor_position(Position {
            x: chunks[1].x + app.command.get_raw_len() as u16 + 1,
            y: chunks[1].y + 1,
        });
    } else if app.mode == Mode::Search {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(frame.area());
        frame.render_widget(Clear, frame.area());
        frame.render_widget(paragraph, chunks[0]);
        frame.render_widget(commandline(app), chunks[1]);
        frame.set_cursor_position(Position {
            x: chunks[1].x + app.query.get_raw_len() as u16 + 1,
            y: chunks[1].y + 1,
        });
    } else {
        frame.render_widget(Clear, frame.area());
        frame.render_widget(paragraph, frame.area());
    };
}

fn get_logs(app: &App) -> Vec<Line<'_>> {
    let logs = if let Some(_filter) = app.filter {
        &app.filtered
    } else if app.query.is_some() {
        &app.query_result
    } else {
        &app.logs
    };
    let query = app.query.get();
    let parsed: Vec<Line> = logs
        .iter()
        .map(|log| {
            let level_span = Span::styled(format!("{}", log.level), level_style(&log.level));
            let msg_span = if app.mode == Mode::Search && query.is_some() {
                if let Some(query) = query {
                    highlighted_search(&log.message, query)
                } else {
                    Vec::new()
                }
            } else {
                let clean_msg = strip(&log.message);
                vec![Span::raw(String::from_utf8_lossy(&clean_msg).into_owned())]
            };
            let mut line = Line::from(vec![level_span, Span::raw(" ")]);
            line.extend(msg_span.iter().cloned());
            line
        })
        .collect();
    parsed
}

fn highlighted_search<'a>(message: &'a str, pattern: &str) -> Vec<Span<'a>> {
    let mut line: Vec<Span> = Vec::new();
    let mut parts = message.split(pattern).peekable();
    while let Some(part) = parts.next() {
        line.push(Span::raw(part));
        if parts.peek().is_some() {
            line.push(Span::styled(pattern.to_string(), search_result_style()));
        }
    }
    line
}
