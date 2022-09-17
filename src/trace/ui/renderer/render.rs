use crate::error::{Result, TraceError};
use crate::trace::app::App;

use crate::trace::ui::panels::*;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs};
use tui::{Frame, Terminal};

pub fn render<B: Backend>(t: &mut Terminal<B>, app: &App) -> Result<()> {
    t.draw(|mut f| {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(5)].as_ref())
            .split(f.size());

        render_top(&mut f, app, sub_areas[0]);
        #[allow(clippy::single_match)]
        match app.tabs.selection {
            0 => {
                render_charts(&mut f, app, sub_areas[1]);
            }
            _ => {}
        };
    })
    .map_err(|e| TraceError::IoError(e.to_string()))
}

fn render_top<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    render_intro(f, app, sub_areas[0]);
    process_panel(f, app, sub_areas[1]);
}

fn render_intro<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let tabs = Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("Tracer"))
        .titles(&app.tabs.titles)
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.selection);

    f.render_widget(tabs, area);
}

pub fn render_charts<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    mem_history_panel(f, app, sub_areas[0]);
    cpu_usage_history_panel(f, app, sub_areas[1]);
}
