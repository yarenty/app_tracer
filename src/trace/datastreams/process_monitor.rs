use crate::trace::datastreams::data_stream::{Readings, SysDataStream};

pub struct ProcessMonitor {
    pub processes: Vec<(u32, String, f32, u64)>, //PID, Command, CPU. mem (kb)
}

impl SysDataStream for ProcessMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    fn poll(&mut self, system_info: &Readings) {
        self.processes.clear();
        self.processes.push((
            system_info.get_pid(),
            system_info.get_process(),
            system_info.get_cpu(),
            system_info.get_mem(),
        ));
    }
}
