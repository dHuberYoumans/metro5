use app::{app_state::PendingKey, scroll_state::Offset};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::styles::{
    border_style, command_line_style, help_style, help_title_style, pending_key_style, title_style,
};

pub(crate) struct Monitor<'a> {
    pub log_lines: Vec<Line<'a>>,
    pub scroll_offset: Offset,
    pub pending_key: PendingKey,
}

impl Widget for Monitor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(Span::styled(" Metro 5 ", title_style()));
        let pending_key = if let Some(key) = self.pending_key.get() {
            Line::from(Span::styled(format!(" {} ", key), pending_key_style()))
        } else {
            Line::default()
        };
        let block = Block::bordered()
            .title(title.right_aligned())
            .title_bottom(pending_key.right_aligned())
            .border_set(border::ROUNDED)
            .border_style(border_style());
        let body = Paragraph::new(self.log_lines)
            .block(block)
            .scroll((self.scroll_offset.y, self.scroll_offset.x));
        body.render(area, buf)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Commandline {
    pub title: String,
    pub text: String,
}

impl Widget for Commandline {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(self.title);
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED)
            .border_style(command_line_style());
        let text = Paragraph::new(Line::from(self.text)).block(block);
        text.render(area, buf)
    }
}

pub(crate) struct Help {
    pub title: String,
    pub text: String,
}

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(self.title).style(help_title_style());
        let block = Block::default()
            .title(title.centered())
            .borders(Borders::NONE)
            .style(help_style());
        let body = Paragraph::new(self.text).block(block);
        let area = centered_rect(30, 25, area);
        body.render(area, buf)
    }
}

pub fn help_text() -> String {
    String::from(
        r#"
    - ':'   -- enter command mode

    - '/'   -- search

    - 'esc' -- clear all 

    - ':man' -- show commands
    "#,
    )
}

pub fn man_text() -> String {
    String::from(
        r#"
    - ':help: | :h'         -- show help

    - ':quit | :q'          -- quit

    - ':search <pattern>'    -- search log message for <pattern>

    - ':filter <log level>' -- filter for <log level> (info/log/warn/error)

    "#,
    )
}

pub fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(rect);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
