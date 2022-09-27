use sysinfo::{Pid, System as SysInfoSystem};
use sysinfo::{ProcessExt, SystemExt};

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

        let p = self.sys_info_src.process(self.pid).unwrap();
        let mut cpu = p.cpu_usage();
        let mut mem = p.memory();

        let ps = self.sys_info_src.processes();

        let mut subs: Vec<Pid> = vec![];
        for p in ps
            .iter()
            .map(|(_k, v)| v)
            .filter(|p| p.parent() == Some(self.pid))
        {
            subs.push(p.pid());
            cpu += p.cpu_usage();
            mem += p.memory();
        }

        // TODO: 2 levels of parents PIDs at the moment - need to fix it / check if needed more
        for p in ps
            .iter()
            .map(|(_k, v)| v)
            .filter(|p| subs.contains(&p.parent().unwrap_or_else(|| Pid::from(0))))
        {
            cpu += p.cpu_usage();
            mem += p.memory();
        }

        self.readings.refresh(cpu, mem);
        self.cpu_info.poll(&self.readings);
        self.mem_info.poll(&self.readings);
        self.process_info.poll(&self.readings);
        Ok(())
    }
}
