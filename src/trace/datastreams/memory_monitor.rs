use sysinfo::{Pid, ProcessExt, System, SystemExt};

use crate::trace::datastreams::{data_stream::SysDataStream, utils};

pub struct MemoryMonitor {
    pub memory_usage: u64,
    pub memory_usage_history: Vec<f64>, //Name, Usage
    pub total_memory: u64,
    max_history_len: usize,
    interpolation_len: u16,
}

impl SysDataStream for MemoryMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            memory_usage: 0,
            total_memory: 10,
            memory_usage_history: vec![0.0; max_hist_len],
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
        }
    }

    fn poll(&mut self, system_info: &mut System, pid: &Pid) {
        system_info.refresh_process(*pid);

        let pr = system_info.process(*pid).unwrap();

        self.memory_usage = pr.memory();
        self.total_memory = system_info.total_memory();

        while self.memory_usage_history.len() >= self.max_history_len {
            self.memory_usage_history.remove(0);
        }
        let last_mem = match self.memory_usage_history.last() {
            Some(l) => *l,
            None => 0.0,
        };
        self.memory_usage_history.extend_from_slice(
            utils::interpolate(
                last_mem,
                self.memory_usage as f64 / self.total_memory as f64,
                self.interpolation_len,
            )
            .as_slice(),
        );
    }
}
