use crate::trace::app::App;
use crate::trace::datastreams::GpuReadings;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Paragraph};
use ratatui::Frame;

pub fn gpu_memory_panel(f: &mut Frame, _app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Memory Usage")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];
    let memory_usage_percent = if gpu.memory.total > 0 {
        (gpu.memory.used as f64 / gpu.memory.total as f64) * 100.0
    } else {
        0.0
    };

    // Create memory gauge
    let memory_gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("GPU {} Memory Usage", gpu.index))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        )
        .gauge_style(
            if memory_usage_percent > 90.0 {
                Style::default().fg(Color::Red)
            } else if memory_usage_percent > 75.0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            },
        )
        .ratio(memory_usage_percent / 100.0)
        .label(format!(
            "{:.1}% ({:.1} GB / {:.1} GB)",
            memory_usage_percent,
            gpu.memory.used as f64 / 1024.0,
            gpu.memory.total as f64 / 1024.0
        ));

    f.render_widget(memory_gauge, area);
}

pub fn gpu_memory_history_panel(f: &mut Frame, app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Memory History")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];
    
    // Create a simple memory usage history (we'll need to add this to the app structure)
    // For now, we'll create a placeholder chart
    let memory_usage_percent = if gpu.memory.total > 0 {
        (gpu.memory.used as f64 / gpu.memory.total as f64) * 100.0
    } else {
        0.0
    };

    // Create a simple data point for the current memory usage
    let ds: Vec<(f64, f64)> = vec![(0.0, memory_usage_percent)];
    
    let datasets = vec![Dataset::default()
        .name(format!("GPU {} Memory", gpu.index))
        .marker(Marker::Braille)
        .style(Style::default().fg(Color::LightBlue))
        .data(&ds)];

    let style = Style::default().add_modifier(Modifier::ITALIC);
    let labels = vec![
        Span::styled("0", style),
        Span::styled("25", style),
        Span::styled("50", style),
        Span::styled("75", style),
        Span::styled("100", style),
    ];

    let memory_chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    format!("GPU {} Memory History", gpu.index),
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
                .title("Usage (%)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(labels),
        );

    f.render_widget(memory_chart, area);
}
