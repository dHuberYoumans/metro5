use crate::styles::{
    BORDER_STYLE, COMMAND_LINE_STYLE, ERROR_STYLE, PENDING_KEY_STYLE, TITLE_STYLE,
};
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
    pub error: Option<String>,
}

impl Widget for Monitor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(Span::styled(" Metro 5 ", TITLE_STYLE));
        let pending_key = if let Some(key) = self.pending_key.get() {
            Line::from(Span::styled(format!(" {} ", key), PENDING_KEY_STYLE))
        } else {
            Line::default()
        };
        let error = if let Some(err) = self.error {
            Line::from(Span::styled(format!(" {} ", err), ERROR_STYLE))
        } else {
            Line::default()
        };
        let block = Block::bordered()
            .title(title.right_aligned())
            .title_bottom(pending_key.right_aligned())
            .title_bottom(error.left_aligned())
            .border_set(border::ROUNDED)
            .border_style(BORDER_STYLE);
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
            .border_style(COMMAND_LINE_STYLE);
        let text = Paragraph::new(Line::from(self.text)).block(block);
        text.render(area, buf)
    }
}
