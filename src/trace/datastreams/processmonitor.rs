use std::thread;
use std::time::Duration;
use sysinfo::{Pid, PidExt, Process, System, ProcessExt, SystemExt, ProcessRefreshKind};
use crate::trace::datastreams::datastream::SysDataStream;
use num_traits::cast::ToPrimitive;

pub struct ProcessMonitor {
    pub processes: Vec<(u32, String, f32, u64)>, //PID, Command, CPU. mem (kb)
}

impl SysDataStream for ProcessMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    fn poll(&mut self, system_info: &mut System, pid: &Pid) {
        system_info.refresh_process(*pid);
        let process = system_info.process(*pid).unwrap();
        self.processes.clear();
        thread::sleep(Duration::from_millis(10));
        self.processes.push(ProcessMonitor::parse_process_info(*pid, process));
    }
}

impl ProcessMonitor {
    fn parse_process_info(pid: Pid, process: &Process) -> (u32, String, f32, u64) {
        (pid.as_u32(), String::from(process.name()), process.cpu_usage(), process.memory())
    }
}
