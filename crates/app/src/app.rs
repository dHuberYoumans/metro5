use crate::{
    app_state::{AppState, Mode},
    commands::*,
    errors::ApplicationError,
    events::AppEvent,
    scroll_state::ScrollState,
};
use domain::{
    parse_log,
    process::{Key, ProcessEvent, StreamKind},
};

pub struct App {
    pub scroll_state: ScrollState,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState::default(),
            scroll_state: ScrollState::default(),
        }
    }

    pub fn scroll(&mut self, scroll: Scroll) {
        match scroll {
            Scroll::Up => self.scroll_state.scroll_up(),
            Scroll::Down => self.scroll_state.scroll_down(),
            Scroll::Top => self.scroll_state.scroll_to_top(),
            Scroll::Bottom => self.scroll_state.scroll_to_bottom(),
            Scroll::UpByHalfPage => self.scroll_state.scroll_up_by_half_page(),
            Scroll::DownByHalfPage => self.scroll_state.scroll_down_by_half_page(),
        }
    }

    pub fn handle_process_event(
        &mut self,
        process_event: ProcessEvent,
    ) -> Result<Option<AppCommand>, ApplicationError> {
        // could handle stdout and stderr separately
        match process_event {
            ProcessEvent::Stream(stream) => match stream.stream {
                StreamKind::Stdout => self.handle_stream(AppEvent::LogReceived(stream.line)),
                StreamKind::Stderr => self.handle_stream(AppEvent::LogReceived(stream.line)),
            },
            ProcessEvent::Key(key) => self.handle_key_press(key),
        }
    }

    pub fn handle_stream(
        &mut self,
        event: AppEvent,
    ) -> Result<Option<AppCommand>, ApplicationError> {
        match event {
            AppEvent::LogReceived(log) => {
                self.handle_log_event(&log);
                Ok(None)
            }
        }
    }

    fn handle_key_press(&mut self, key: Key) -> Result<Option<AppCommand>, ApplicationError> {
        match key {
            Key::CtrlC => Ok(Some(AppCommand::QuitApp)),
            Key::CtrlU => Ok(Some(AppCommand::Scroll(Scroll::UpByHalfPage))),
            Key::CtrlD => Ok(Some(AppCommand::Scroll(Scroll::DownByHalfPage))),
            Key::Backspace => self.handle_backspace(),
            Key::Esc => self.handle_esc(),
            Key::Enter => {
                if matches!(self.state.mode, Mode::Command | Mode::Search) {
                    self.handle_enter()
                } else {
                    Ok(None)
                }
            }
            Key::Char(ch) => self.handle_char_input(ch),
        }
    }

    fn handle_backspace(&mut self) -> Result<Option<AppCommand>, ApplicationError> {
        //trim leading : resp. leading /
        if self.state.mode == Mode::Command {
            if self.state.command.raw[1..].is_empty() {
                self.state.mode = Mode::Normal;
            } else {
                self.state.command.raw.pop();
            }
        };
        if self.state.mode == Mode::Search {
            if self.state.query.raw[1..].is_empty() {
                self.state.mode = Mode::Normal;
            } else {
                self.state.query.raw.pop();
                self.state.apply_query();
                return Ok(Some(AppCommand::SetQuery(
                    self.state.query.get().unwrap().to_string(),
                )));
            }
        }
        Ok(None)
    }

    fn handle_esc(&mut self) -> Result<Option<AppCommand>, ApplicationError> {
        match self.state.mode {
            Mode::Search => {
                self.state.query.clear();
                self.state.set_mode(Mode::Normal);
            }
            Mode::Command => {
                self.state.command.clear();
                self.state.set_mode(Mode::Normal);
            }
            _ => self.state.clear_state(),
        }
        Ok(None)
    }

    fn handle_enter(&mut self) -> Result<Option<AppCommand>, ApplicationError> {
        let cmd: Option<AppCommand> = match self.state.mode {
            Mode::Command => self.state.command.get_cmd(),
            Mode::Search => self
                .state
                .query
                .get()
                .map(|query| AppCommand::SetQuery(query.to_string())),
            Mode::Help => Some(AppCommand::ShowHelp),
            Mode::Manual => Some(AppCommand::ShowManual),
            _ => None,
        };
        self.state.mode = Mode::Normal;
        Ok(cmd)
    }

    fn handle_char_input(&mut self, ch: char) -> Result<Option<AppCommand>, ApplicationError> {
        if !matches!(self.state.mode, Mode::Command | Mode::Search) {
            match ch {
                'r' | 'd' => {
                    return Ok(Some(AppCommand::SendToMetro(ch.to_string())));
                }
                'k' => return Ok(Some(AppCommand::Scroll(Scroll::Up))),
                'j' => return Ok(Some(AppCommand::Scroll(Scroll::Down))),
                'G' => return Ok(Some(AppCommand::Scroll(Scroll::Bottom))),
                'g' => {
                    if self.state.pending_key.key == Some('g') {
                        self.state.pending_key.reset();
                        return Ok(Some(AppCommand::Scroll(Scroll::Top)));
                    } else {
                        self.state.pending_key.set('g');
                    }
                }
                ':' => {
                    self.state.mode = Mode::Command;
                    self.state.command.raw = ":".to_string();
                    return Ok(None);
                }
                '/' => {
                    self.state.mode = Mode::Search;
                    self.state.query.raw = "/".to_string();
                    return Ok(None);
                }
                _ => return Ok(None),
            }
        }
        if self.state.mode == Mode::Command {
            self.state.command.raw.push(ch);
        }
        if self.state.mode == Mode::Search {
            self.state.query.raw.push(ch);
            self.state.apply_query();
            return Ok(Some(AppCommand::SetQuery(
                self.state.query.get().unwrap().to_string(),
            )));
        }
        Ok(None)
    }

    fn handle_log_event(&mut self, log: &str) {
        let parsed = parse_log(log);
        if let Some(filter) = &self.state.filter {
            if filter.matches(&parsed) {
                self.state.filtered.push(parsed.clone());
            }
        } else {
            self.state.filtered.push(parsed.clone());
        }
        self.state.logs.push(parsed);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scroll_state::Offset;
    use domain::{entities::*, errors::DomainError};

    #[test]
    fn it_should_request_quit() {
        assert_eq!("q".parse::<AppCommand>().unwrap(), AppCommand::QuitApp);
        assert_eq!("quit".parse::<AppCommand>().unwrap(), AppCommand::QuitApp);
    }

    #[test]
    fn it_should_set_filter() {
        let filter = Filter::Level(LogLevel::Info);
        assert_eq!(
            "filter info".parse::<AppCommand>().unwrap(),
            AppCommand::SetFilter(filter)
        );
    }

    #[test]
    fn it_should_reset_filter() {
        assert_eq!(
            "filter reset".parse::<AppCommand>().unwrap(),
            AppCommand::ResetFilter
        );
    }

    #[test]
    fn it_should_err_on_invalid_filters() {
        let cmd = "filter invalid".parse::<AppCommand>();
        assert!(cmd.is_err());
        assert!(matches!(
            cmd.unwrap_err(),
            ApplicationError::Domain(DomainError::InvalidFilter)
        ));
    }

    #[test]
    fn it_should_err_for_more_than_one_arg() {
        let cmd = "filter arg1 arg2".parse::<AppCommand>();
        assert!(cmd.is_err());
        assert!(matches!(
            cmd.unwrap_err(),
            ApplicationError::Domain(DomainError::InvalidFilter)
        ));
    }

    #[test]
    fn it_should_err_on_unknown_command() {
        let cmd = "unkown".parse::<AppCommand>();
        assert!(cmd.is_err());
        assert!(matches!(cmd.unwrap_err(), ApplicationError::UnknownCommand));
    }

    #[test]
    fn it_should_set_query() {
        let query = "query";
        let query_cmd = format!("search {}", query);
        assert_eq!(
            query_cmd.parse::<AppCommand>().unwrap(),
            AppCommand::SetQuery(query.to_string())
        )
    }

    #[test]
    fn it_should_scroll_down() {
        let mut app = App::new();
        app.scroll_state.scroll_down();
        assert_eq!(app.scroll_state.get_offset().y, 1)
    }

    #[test]
    fn it_should_scroll_up() {
        let mut app = App::new();
        app.scroll_state.scroll_down();
        app.scroll_state.scroll_up();
        assert_eq!(app.scroll_state.get_offset().y, 0)
    }

    #[test]
    fn it_should_scroll_to_top() {
        let mut app = App::new();
        app.scroll_state.set_offset(Offset { x: 0, y: 42 });
        app.scroll_state.scroll_to_top();
        assert_eq!(app.scroll_state.get_offset().y, 0)
    }
}
