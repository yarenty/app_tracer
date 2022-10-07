use crate::trace::app::App;

use itertools::Itertools;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols::Marker;
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset};
use tui::Frame;

pub fn mem_history_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let total = (app.datastreams.readings.get_total_memory() / 1024 / 1024) as f64;

    let mem = if app.autoscale {
        let auto: _ = &app.mem_panel_memory.iter().map(|(_x, y)| y).collect_vec();
        let auto = auto.iter().max_by(|a, b| a.total_cmp(b)).or(Some(&&0.0));
        let m = auto.unwrap();

        (*m * total + 0.9).round() // to not to be 100% almost all the time
    } else {
        total
    };

    let ds: Vec<(f64, f64)> = app
        .mem_panel_memory
        .iter()
        .map(|(x, y)| (*x, *y * total / mem))
        .collect_vec();
    let datasets = vec![Dataset::default()
        .name(&app.mem_usage_str)
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::LightGreen))
        .data(&ds)];

    let c100 = format!("{}", mem);
    let c75 = format!("{}", mem * 0.75);
    let c50 = format!("{}", mem * 0.5);
    let c25 = format!("{}", mem * 0.25);

    let style = Style::default().add_modifier(Modifier::ITALIC);
    // let label = vec![Span::styled("", style)];
    let labels = vec![
        Span::styled("0", style),
        Span::styled(&c25, style),
        Span::styled(&c50, style),
        Span::styled(&c75, style),
        Span::styled(&c100, style),
    ];
    let title = Span::styled(
        "Memory Usage",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let mem_history_panel = Chart::new(datasets)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        )
        .x_axis(
            Axis::default()
                .title("")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.window), // .labels(label),
        )
        .y_axis(
            Axis::default()
                .title("Usage (GB)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 1.0])
                .labels(labels),
        );

    f.render_widget(mem_history_panel, area);
}
