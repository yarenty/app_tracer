use ratatui::text::Line;

pub struct Tabs<'a> {
    pub titles: Vec<Line<'a>>,
    pub selection: usize,
}

impl Tabs<'_> {
    pub fn next(&mut self) {
        self.selection = (self.selection + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        } else {
            self.selection = self.titles.len() - 1;
        }
    }
}
