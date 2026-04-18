use domain::entities::LogEntry;

use crate::errors::ApplicationError;

#[derive(Debug, Default)]
pub struct Query {
    pub raw: String,
    pub query: Option<String>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            raw: String::new(),
            query: None,
        }
    }

    pub fn clear(&mut self) {
        self.raw.clear();
        self.query = None
    }

    pub fn apply(&mut self, logs: &[LogEntry]) -> Option<Vec<LogEntry>> {
        let _ = self.parse();
        if self.query.is_some() {
            let result: Vec<LogEntry> = logs
                .iter()
                .filter(|log| self.matches(log))
                .cloned()
                .collect();
            return Some(result);
        };
        None
    }

    pub fn matches(&mut self, log: &LogEntry) -> bool {
        if let Some(query) = self.get() {
            log.message.contains(query)
        } else {
            false
        }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn get_raw_len(&self) -> usize {
        self.raw.len()
    }

    pub fn get(&self) -> Option<&str> {
        self.query.as_deref()
    }

    pub fn set(&mut self, query: String) {
        self.query = Some(query)
    }

    pub fn is_some(&self) -> bool {
        self.query.is_some()
    }

    fn parse(&mut self) -> Result<(), ApplicationError> {
        let stripped = self
            .raw
            .strip_prefix('/')
            .ok_or(ApplicationError::InvalidCommandPrefix)?;
        self.query = Some(stripped.to_string());
        Ok(())
    }
}
