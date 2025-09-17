use crate::trace::app::App;
use crate::trace::datastreams::GpuReadings;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Paragraph};
use ratatui::Frame;

pub fn gpu_power_panel(f: &mut Frame, app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Power")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];

    if let Some(power_draw) = gpu.power.draw {
        // Create power gauge
        let power_gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!("GPU {} Power Draw", gpu.index))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .gauge_style(
                if power_draw > 200.0 {
                    Style::default().fg(Color::Red)
                } else if power_draw > 150.0 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                },
            )
            .ratio(power_draw as f64 / 300.0) // Assuming max power around 300W
            .label(format!("{:.1}W", power_draw));

        f.render_widget(power_gauge, area);
    } else {
        let no_power_text = Paragraph::new("Power data not available")
            .block(
                Block::default()
                    .title(format!("GPU {} Power Draw", gpu.index))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(no_power_text, area);
    }
}

pub fn gpu_power_history_panel(f: &mut Frame, app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Power History")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // For now, show the first GPU (we'll add multi-GPU support later)
    let gpu = &gpu_readings.gpus[0];
    
    if let Some(power_draw) = gpu.power.draw {
        // Create power data point
        let power_data = vec![(0.0, power_draw as f64)];
        
        let datasets = vec![Dataset::default()
            .name(format!("GPU {} Power", gpu.index))
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightYellow))
            .data(&power_data)];

        let style = Style::default().add_modifier(Modifier::ITALIC);
        let labels = vec![
            Span::styled("0", style),
            Span::styled("75", style),
            Span::styled("150", style),
            Span::styled("225", style),
            Span::styled("300", style),
        ];

        let power_chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(Span::styled(
                        format!("GPU {} Power History", gpu.index),
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
                    .title("Power (W)")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 300.0])
                    .labels(labels),
            );

        f.render_widget(power_chart, area);
    } else {
        let no_power_text = Paragraph::new("Power data not available")
            .block(
                Block::default()
                    .title(format!("GPU {} Power History", gpu.index))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(no_power_text, area);
    }
}
