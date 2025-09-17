use crate::trace::app::App;
use crate::trace::datastreams::GpuReadings;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Paragraph};
use ratatui::Frame;

pub fn gpu_temperature_panel(f: &mut Frame, _app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Temperature")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];

    if let Some(temp) = gpu.temperature.gpu {
        // Create temperature gauge
        let temp_gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!("GPU {} Temperature", gpu.index))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .gauge_style(
                if temp > 85 {
                    Style::default().fg(Color::Red)
                } else if temp > 75 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                },
            )
            .ratio(temp as f64 / 100.0) // Assuming max temp around 100°C
            .label(format!("{}°C", temp));

        f.render_widget(temp_gauge, area);
    } else {
        let no_temp_text = Paragraph::new("Temperature not available")
            .block(
                Block::default()
                    .title(format!("GPU {} Temperature", gpu.index))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(no_temp_text, area);
    }
}

pub fn gpu_temperature_history_panel(f: &mut Frame, app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Temperature History")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];
    
    if let Some(temp) = gpu.temperature.gpu {
        // Create temperature data point
        let temp_data = vec![(0.0, temp as f64)];
        
        let datasets = vec![Dataset::default()
            .name(format!("GPU {} Temperature", gpu.index))
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightMagenta))
            .data(&temp_data)];

        let style = Style::default().add_modifier(Modifier::ITALIC);
        let labels = vec![
            Span::styled("0", style),
            Span::styled("25", style),
            Span::styled("50", style),
            Span::styled("75", style),
            Span::styled("100", style),
        ];

        let temperature_chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(Span::styled(
                        format!("GPU {} Temperature History", gpu.index),
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
                    .title("Temperature (°C)")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(labels),
            );

        f.render_widget(temperature_chart, area);
    } else {
        let no_temp_text = Paragraph::new("Temperature data not available")
            .block(
                Block::default()
                    .title(format!("GPU {} Temperature History", gpu.index))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(no_temp_text, area);
    }
}
