use std::collections::HashMap;
use sysinfo::{Pid, PidExt};
use termion::event::Key;

use crate::trace::cmd::Cmd;
use crate::trace::ui::tabs::Tabs;
use crate::trace::appdatastreams::AppDataStreams;
use crate::error::{Result, TraceError};

pub struct App<'a> {
    pub pid: Pid,
    pub selected_proc: usize,
    pub tabs: Tabs<'a>,
    pub window: [f64; 2],
    pub cpu_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    pub mem_panel_memory: Vec<(f64, f64)>,
    pub mem_usage_str: String,
    pub datastreams: AppDataStreams
}

impl <'a> App<'a> {
    pub fn new(history_len: usize, interpolation_len: u16, pid:Pid) -> Result<Self> {
        Ok(Self {
            pid: pid,
            // selected_proc: pid.as_u32() as usize,
            selected_proc: 0,
            tabs: Tabs {
                titles: {
                        vec!["Live tracing memory and CPU usage"]
                },
                selection: 0,
            },
            window: [0.0, history_len as f64],
            cpu_panel_memory: HashMap::new(),
            mem_panel_memory: Vec::new(),
            mem_usage_str: String::new(),
            datastreams: AppDataStreams::new(history_len, interpolation_len, pid)?
        })
    }



    pub fn input_handler(&mut self, input: Key) -> Option<Cmd>{
        match input {
            Key::Char('q') => {
                return Some(Cmd::Quit);
            }
            Key::Up => {

                if  self.tabs.selection == 0 && self.selected_proc > 0 {
                    self.selected_proc -= 1
                }
            }
            Key::Down => {

                if  self.tabs.selection == 0 && self.selected_proc < self.datastreams.process_info.processes.len() - 1 {
                    self.selected_proc += 1;
                }
            },
            Key::Left => {
                self.tabs.previous();
            }
            Key::Right => {
                self.tabs.next();
            },
            _ => {}
        }
        None
    }

    pub fn update(&mut self) -> Result<()> {
        self.datastreams.update()?;
        //CPU History Parsing
        {
            for (name, usage) in &self.datastreams.cpu_info.cpu_usage_history {
                let pairwise_data = usage.iter()
                                        .enumerate()
                                        .map(|x| (x.0 as f64, *x.1 as f64))
                                        .collect::<Vec<(f64, f64)>>();
                let mut core_name = name.clone();
                let core_num;
                if cfg!(target_os = "macos") {
                    #[allow(clippy::match_wild_err_arm)]
                    match core_name.parse::<u32>() {
                        Ok(num) => {core_num = num - 1}, //MacOS
                        Err(_) => {panic!("Unable to parse CPU ID")}
                    }
                } else if core_name.contains("cpu") {
                    let (_,s) = core_name.split_at_mut(3);
                    #[allow(clippy::match_wild_err_arm)]
                    match s.parse::<u32>() {
                        Ok(num) => {core_num = num},
                        Err(_) => {panic!("Unable to parse CPU ID")},
                    }
                } else {
                    panic!("Cannot get CPU ID");
                }

                // let  core_num = core_num -1;
                let core_label = core_num.to_string();
                
                //fixed number of cores
                core_name = format!("Total CPU: {:.3}%", 
                                                      (self.datastreams.cpu_info.cpu_core_info[(core_num) as usize].1 * 100.0).to_string());
                self.cpu_panel_memory.insert(core_num, (core_name, pairwise_data));
            }
            
        }
        //Memory History Parsing
        {
            self.mem_panel_memory =  self.datastreams.mem_info.memory_usage_history.iter()
                                                                        .enumerate()
                                                                        .map(|(i, u)| (i as f64, *u))
                                                                        .collect::<Vec<(f64, f64)>>();
            self.mem_usage_str = format!("Memory ({:.2}%)", 100.0 * self.datastreams.mem_info.memory_usage as f64 / self.datastreams.mem_info.total_memory as f64);
        }
        

        Ok(())
    }



    fn si_prefix(num: u64) -> (u64, String) {
        let n = num as f64;
        if n == 0.0 {
            return (1_u64, String::from(""));
        }
        match n.log(10.0) as u64 {
            0 | 1 | 2 => (10_u64.pow(0), String::from("")),
            3 | 4 | 5 => (10_u64.pow(3), String::from("K")),
            6 | 7 | 8 => (10_u64.pow(6), String::from("M")),
            9 | 10 | 11 => (10_u64.pow(9), String::from("G")),
            12 | 13 | 14 => (10_u64.pow(12), String::from("T")),
            15 | 16 | 17 => (10_u64.pow(15), String::from("P")),
            _ => (10_u64.pow(18), String::from("E")),
        }
    }
}
