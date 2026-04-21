use std::str::FromStr;

use crate::errors::ApplicationError;
use domain::{DomainError, entities::*};

#[derive(Debug, Default)]
pub struct Command {
    pub raw: String,
    pub cmd: Option<AppCommand>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            raw: String::new(),
            cmd: None,
        }
    }

    pub fn clear(&mut self) {
        self.raw.clear();
        self.cmd = None
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn get_cmd(&mut self) -> Option<AppCommand> {
        let _ = self.parse();
        self.cmd.clone()
    }

    pub fn get_raw_len(&self) -> usize {
        self.raw.len()
    }

    fn parse(&mut self) -> Result<(), ApplicationError> {
        let stripped = self
            .raw
            .strip_prefix(':')
            .ok_or(ApplicationError::InvalidCommandPrefix)?;
        self.cmd = Some(AppCommand::from_str(stripped)?);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppCommand {
    SendToMetro(String),
    SetFilter(Filter),
    ResetFilter,
    SetQuery(String),
    ClearState,
    ResetQuery,
    QuitApp,
    ShowHelp,
    Scroll(Scroll),
    HelpMenu(HelpCommand),
    WriteToFile(String),
    WriteAndQuit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HelpCommand {
    SelectNext,
    SelectPrev,
    ExpandSection,
    CollapseSection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scroll {
    Up,
    Down,
    Top,
    Bottom,
    UpByHalfPage,
    DownByHalfPage,
}

impl FromStr for AppCommand {
    type Err = ApplicationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let cmd = parts.next();
        match cmd {
            Some("quit") | Some("q") => Ok(AppCommand::QuitApp),
            Some("filter") | Some("f") => {
                let arg = parts
                    .next()
                    .ok_or(ApplicationError::Domain(DomainError::InvalidFilter))?;
                if parts.next().is_some() {
                    return Err(ApplicationError::Domain(DomainError::InvalidFilter));
                }
                if arg == "reset" {
                    return Ok(AppCommand::ResetFilter);
                }
                match arg.parse::<LogLevel>() {
                    Ok(log_level) => Ok(AppCommand::SetFilter(Filter::Level(log_level))),
                    Err(_) => Err(ApplicationError::Domain(DomainError::InvalidFilter)),
                }
            }
            Some("search") | Some("s") => {
                let query = parts.collect::<Vec<&str>>().join(" ");
                if query.is_empty() {
                    Ok(AppCommand::ResetQuery)
                } else {
                    Ok(AppCommand::SetQuery(query))
                }
            }
            Some("write") | Some("w") => {
                let path = parts.next().ok_or(ApplicationError::UnknownCommand)?;
                if parts.next().is_some() {
                    return Err(ApplicationError::TooManyArguments);
                }
                Ok(AppCommand::WriteToFile(path.to_string()))
            }
            Some("wq") => {
                if parts.next().is_some() {
                    return Err(ApplicationError::TooManyArguments);
                }
                Ok(AppCommand::WriteAndQuit)
            }
            Some("help") | Some("h") => Ok(AppCommand::ShowHelp),
            _ => Err(ApplicationError::UnknownCommand),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_parse_quit() {
        let expected = AppCommand::QuitApp;
        let cmd = "quit";
        assert_eq!(AppCommand::from_str(cmd).unwrap(), expected);
    }

    #[test]
    fn it_should_parse_filter() {
        let log_level = LogLevel::Info;
        let expected = AppCommand::SetFilter(Filter::Level(log_level));
        let cmd = "filter info";
        assert_eq!(AppCommand::from_str(cmd).unwrap(), expected);
    }

    #[test]
    fn it_should_throw_for_invalid_filter() {
        let cmd = "filter foo";
        let err = AppCommand::from_str(cmd);
        assert!(err.is_err());
    }

    #[test]
    fn it_should_parse_search() {
        let query = "alice and bob".to_string();
        let cmd = format!("search {}", query);
        let expected = AppCommand::SetQuery(query);
        assert_eq!(AppCommand::from_str(&cmd).unwrap(), expected);
    }

    #[test]
    fn it_should_parse_write() {
        let path = "/tmp/path".to_string();
        let cmd = format!("write {}", path);
        let expected = AppCommand::WriteToFile(path);
        assert_eq!(AppCommand::from_str(&cmd).unwrap(), expected);
    }

    #[test]
    fn it_should_err_for_write_with_many_arguments() {
        let args = "foo bar baz".to_string();
        let cmd = format!("write {}", args);
        let err = AppCommand::from_str(&cmd);
        assert!(err.is_err());
    }

    #[test]
    fn it_should_parse_write_and_quit() {
        let cmd = "wq";
        let expected = AppCommand::WriteAndQuit;
        assert_eq!(AppCommand::from_str(cmd).unwrap(), expected);
    }

    #[test]
    fn it_should_err_for_write_and_quit_with_many_arguments() {
        let cmd = "wq foo";
        let err = AppCommand::from_str(cmd);
        assert!(err.is_err());
    }

    #[test]
    fn it_should_parse_help() {
        let expected = AppCommand::ShowHelp;
        let cmd = "help";
        assert_eq!(AppCommand::from_str(cmd).unwrap(), expected);
    }
}
