use crate::locales::{HelpMenu, HelpSection, HelpSections};
use crate::styles::{
    HELP_BLOCK_STYLE, HELP_HIGHLIGHT_STYLE, HELP_SELECTED_STYLE, HELP_TITLE_STYLE,
};
use ratatui::widgets::Paragraph;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, List, ListState, StatefulWidget, Widget},
};

#[derive(Debug, Clone)]
pub(crate) struct Help {
    pub title: &'static str,
    pub list_state: ListState,
    pub expanded: Option<usize>,
    sections: Vec<HelpSection>,
}

impl Help {
    pub fn new(title: &'static str) -> Self {
        let sections = HelpMenu::default()
            .add(HelpSections::GettingStarted)
            .add(HelpSections::Commands)
            .add(HelpSections::Navigation)
            .build();
        Help {
            title,
            expanded: None,
            sections,
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    fn block(&self, title: &'static str) -> Block<'_> {
        Block::default()
            .title(Line::from(title).style(HELP_TITLE_STYLE).centered())
            .borders(Borders::NONE)
            .style(HELP_BLOCK_STYLE)
    }

    pub fn number_of_sections(&self) -> usize {
        self.sections.len()
    }
}

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let area = centered_rect(50, 40, area);
        if let Some(idx) = self.expanded {
            let block = self.block(self.sections[idx].title);
            let body = Paragraph::new(self.sections[idx].text).block(block);
            body.render(area, buf);
        } else {
            let mut list_state = self.list_state;
            let block = self.block(self.title);
            let list = List::new(self.sections.iter().map(|section| section.title))
                .block(block)
                .style(HELP_SELECTED_STYLE)
                .highlight_style(HELP_HIGHLIGHT_STYLE)
                .highlight_symbol(" > ");
            StatefulWidget::render(list, area, buf, &mut list_state);
        }
    }
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
