use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    prelude::CrosstermBackend,
    text::{Line, Span},
};
use std::io::Stdout;
use strip_ansi_escapes::strip;

use crate::{
    errors::TuiError,
    help::Help,
    styles::{SEARCH_RESULT_STYLE, level_style},
    widgets::Monitor,
};
use app::{app::App, app_state::Mode, scroll_state::Size};

pub mod errors;
mod help;
mod locales;
mod styles;
mod ui;
mod widgets;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn init() -> Result<Self, TuiError> {
        let mut stdout = std::io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn restore(&mut self) -> Result<(), TuiError> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<(), TuiError> {
        self.terminal.draw(|frame| {
            let help = Help::new(" Help ");
            app.scroll_state.set_page_size(Size {
                width: frame.area().width,
                height: frame.area().height,
            });
            app.scroll_state.set_size(Size {
                width: frame.area().width,
                height: app.state.logs.len() as u16,
            });
            app.help_state
                .set_number_of_sections(help.number_of_sections());
            let error = app.state.get_error().map(|error| error.to_string());
            let scroll_offset = app.scroll_state.get_offset();
            let log_lines = get_logs(app);
            let pending_key = app.state.pending_key;
            let monitor = Monitor {
                log_lines,
                scroll_offset,
                pending_key,
                error,
            };
            ui::render(frame, app, help.clone(), monitor);
        })?;
        Ok(())
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
            line.push(Span::styled(pattern.to_string(), SEARCH_RESULT_STYLE));
        }
    }
    line
}

pub fn panic_hook_init() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));
}
