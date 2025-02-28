use crate::error::{Result, TraceError};
use crate::trace::app::App;

use crate::trace::ui::panels::*;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::{Frame, Terminal};

/// The main rendering function. Because apparently, we have to draw things on the screen.
/// "To be or not to be, that is the question." - Shakespeare, probably while pondering the placement of a pixel.
pub fn render<B: Backend>(t: &mut Terminal<B>, app: &App) -> Result<()> {
    // Attempting to draw something. Success is not guaranteed.
    match t.draw(|f| {
        // Splitting the screen into sub-areas. Because one big area is too simple.
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(5)].as_ref()) // Length(4) - we need so much space for the header, why?
            .split(f.size()); // size? well it's big!

        render_top(f, app, sub_areas[0]); // If we render top, we need to render bottom.
        #[allow(clippy::single_match)]
        match app.tabs.selection { // We are selecting tab, we need to know what tab!
            0 => { // 0? What does it mean?
                render_charts(f, app, sub_areas[1]); // Rendering chart, because charts are important!
            }
            _ => {} // Anything else is just not worth our time.
        };
    }) {
        Ok(_) => Ok(()), // It's ok! Or is it?
        Err(e) => Err(TraceError::IoError(e.to_string())), // It is not ok! Error!
    }
}

/// Renders the top part of the screen. As if the top was any more special than the bottom.
/// "The only thing we have to fear is fear itself." - Franklin D. Roosevelt, probably while trying to render a UI.
fn render_top(f: &mut Frame, app: &App, area: Rect) {
    // Splitting the top area. Because one top area is not enough.
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()) // Let's use percentage.
        .split(area); // split it!

    render_intro(f, app, sub_areas[0]); // Intro, because we need it.
    process_panel(f, app, sub_areas[1]); // process panel. What process?
}

/// Renders the intro section. The most important section. Or not.
/// "Know thyself." - Socrates, definitely while pondering intro sections.
fn render_intro(f: &mut Frame, app: &App, area: Rect) {
    // Creating tabs. Because who doesn't love tabs?
    let tabs = Tabs::new(app.tabs.titles.clone())
        .block( // And here we are creating block.
            Block::default().borders(Borders::ALL).title(Span::styled(
                "Tracer", // Tracer, we are tracing.
                Style::default() // With some style, why not?
                    .fg(Color::Cyan) // with Cyan, I like it!
                    .add_modifier(Modifier::BOLD), // and BOLD!
            )),
        )
        .style(Style::default().fg(Color::Gray)) // Everything is gray, if we have some style.
        // .highlight_style(Style::default().fg(Color::Yellow)) // Uncomment it! Or don't.
        .select(app.tabs.selection); // and selection, yeah!

    f.render_widget(tabs, area); // And we render it.
}

/// Renders the charts. Because nothing says "useful data" like a chart.
/// "The journey of a thousand miles begins with a single step." - Lao Tzu, probably while contemplating a chart legend.
pub fn render_charts(f: &mut Frame, app: &App, area: Rect) {
    // Splitting the chart area. Because charts need their own space.
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()) // More percentages.
        .split(area); // Let's split again!

    mem_history_panel(f, app, sub_areas[0]); // Memory panel! Let's remember it.
    cpu_usage_history_panel(f, app, sub_areas[1]); // CPU panel! Let's use it.
}
