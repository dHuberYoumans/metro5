use crate::help::Help;
use app::{app::App, app_state::Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Position,
    widgets::{Clear, Widget},
};

use crate::widgets::*;

pub fn render(frame: &mut Frame, app: &App, mut help: Help, monitor: Monitor) {
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
            help.list_state.select(Some(app.help_state.get_selected()));
            help.expanded = app.help_state.get_expanded();
            help.render(frame.area(), frame.buffer_mut());
        }
        _ => {
            Clear.render(frame.area(), frame.buffer_mut());
            monitor.render(frame.area(), frame.buffer_mut());
        }
    }
}
