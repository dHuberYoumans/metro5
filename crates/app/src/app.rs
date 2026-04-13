use crate::{commands::*, errors::ApplicationError, events::AppEvent, queries::Query};
use domain::{
    entities::*,
    parse_log,
    process::{Key, ProcessEvent, StreamKind},
};

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Search,
    Command,
    Help,
    Manual,
}

pub struct App {
    pub logs: Vec<LogEntry>,
    pub filter: Option<Filter>,
    pub filtered: Vec<LogEntry>,
    pub query: Query,
    pub query_result: Vec<LogEntry>,
    pub mode: Mode,
    pub command: Command,
}

impl App {
    pub fn new() -> Self {
        App {
            logs: Vec::new(),
            filter: None,
            filtered: Vec::new(),
            query: Query::new(),
            query_result: Vec::new(),
            mode: Mode::Normal,
            command: Command::new(),
        }
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = Some(filter)
    }

    pub fn reset_filter(&mut self) {
        self.filter = None
    }

    pub fn apply_filter(&mut self) {
        if let Some(filter) = self.filter {
            self.filtered = self
                .logs
                .iter()
                .filter(|log| filter.matches(log))
                .cloned()
                .collect();
        };
    }

    pub fn set_query(&mut self, query: String) {
        self.query.set(query)
    }

    pub fn reset_query(&mut self) {
        self.query.clear();
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode
    }

    pub fn clear_state(&mut self) {
        self.reset_query();
        self.reset_filter();
        self.set_mode(Mode::Normal);
    }

    pub fn apply_query(&mut self) {
        if let Some(result) = self.query.apply(&self.logs) {
            self.query_result = result;
        }
    }

    pub fn show_help(&mut self) {
        self.set_mode(Mode::Help);
    }

    pub fn show_man(&mut self) {
        self.set_mode(Mode::Manual);
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
            Key::Backspace => self.handle_backspace(),
            Key::Esc => self.handle_esc(),
            Key::Enter => {
                if matches!(self.mode, Mode::Command | Mode::Search) {
                    self.handle_enter()
                } else {
                    Ok(None)
                }
            }
            Key::Char(ch) => self.handle_char_input(ch),
        }
    }

    fn handle_backspace(&mut self) -> Result<Option<AppCommand>, ApplicationError> {
        //trim leading : resp leading /
        if self.mode == Mode::Command {
            if self.command.raw[1..].is_empty() {
                self.mode = Mode::Normal;
            } else {
                self.command.raw.pop();
            }
        };
        if self.mode == Mode::Search {
            if self.query.raw[1..].is_empty() {
                self.mode = Mode::Normal;
            } else {
                self.query.raw.pop();
                self.apply_query();
                return Ok(Some(AppCommand::SetQuery(
                    self.query.get().unwrap().to_string(),
                )));
            }
        }
        Ok(None)
    }

    fn handle_esc(&mut self) -> Result<Option<AppCommand>, ApplicationError> {
        match self.mode {
            Mode::Search => {
                self.query.clear();
                self.set_mode(Mode::Normal);
            }
            Mode::Command => {
                self.command.clear();
                self.set_mode(Mode::Normal);
            }
            _ => self.clear_state(),
        }
        Ok(None)
    }

    fn handle_enter(&mut self) -> Result<Option<AppCommand>, ApplicationError> {
        let cmd: Option<AppCommand> = match self.mode {
            Mode::Command => self.command.get_cmd(),
            Mode::Search => self
                .query
                .get()
                .map(|query| AppCommand::SetQuery(query.to_string())),
            Mode::Help => Some(AppCommand::ShowHelp),
            Mode::Manual => Some(AppCommand::ShowManual),
            _ => None,
        };
        self.mode = Mode::Normal;
        Ok(cmd)
    }

    fn handle_char_input(&mut self, ch: char) -> Result<Option<AppCommand>, ApplicationError> {
        if !matches!(self.mode, Mode::Command | Mode::Search) {
            match ch {
                'r' | 'j' | 'd' => {
                    return Ok(Some(AppCommand::SendToMetro(ch.to_string())));
                }
                ':' => {
                    self.mode = Mode::Command;
                    self.command.raw = ":".to_string();
                    return Ok(None);
                }
                '/' => {
                    self.mode = Mode::Search;
                    self.query.raw = "/".to_string();
                    return Ok(None);
                }
                _ => return Ok(None),
            }
        }
        if self.mode == Mode::Command {
            self.command.raw.push(ch);
        }
        if self.mode == Mode::Search {
            self.query.raw.push(ch);
            self.apply_query();
            return Ok(Some(AppCommand::SetQuery(
                self.query.get().unwrap().to_string(),
            )));
        }
        Ok(None)
    }

    fn handle_log_event(&mut self, log: &str) {
        let parsed = parse_log(log);
        if let Some(filter) = &self.filter {
            if filter.matches(&parsed) {
                self.filtered.push(parsed.clone());
            }
        } else {
            self.filtered.push(parsed.clone());
        }
        self.logs.push(parsed);
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
    use domain::errors::DomainError;

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
}
