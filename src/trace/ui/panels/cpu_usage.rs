use crate::trace::app::App;
use sysinfo::{System, SystemExt};

use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols::Marker;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset};
use tui::Frame;

pub fn cpu_usage_history_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let mut data = app
        .cpu_panel_memory
        .iter()
        .map(|x| (*x.0, (x.1).0.clone(), (x.1).1.clone()))
        .collect::<Vec<(u32, String, Vec<(f64, f64)>)>>();
    data.sort_by_key(|k| k.0);

    let datasets = &data
        .iter()
        .map(|x| {
            Dataset::default()
                .name(&x.1)
                .marker(Marker::Braille)
                .style(Style::default().fg(color_map(x.0)))
                .data(&x.2)
        })
        .collect::<Vec<Dataset>>();

    let mut s = System::new();
    s.refresh_system();
    // let cpus = (s.cpus().len() / 4) as f64;
    let cpus = (s.cpus().len()) as f64;
    let c100 = format!("{}", cpus * 100.0);
    let c75 = format!("{}", cpus * 75.0);
    let c50 = format!("{}", cpus * 50.0);
    let c25 = format!("{}", cpus * 25.0);

    let labels = ["0", &c25, &c50, &c75, &c100];

    let cpu_usage = Chart::default()
        .block(
            Block::default()
                .title("CPU Usage")
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
                .title("Usage (%)")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::ITALIC))
                .bounds([0.0, cpus])
                .labels(&labels),
        )
        .datasets(datasets);

    f.render_widget(cpu_usage, area);
}

fn color_map(key: u32) -> Color {
    match key % 10 {
        0 => Color::LightRed,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Magenta,
        4 => Color::Cyan,
        5 => Color::Red,
        6 => Color::LightGreen,
        7 => Color::LightYellow,
        8 => Color::LightMagenta,
        9 => Color::LightCyan,
        _ => Color::White,
    }
}
