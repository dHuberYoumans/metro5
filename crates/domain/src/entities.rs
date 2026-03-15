use crate::DomainError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
}

impl Default for LogEntry {
    fn default() -> Self {
        LogEntry {
            level: LogLevel::None,
            message: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    Level(LogLevel),
}

impl Filter {
    pub fn matches(&self, log: &LogEntry) -> bool {
        match self {
            Filter::Level(level) => log.level == *level,
        }
    }
}

impl FromStr for Filter {
    type Err = DomainError;
    fn from_str(filter: &str) -> std::result::Result<Self, Self::Err> {
        match filter.to_lowercase().as_str() {
            "log" => Ok(Filter::Level(LogLevel::Log)),
            "info" => Ok(Filter::Level(LogLevel::Info)),
            "warn" => Ok(Filter::Level(LogLevel::Warn)),
            "error" => Ok(Filter::Level(LogLevel::Error)),
            _ => Err(DomainError::InvalidFilter),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogLevel {
    None,
    Info,
    Warn,
    Error,
    Log,
}

impl FromStr for LogLevel {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "log" => Ok(LogLevel::Log),
            "" => Ok(LogLevel::None),
            _ => Err(DomainError::InvalidLogLevel),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "[INFO]"),
            LogLevel::Warn => write!(f, "[WARN]"),
            LogLevel::Error => write!(f, "[ERROR]"),
            LogLevel::Log => write!(f, "[LOG]"),
            LogLevel::None => write!(f, ""),
        }
    }
}

impl LogLevel {
    pub fn marker(self) -> &'static str {
        match self {
            LogLevel::Info => " INFO ",
            LogLevel::Log => " LOG ",
            LogLevel::Warn => " WARN ",
            LogLevel::Error => " ERROR ",
            LogLevel::None => "",
        }
    }

    pub fn all() -> &'static [LogLevel] {
        &[
            LogLevel::Info,
            LogLevel::Log,
            LogLevel::Warn,
            LogLevel::Error,
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn filter_matches_log_level() {
        let log = LogEntry {
            level: LogLevel::Info,
            message: "match".to_string(),
        };
        let filter = Filter::Level(LogLevel::Info);
        assert!(filter.matches(&log));
        let log = LogEntry {
            level: LogLevel::Error,
            message: "no match".to_string(),
        };
        assert!(!filter.matches(&log));
    }

    #[test]
    pub fn log_level_marker_formats_correctly() {
        let level = LogLevel::Info;
        assert!(level.marker().contains(" INFO "));
        let level = LogLevel::None;
        assert!(level.marker().is_empty());
    }

    #[test]
    pub fn returns_all_log_level() {
        let all_levels = [
            LogLevel::Info,
            LogLevel::Log,
            LogLevel::Warn,
            LogLevel::Error,
        ];
        assert_eq!(LogLevel::all(), all_levels);
    }

    #[test]
    fn parses_log_levels() {
        assert_eq!("Info".parse::<LogLevel>().unwrap(), LogLevel::Info);
    }
}
