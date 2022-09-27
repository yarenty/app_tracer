use std::collections::HashMap;
use sysinfo::Pid;
use termion::event::Key;

use crate::error::Result;
use crate::trace::app_data_streams::AppDataStreams;
use crate::trace::cmd::Cmd;
use crate::trace::ui::tabs::Tabs;
use const_format::formatcp;

const INFO: &str = formatcp!(
    "Live tracing memory and CPU usage, version {}.",
    env!("CARGO_PKG_VERSION")
);

pub struct App<'a> {
    pub pid: Pid,
    pub selected_proc: usize,
    pub tabs: Tabs<'a>,
    pub window: [f64; 2],
    pub cpu_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    pub mem_panel_memory: Vec<(f64, f64)>,
    pub mem_usage_str: String,
    pub datastreams: AppDataStreams,
    pub autoscale: bool,
}

impl<'a> App<'a> {
    pub fn new(
        history_len: usize,
        interpolation_len: u16,
        pid: Pid,
        autoscale: bool,
    ) -> Result<Self> {
        Ok(Self {
            pid,
            selected_proc: 0,
            tabs: Tabs {
                titles: { vec![INFO, "   q-Quit"] },
                selection: 0,
            },
            window: [0.0, history_len as f64],
            cpu_panel_memory: HashMap::new(),
            mem_panel_memory: Vec::new(),
            mem_usage_str: String::new(),
            datastreams: AppDataStreams::new(history_len, interpolation_len, pid)?,
            autoscale,
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
            for (name, usage) in &self.datastreams.cpu_info.cpu_usage_history {
                let pairwise_data = usage
                    .iter()
                    .enumerate()
                    .map(|x| (x.0 as f64, *x.1 as f64))
                    .collect::<Vec<(f64, f64)>>();
                let mut core_name = name.clone();
                let core_num;
                if cfg!(target_os = "macos") {
                    #[allow(clippy::match_wild_err_arm)]
                    match core_name.parse::<u32>() {
                        Ok(num) => core_num = num - 1, //MacOS
                        Err(_) => {
                            panic!("Unable to parse CPU ID")
                        }
                    }
                } else if core_name.contains("cpu") {
                    let (_, s) = core_name.split_at_mut(3);
                    #[allow(clippy::match_wild_err_arm)]
                    match s.parse::<u32>() {
                        Ok(num) => core_num = num,
                        Err(_) => {
                            panic!("Unable to parse CPU ID")
                        }
                    }
                } else {
                    #[allow(clippy::match_wild_err_arm)]
                    match core_name.parse::<u32>() {
                        Ok(num) => core_num = num - 1,
                        Err(_) => {
                            panic!("Unable to parse CPU ID")
                        }
                    }
                }

                //fixed number of cores
                core_name = format!(
                    "Total CPU: ({:.2}%)",
                    (self.datastreams.cpu_info.cpu_core_info[(core_num) as usize].1 * 100.0)
                        .to_string()
                );
                self.cpu_panel_memory
                    .insert(core_num, (core_name, pairwise_data));
            }
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
