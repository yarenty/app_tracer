use sysinfo::SystemExt;
use sysinfo::{Pid, System as SysInfoSystem};

use crate::trace::datastreams::{
    CPUMonitor, MemoryMonitor, ProcessMonitor, Readings, SysDataStream,
};

use crate::error::Result;

pub struct AppDataStreams {
    pub pid: Pid,
    pub cpu_info: CPUMonitor,
    pub mem_info: MemoryMonitor,
    pub process_info: ProcessMonitor,
    pub sys_info_src: SysInfoSystem,
    pub readings: Readings,
}

impl AppDataStreams {
    pub fn new(history_len: usize, interpolation_len: u16, pid: Pid) -> Result<Self> {
        let mut sys = SysInfoSystem::new();
        let readings = Readings::new(&mut sys, pid);
        Ok(Self {
            pid,
            cpu_info: SysDataStream::new(history_len, interpolation_len),
            mem_info: SysDataStream::new(history_len, interpolation_len),
            process_info: SysDataStream::new(history_len, interpolation_len),
            sys_info_src: sys,
            readings,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.sys_info_src.refresh_process(self.pid);
        self.readings
            .refresh(self.sys_info_src.process(self.pid).unwrap());

        self.cpu_info.poll(&self.readings);
        self.mem_info.poll(&self.readings);
        self.process_info.poll(&self.readings);
        Ok(())
    }
}
