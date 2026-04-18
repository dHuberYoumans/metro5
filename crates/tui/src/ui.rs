use crate::styles::{level_style, search_result_style};
use app::{app::App, app_state::Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Position,
    text::{Line, Span},
    widgets::{Clear, Widget},
};
use strip_ansi_escapes::strip;

use crate::widgets::*;

pub fn render(frame: &mut Frame, app: &App) {
    let logs = get_logs(app);
    let monitor = Monitor {
        log_lines: logs,
        scroll_offset: app.scroll_state.get_offset(),
        pending_key: app.state.pending_key,
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());
    match app.state.mode {
        Mode::Command => {
            let commandline = Commandline {
                title: " cmd ".to_string(),
                text: app.state.command.get_raw().to_string(),
            };
            Clear.render(frame.area(), frame.buffer_mut());
            monitor.render(chunks[0], frame.buffer_mut());
            commandline.render(chunks[1], frame.buffer_mut());
            frame.set_cursor_position(Position {
                x: chunks[1].x + app.state.command.get_raw_len() as u16 + 1,
                y: chunks[1].y + 1,
            });
        }
        Mode::Search => {
            let commandline = Commandline {
                title: " search ".to_string(),
                text: app.state.query.get_raw().to_string(),
            };
            Clear.render(frame.area(), frame.buffer_mut());
            monitor.render(chunks[0], frame.buffer_mut());
            commandline.render(chunks[1], frame.buffer_mut());
            frame.set_cursor_position(Position {
                x: chunks[1].x + app.state.query.get_raw_len() as u16 + 1,
                y: chunks[1].y + 1,
            });
        }
        Mode::Help => {
            let help = Help {
                title: "Help".to_string(),
                text: help_text(),
            };
            help.render(frame.area(), frame.buffer_mut());
        }
        Mode::Manual => {
            let man = Help {
                title: "Commands".to_string(),
                text: man_text(),
            };
            man.render(frame.area(), frame.buffer_mut());
        }
        _ => {
            Clear.render(frame.area(), frame.buffer_mut());
            monitor.render(frame.area(), frame.buffer_mut());
        }
    }
}

fn get_logs(app: &App) -> Vec<Line<'_>> {
    let logs = if let Some(_filter) = app.state.filter {
        &app.state.filtered
    } else if app.state.query.is_some() {
        &app.state.query_result
    } else {
        &app.state.logs
    };
    let query = app.state.query.get();
    let parsed: Vec<Line> = logs
        .iter()
        .map(|log| {
            let level_span = Span::styled(format!("{}", log.level), level_style(&log.level));
            let msg_span = if app.state.mode == Mode::Search && query.is_some() {
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
