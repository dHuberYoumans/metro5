pub mod entities;
pub mod errors;
pub mod process;

pub use errors::*;

use self::entities::*;

pub fn parse_log(log: &str) -> LogEntry {
    let mut entry = LogEntry {
        level: LogLevel::None,
        message: log.to_string(),
    };
    for level in LogLevel::all() {
        let marker = level.marker();
        if let Some(index) = log.find(marker) {
            let message = log[index + marker.len()..].trim().to_string();
            entry = LogEntry {
                level: *level,
                message,
            }
        }
    }
    entry
}

#[test]
pub fn unit_test_parse_log_with_level() {
    let log = " INFO this is an Info-log";
    let parsed = parse_log(log);
    assert_eq!(parsed.level, LogLevel::Info);
    assert!(parsed.message.contains("this is an Info-log"));
}

#[test]
pub fn unit_test_parse_log_without_level() {
    let log = " This is a log without log-level";
    let parsed = parse_log(log);
    assert_eq!(parsed.level, LogLevel::None);
    assert!(parsed.message.contains(log));
}
