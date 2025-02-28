use std::sync::LazyLock;
use sysinfo::Pid;
use termion::event::Key;

use crate::error::Result;
use crate::trace::app_data_streams::AppDataStreams;
use crate::trace::cmd::Cmd;
use crate::trace::ui::tabs::Tabs;
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Spans};

static INFO: LazyLock<String> = LazyLock::new(|| {
    format!(
        "Live tracing memory and CPU usage, version {}.",
        env!("CARGO_PKG_VERSION")
    )
});

pub struct App<'a> {
    #[allow(dead_code)]
    pub pid: Pid,
    pub selected_proc: usize,
    pub tabs: Tabs<'a>,
    pub window: [f64; 2],
    pub cpu_panel_memory: Vec<(f64, f64)>,
    pub mem_panel_memory: Vec<(f64, f64)>,
    pub cpu_usage_str: String,
    pub mem_usage_str: String,
    pub datastreams: AppDataStreams,
    pub autoscale: bool,
    pub refresh: u64,
}

impl<'a> App<'a> {
    pub fn new(
        history_len: usize,
        interpolation_len: u16,
        pid: Pid,
        autoscale: bool,
        refresh: u64,
    ) -> Result<Self> {
        Ok(Self {
            pid,
            selected_proc: 0,
            tabs: Tabs {
                titles: {
                    vec![Spans::from(vec![
                        Span::styled(&*INFO, Style::default().fg(Color::LightYellow)),
                        Span::styled("   q-Quit", Style::default().fg(Color::Yellow)),
                    ])]
                },
                selection: 0,
            },
            window: [0.0, history_len as f64],
            cpu_panel_memory: Vec::new(),
            mem_panel_memory: Vec::new(),
            cpu_usage_str: String::new(),
            mem_usage_str: String::new(),
            datastreams: AppDataStreams::new(history_len, interpolation_len, pid)?,
            autoscale,
            refresh,
        })
    }

    pub fn input_handler(&mut self, input: Key) -> Option<Cmd> {
        match input {
            Key::Char('q') => {
                return Some(Cmd::Quit);
            }
            Key::Up => {
                if self.tabs.selection == 0 && self.selected_proc > 0 {
                    self.selected_proc -= 1
                }
            }
            Key::Down => {
                if self.tabs.selection == 0
                    && self.selected_proc < self.datastreams.process_info.processes.len() - 1
                {
                    self.selected_proc += 1;
                }
            }
            Key::Left => {
                self.tabs.previous();
            }
            Key::Right => {
                self.tabs.next();
            }
            _ => {}
        }
        None
    }

    pub fn update(&mut self) -> Result<()> {
        self.datastreams.update()?;
        //CPU History Parsing
        {
            self.cpu_panel_memory = self
                .datastreams
                .cpu_info
                .cpu_usage_history
                .iter()
                .enumerate()
                .map(|(i, u)| (i as f64, *u as f64))
                .collect::<Vec<(f64, f64)>>();

            self.cpu_usage_str =
                format!("Total CPU: ({:.2}%)", self.datastreams.cpu_info.cpu_usage);
        }
        //Memory History Parsing
        {
            self.mem_panel_memory = self
                .datastreams
                .mem_info
                .memory_usage_history
                .iter()
                .enumerate()
                .map(|(i, u)| (i as f64, *u))
                .collect::<Vec<(f64, f64)>>();
            self.mem_usage_str = format!(
                "Total memory ({:.2}%)",
                100.0 * self.datastreams.mem_info.memory_usage as f64
                    / self.datastreams.mem_info.total_memory as f64
            );
        }

        Ok(())
    }
}
