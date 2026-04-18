use std::str::FromStr;

use crate::errors::ApplicationError;
use domain::{DomainError, entities::*};

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

impl Default for Command {
    fn default() -> Self {
        Self::new()
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
    ShowManual,
    Scroll(Scroll),
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
            Some("q") | Some("quit") => Ok(AppCommand::QuitApp),
            Some("filter") => {
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
            Some("search") => {
                let query = parts.collect::<Vec<&str>>().join(" ");
                if query.is_empty() {
                    Ok(AppCommand::ResetQuery)
                } else {
                    Ok(AppCommand::SetQuery(query))
                }
            }
            Some("help") | Some("h") => Ok(AppCommand::ShowHelp),
            Some("manual") | Some("man") => Ok(AppCommand::ShowManual),
            _ => Err(ApplicationError::UnknownCommand),
        }
    }
}
