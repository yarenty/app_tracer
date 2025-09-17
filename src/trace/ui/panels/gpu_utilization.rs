use crate::trace::app::App;
use crate::trace::datastreams::GpuReadings;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Paragraph};
use ratatui::Frame;

pub fn gpu_utilization_panel(f: &mut Frame, _app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Utilization")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];

    // Create GPU utilization gauge
    let gpu_util_gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("GPU {} Utilization", gpu.index))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        )
        .gauge_style(
            if gpu.utilization.gpu > 90 {
                Style::default().fg(Color::Red)
            } else if gpu.utilization.gpu > 75 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            },
        )
        .ratio(gpu.utilization.gpu as f64 / 100.0)
        .label(format!("GPU: {}%", gpu.utilization.gpu));

    // Create memory utilization gauge
    let mem_util_gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("GPU {} Memory Utilization", gpu.index))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        )
        .gauge_style(
            if gpu.utilization.memory > 90 {
                Style::default().fg(Color::Red)
            } else if gpu.utilization.memory > 75 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Blue)
            },
        )
        .ratio(gpu.utilization.memory as f64 / 100.0)
        .label(format!("Memory: {}%", gpu.utilization.memory));

    // Split the area into two gauges
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage(50),
            ratatui::layout::Constraint::Percentage(50),
        ])
        .split(area);

    f.render_widget(gpu_util_gauge, chunks[0]);
    f.render_widget(mem_util_gauge, chunks[1]);
}

pub fn gpu_utilization_history_panel(f: &mut Frame, app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Utilization History")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];
    
    // Create utilization data points
    let gpu_util_data = vec![(0.0, gpu.utilization.gpu as f64)];
    let mem_util_data = vec![(0.0, gpu.utilization.memory as f64)];
    
    let datasets = vec![
        Dataset::default()
            .name("GPU Utilization")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightRed))
            .data(&gpu_util_data),
        Dataset::default()
            .name("Memory Utilization")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightBlue))
            .data(&mem_util_data),
    ];

    let style = Style::default().add_modifier(Modifier::ITALIC);
    let labels = vec![
        Span::styled("0", style),
        Span::styled("25", style),
        Span::styled("50", style),
        Span::styled("75", style),
        Span::styled("100", style),
    ];

    let utilization_chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    format!("GPU {} Utilization History", gpu.index),
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
                .bounds([0.0, 100.0]),
        )
        .y_axis(
            Axis::default()
                .title("Utilization (%)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(labels),
        );

    f.render_widget(utilization_chart, area);
}
