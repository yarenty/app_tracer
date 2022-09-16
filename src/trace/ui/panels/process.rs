use crate::trace::app::App;
use crate::trace::ui::panels::utils;

use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Row, Table};
use tui::Frame;

pub fn process_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let mut process_by_cpu = app.datastreams.process_info.processes.clone();
    process_by_cpu.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let (selected_proc, process_to_display) =
        utils::scrolling(area, app.selected_proc, &process_by_cpu[..]);

    let selected_style = Style::default().fg(Color::White).bg(Color::Green);
    let default_style = Style::default().fg(Color::LightBlue);
    let proc_table = Table::new(
        ["PID", "Command", "%CPU", "Mem (KB)"].iter(),
        process_to_display.iter().enumerate().map(|(i, s)| {
            let style = if i == selected_proc {
                &selected_style
            } else {
                &default_style
            };
            Row::StyledData(
                vec![
                    s.0.to_string(),
                    s.1.to_string(),
                    format!("{:.2}", s.2),
                    s.3.to_string(),
                ]
                .into_iter(),
                *style,
            )
        }),
    )
    .block(Block::default().title("Process").borders(Borders::ALL))
    .header_style(Style::default().fg(Color::Blue))
    .header_gap(0)
    .widths(
        [
            Constraint::Length(10),
            Constraint::Length(25),
            Constraint::Length(10),
            Constraint::Length(10),
        ]
        .as_ref(),
    );

    f.render_widget(proc_table, area);
}