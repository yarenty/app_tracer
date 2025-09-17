use crate::trace::app::App;
use crate::trace::datastreams::GpuReadings;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

pub fn gpu_processes_panel(f: &mut Frame, app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Processes")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // Split area into GPU info and processes
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // GPU info header
            Constraint::Min(0),    // Processes list
        ])
        .split(area);

    // Show GPU information header
    let gpu = &gpu_readings.gpus[0];
    let gpu_info = Paragraph::new(format!(
        "GPU {}: {} | Driver: {} | Compute: {} | P-State: {}",
        gpu.index, gpu.name, gpu.driver_version, gpu.compute_cap, gpu.pstate
    ))
    .block(
        Block::default()
            .title("GPU Information")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray)),
    )
    .style(Style::default().fg(Color::Cyan));

    f.render_widget(gpu_info, chunks[0]);

    // Show GPU processes
    if gpu_readings.processes.is_empty() {
        let no_processes_text = Paragraph::new("No GPU processes currently running")
            .block(
                Block::default()
                    .title("GPU Processes")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(no_processes_text, chunks[1]);
    } else {
        let process_items: Vec<ListItem> = gpu_readings
            .processes
            .iter()
            .enumerate()
            .map(|(i, process)| {
                let memory_mb = process.used_memory as f64 / 1024.0;
                let is_selected = i == app.selected_proc;
                
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("PID: {:>8} | ", process.pid),
                        style.fg(Color::Cyan),
                    ),
                    Span::styled(
                        format!("{:>20} | ", process.process_name),
                        style.fg(Color::Green),
                    ),
                    Span::styled(
                        format!("Memory: {:>8.1} MB", memory_mb),
                        style.fg(Color::Yellow),
                    ),
                ]))
            })
            .collect();

        let processes_list = List::new(process_items)
            .block(
                Block::default()
                    .title(format!("GPU Processes ({} running)", gpu_readings.processes.len()))
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(processes_list, chunks[1]);
    }
}

pub fn gpu_summary_panel(f: &mut Frame, _app: &App, gpu_readings: &GpuReadings, area: Rect) {
    if gpu_readings.gpus.is_empty() {
        let no_gpu_text = Paragraph::new("No GPUs detected")
            .block(
                Block::default()
                    .title("GPU Summary")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray)),
            )
            .style(Style::default().fg(Color::Red));
        f.render_widget(no_gpu_text, area);
        return;
    }

    // Create a summary of all GPUs
    let gpu = &gpu_readings.gpus[0];
    let memory_usage_percent = if gpu.memory.total > 0 {
        (gpu.memory.used as f64 / gpu.memory.total as f64) * 100.0
    } else {
        0.0
    };

    let summary_text = format!(
        "GPU {}: {}\n\
         Memory: {:.1} GB / {:.1} GB ({:.1}%)\n\
         Utilization: GPU {}%, Memory {}%\n\
         Temperature: {}\n\
         Power: {}\n\
         Processes: {}",
        gpu.index,
        gpu.name,
        gpu.memory.used as f64 / 1024.0,
        gpu.memory.total as f64 / 1024.0,
        memory_usage_percent,
        gpu.utilization.gpu,
        gpu.utilization.memory,
        gpu.temperature.gpu
            .map(|t| format!("{}°C", t))
            .unwrap_or_else(|| "N/A".to_string()),
        gpu.power.draw
            .map(|p| format!("{:.1}W", p))
            .unwrap_or_else(|| "N/A".to_string()),
        gpu_readings.processes.len()
    );

    let summary_paragraph = Paragraph::new(summary_text)
        .block(
            Block::default()
                .title("GPU Summary")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(summary_paragraph, area);
}
