use crate::trace::app::App;
use crate::trace::ui::panels::utils;

use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Row, Table};
use tui::Frame;

pub fn process_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let mut process_by_cpu = app.datastreams.process_info.processes.clone();
    process_by_cpu.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let (_, process_to_display) = utils::scrolling(area, app.selected_proc, &process_by_cpu[..]);

    let s = process_to_display.get(0).unwrap();
    let proc_table = Table::new(vec![
        // Row can be created from simple strings.
        Row::new(vec![
            s.0.to_string(),
            s.1.to_string(),
            format!("{:.2}", s.2),
            s.3.to_string(),
        ])
        .style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])
    // You can set the style of the entire Table.
    .style(Style::default().fg(Color::Gray))
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(vec!["PID", "Command", "%CPU", "Mem (KB)"])
            .style(Style::default().fg(Color::LightBlue))
            // If you want some space between the header and the rest of the rows, you can always
            // specify some margin at the bottom.
            .bottom_margin(0),
    )
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Table"))
    // Columns widths are constrained in the same way as Layout...
    .widths(&[
        Constraint::Length(10),
        Constraint::Length(25),
        Constraint::Length(10),
        Constraint::Length(10),
    ])
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // If you wish to highlight a row in any specific way when it is selected...
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>")
    .block(
        Block::default()
            .title(Span::styled(
                "Application",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL),
    );

    f.render_widget(proc_table, area);
}
