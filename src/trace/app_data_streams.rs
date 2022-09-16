use sysinfo::SystemExt;
use sysinfo::{Pid, System as SysInfoSystem};

use crate::trace::datastreams::{CPUMonitor, MemoryMonitor, ProcessMonitor, SysDataStream};

use crate::error::Result;

pub struct AppDataStreams {
    pub pid: Pid,
    pub cpu_info: CPUMonitor,
    pub mem_info: MemoryMonitor,
    pub process_info: ProcessMonitor,
    pub sys_info_src: SysInfoSystem,
}

impl<'a> AppDataStreams {
    pub fn new(history_len: usize, interpolation_len: u16, pid: Pid) -> Result<Self> {
        Ok(Self {
            pid,
            cpu_info: SysDataStream::new(history_len, interpolation_len),
            mem_info: SysDataStream::new(history_len, interpolation_len),
            process_info: SysDataStream::new(history_len, interpolation_len),
            sys_info_src: SysInfoSystem::new(),
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.sys_info_src.refresh_all();
        self.cpu_info.poll(&mut self.sys_info_src, &self.pid);
        self.mem_info.poll(&mut self.sys_info_src, &self.pid);
        self.process_info.poll(&mut self.sys_info_src, &self.pid);
        Ok(())
    }
}
