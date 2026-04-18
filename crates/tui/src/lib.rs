use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::Stdout;

use crate::errors::TuiError;
use app::{app::App, scroll_state::Size};

pub mod errors;
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
            app.scroll_state.set_page_size(Size {
                width: frame.area().width,
                height: frame.area().height,
            });
            app.scroll_state.set_size(Size {
                width: frame.area().width,
                height: app.logs.len() as u16,
            });
            ui::render(frame, app);
        })?;
        Ok(())
    }
}

pub fn panic_hook_init() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));
}
