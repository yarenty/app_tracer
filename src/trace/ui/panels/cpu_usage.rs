use crate::trace::app::App;

use itertools::Itertools;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols::Marker;
use tui::text::Span;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset};
use tui::Frame;

pub fn cpu_usage_history_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let ds: Vec<(f64, f64)> = app
        .cpu_panel_memory
        .iter()
        .map(|(x, y)| (*x, *y))
        .collect_vec();
    let datasets = vec![Dataset::default()
        .name(&app.cpu_usage_str)
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::LightRed))
        .data(&ds)];

    let cpus = if app.autoscale {
        let cpu_data = &app.cpu_panel_memory;
        let auto: _ = cpu_data.iter().map(|(_x, y)| y).collect_vec();
        // println!("auto:{:?}", auto);
        let auto = auto.iter().max_by(|a, b| a.total_cmp(b)).or(Some(&&1.0));
        // println!("MAX:{:?}", auto);
        let m = auto.unwrap().max(1.0);
        let m = m.min(app.datastreams.readings.get_cpus_count() as f64);

        m.ceil()
    } else {
        app.datastreams.readings.get_cpus_count() as f64
    };
    let c100 = format!("{}", cpus * 100.0);
    let c75 = format!("{}", cpus * 75.0);
    let c50 = format!("{}", cpus * 50.0);
    let c25 = format!("{}", cpus * 25.0);

    let style = Style::default().add_modifier(Modifier::ITALIC);
    let labels = vec![
        Span::styled("0", style),
        Span::styled(&c25, style),
        Span::styled(&c50, style),
        Span::styled(&c75, style),
        Span::styled(&c100, style),
    ];
    let cpu_usage = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "CPU Usage",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().fg(Color::Gray))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title(format!("time[{}ms]", app.refresh))
                .style(Style::default().fg(Color::Gray))
                // .labels(label)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title("Usage (%)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, cpus])
                .labels(labels),
        );

    f.render_widget(cpu_usage, area);
}
