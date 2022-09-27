use crate::trace::app::App;

use itertools::Itertools;
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

    let cpus = if app.autoscale {
        let cpu_data = &app.cpu_panel_memory.get(&0).unwrap().1;
        let auto: _ = cpu_data.iter().map(|(_x, y)| y).collect_vec();
        // println!("auto:{:?}", auto);
        let auto = auto.iter().max_by(|a, b| a.total_cmp(b)).or(Some(&&1.0));
        // println!("MAX:{:?}", auto);
        let m = auto.unwrap().max(1.0);
        let m = m.min(app.datastreams.readings.get_cpus_count() as f64);

        (m + 0.2).round()
    } else {
        app.datastreams.readings.get_cpus_count() as f64
    };
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
