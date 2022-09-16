
use std::io;
use crate::trace::app::App;
use crate::trace::ui::renderer::*;
use crate::error::{Result, TraceError};

use tui::{Terminal, Frame};
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Tabs};
use tui::layout::{Direction, Layout, Rect, Constraint};
use tui::style::{Color, Style};

pub fn render<B: Backend>(t: &mut Terminal<B>, app: &App) -> Result<()> {
    t.draw(|mut f| {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        render_tab_bar(&mut f, app, sub_areas[0]);
        #[allow(clippy::single_match)]
        match app.tabs.selection {
            0 => {
                system_tab::render_system_tab(&mut f, app, sub_areas[1]);
            }
            _ => {}
        };
    }).map_err(|e| TraceError::IoError(e.to_string()))
}


fn render_tab_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let tabs = Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .titles(&app.tabs.titles)
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.selection);

    f.render_widget(tabs, area);
}
