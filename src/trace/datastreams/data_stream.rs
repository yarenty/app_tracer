use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};

pub struct Readings {
    pid: u32,
    process: String,
    cpu: f32,
    mem: u64,
    total: u64,
}

impl Readings {
    pub fn new(sys: &mut System, pid: Pid) -> Self {
        sys.refresh_all();
        let process = sys.process(pid).unwrap();
        Readings {
            pid: process.pid().as_u32(),
            process: String::from(process.name()),
            cpu: process.cpu_usage(),
            mem: process.memory(),
            total: sys.total_memory(),
        }
    }

    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    pub fn get_process(&self) -> String {
        self.process.clone()
    }

    pub fn get_cpu(&self) -> f32 {
        self.cpu
    }

    pub fn get_mem(&self) -> u64 {
        self.mem
    }

    pub fn get_total_memory(&self) -> u64 {
        self.total
    }

    pub fn refresh(&mut self, process: &Process) {
        self.cpu = process.cpu_usage();
        self.mem = process.memory();
        self.pid = process.pid().as_u32();
        self.process = String::from(process.name());
    }
}

pub trait SysDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn poll(&mut self, system_info: &Readings);
}
