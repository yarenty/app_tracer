use crate::trace::datastreams::{data_stream::SysDataStream, utils};
use std::collections::HashMap;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

pub struct CPUMonitor {
    pub cpu_usage: f32,
    pub cpu_core_info: Vec<(String, f32)>, //Name, Usage
    pub cpu_usage_history: HashMap<String, Vec<f32>>, //Name, Usage
    max_history_len: usize,
    interpolation_len: u16,
}

impl SysDataStream for CPUMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            cpu_usage: 0.0,
            cpu_core_info: Vec::new(),
            cpu_usage_history: HashMap::new(),
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
        }
    }

    fn poll(&mut self, system_info: &mut System, pid: &Pid) {
        self.cpu_core_info.clear();
        system_info.refresh_process(*pid);
        let pr = system_info.process(*pid).unwrap();

        let info = ("1".to_string(), pr.cpu_usage() / 100.0); //CPUMonitor::parse_cpu_info(cpu);
        self.cpu_core_info.push(info);
        if let Some(entry) = self.cpu_core_info.last() {
            #[allow(clippy::or_fun_call)]
            let history = self
                .cpu_usage_history
                .entry(entry.0.clone())
                .or_insert(vec![0.0; self.max_history_len]);
            while history.len() >= self.max_history_len {
                history.remove(0);
            }
            let last = match history.last() {
                Some(l) => *l,
                None => 0.0,
            };
            history.extend_from_slice(
                utils::interpolate::<f32>(last, entry.1, self.interpolation_len).as_slice(),
            );
        }

        self.cpu_usage = pr.cpu_usage();
    }
}
