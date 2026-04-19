use crate::styles::{border_style, command_line_style, pending_key_style, title_style};
use app::{app_state::PendingKey, scroll_state::Offset};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
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
