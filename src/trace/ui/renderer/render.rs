use crate::error::{Result, TraceError};
use crate::trace::app::App;

use crate::trace::ui::panels::*;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Tabs, Paragraph};
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
            1 => { // GPU monitoring tab
                render_gpu_panels(f, app, sub_areas[1]);
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

/// Renders GPU panels. Because GPUs are the future. Or present. Or something.
/// "The future is already here – it's just not evenly distributed." - William Gibson, probably about GPU distribution.
pub fn render_gpu_panels(f: &mut Frame, app: &App, area: Rect) {
    if !app.gpu_available {
        let no_gpu_text = Paragraph::new("GPU monitoring not available\nnvidia-smi not found or no GPUs detected")
            .block(
                Block::default()
                    .title("GPU Monitoring")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // Split the area into multiple sections for different GPU metrics
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // GPU summary
            Constraint::Min(0),     // Main metrics area
        ])
        .split(area);

    // GPU Summary at the top
    gpu_summary_panel(f, app, &app.gpu_readings, main_chunks[0]);

    // Split the main area into GPU metrics
    let metrics_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left side
            Constraint::Percentage(50), // Right side
        ])
        .split(main_chunks[1]);

    // Left side: Memory and Utilization
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Memory
            Constraint::Percentage(50), // Utilization
        ])
        .split(metrics_chunks[0]);

    gpu_memory_panel(f, app, &app.gpu_readings, left_chunks[0]);
    gpu_utilization_panel(f, app, &app.gpu_readings, left_chunks[1]);

    // Right side: Temperature, Power, and Processes
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // Temperature
            Constraint::Length(6),  // Power
            Constraint::Min(0),     // Processes
        ])
        .split(metrics_chunks[1]);

    gpu_temperature_panel(f, app, &app.gpu_readings, right_chunks[0]);
    gpu_power_panel(f, app, &app.gpu_readings, right_chunks[1]);
    gpu_processes_panel(f, app, &app.gpu_readings, right_chunks[2]);
}
