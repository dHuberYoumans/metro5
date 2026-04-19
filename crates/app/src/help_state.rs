#[derive(Debug, Default)]
pub struct HelpState {
    number_of_sections: usize,
    selected: usize,
    expanded: Option<usize>,
}

impl HelpState {
    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1).min(self.number_of_sections - 1);
    }

    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    pub fn expand_section(&mut self) {
        self.expanded = Some(self.selected);
    }

    pub fn collase_section(&mut self) {
        self.expanded = None;
    }

    pub fn get_selected(&self) -> usize {
        self.selected
    }

    pub fn get_expanded(&self) -> Option<usize> {
        self.expanded
    }

    pub fn get_number_of_sections(&self) -> usize {
        self.number_of_sections
    }

    pub fn set_number_of_sections(&mut self, number: usize) {
        self.number_of_sections = number;
    }
}
