use crate::trace::app::App;

use itertools::Itertools;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols::Marker;
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
    let datasets = [Dataset::default()
        .name(&app.mem_usage_str)
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::LightGreen))
        .data(&ds)];

    let c100 = format!("{}", mem);
    let c75 = format!("{}", mem * 0.75);
    let c50 = format!("{}", mem * 0.5);
    let c25 = format!("{}", mem * 0.25);

    let labels = ["0", &c25, &c50, &c75, &c100];

    let mem_history_panel = Chart::default()
        .block(
            Block::default()
                .title("Memory Usage")
                .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::ITALIC))
                .bounds(app.window)
                .labels(&[""]),
        )
        .y_axis(
            Axis::default()
                .title("Usage (GB)")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::ITALIC))
                .bounds([0.0, 1.0])
                .labels(&labels),
        )
        .datasets(&datasets);

    f.render_widget(mem_history_panel, area);
}
