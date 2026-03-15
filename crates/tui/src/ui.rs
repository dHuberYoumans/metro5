use crate::styles::level_style;
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
    } else {
        frame.render_widget(Clear, frame.area());
        frame.render_widget(paragraph, frame.area());
    };
}

fn get_logs(app: &App) -> Vec<Line<'_>> {
    let logs = if let Some(_filter) = app.filter {
        &app.filtered
    } else {
        &app.logs
    };
    let parsed: Vec<Line> = logs
        .iter()
        .map(|log| {
            let level_span = Span::styled(format!("{}", log.level), level_style(&log.level));
            let clean_msg = strip(&log.message);
            let msg_span = Span::raw(String::from_utf8_lossy(&clean_msg).into_owned());
            Line::from(vec![level_span, Span::raw(" "), msg_span])
        })
        .collect();
    parsed
}
