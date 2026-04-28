use crate::{commands::*, errors::ApplicationError, queries::Query};
use domain::entities::*;

#[derive(Debug, Default)]
pub struct AppState {
    pub logs: Vec<LogEntry>,
    pub filter: Option<Filter>,
    pub filtered: Vec<LogEntry>,
    pub query: Query,
    pub query_result: Vec<LogEntry>,
    pub mode: Mode,
    pub command: Command,
    pub pending_key: PendingKey,
    pub path: Option<String>,
    pub error: Option<ApplicationError>,
}

impl AppState {
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
        self.pending_key.reset();
        self.error = None;
    }

    pub fn apply_query(&mut self) {
        if let Some(result) = self.query.apply(&self.logs) {
            self.query_result = result;
        }
    }

    pub fn show_help(&mut self) {
        self.set_mode(Mode::Help);
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn get_error(&self) -> Option<&ApplicationError> {
        self.error.as_ref()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PendingKey {
    pub key: Option<char>,
}

impl PendingKey {
    pub fn reset(&mut self) {
        self.key = None;
    }

    pub fn set(&mut self, key: char) {
        self.key = Some(key);
    }

    pub fn get(&self) -> Option<char> {
        self.key
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    Normal,
    Search,
    Command,
    Help,
}
