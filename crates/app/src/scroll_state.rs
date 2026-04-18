#[derive(Debug, Default)]
pub struct ScrollState {
    offset: Offset,
    size: Option<Size>,
    page_size: Option<Size>,
}

impl ScrollState {
    pub fn set_offset(&mut self, offset: Offset) {
        self.offset = offset;
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = Some(size);
    }

    pub fn set_page_size(&mut self, page_size: Size) {
        self.page_size = Some(page_size);
    }

    pub fn get_offset(&self) -> Offset {
        self.offset
    }

    pub fn get_size(&self) -> Option<Size> {
        self.size
    }

    pub fn get_page_size(&self) -> Option<Size> {
        self.page_size
    }

    pub fn scroll_up(&mut self) {
        self.offset.y = self.offset.y.saturating_sub(1)
    }

    pub fn scroll_up_by_half_page(&mut self) {
        if let Some(page) = self.page_size {
            self.offset.y = self.offset.y.saturating_sub(page.height / 2)
        };
    }

    pub fn scroll_down_by_half_page(&mut self) {
        if let Some(page) = self.page_size {
            self.offset.y = self.offset.y.saturating_add(page.height / 2)
        };
    }

    pub fn scroll_down(&mut self) {
        self.offset.y = self.offset.y.saturating_add(1)
    }

    pub fn scroll_to_top(&mut self) {
        self.offset.y = 0;
    }

    pub fn scroll_to_bottom(&mut self) {
        let page_height = self.page_size.map_or(0, |page_size| page_size.height);
        let bottom = self
            .size
            .map_or(u16::MAX, |size| size.height.saturating_sub(page_height - 1));
        self.offset.y = bottom;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Offset {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
