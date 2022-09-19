use sysinfo::{Pid, PidExt, Process, ProcessExt, System, SystemExt};

/// Cache structure between calls to system.refresh - to make sure all output is the same,
/// and get proper CPU readings (they need some time between consecutive calls)
pub struct Readings {
    pid: u32,
    process: String,
    cpu: f32,
    mem: u64,
    total: u64,
    cpus_no: usize,
}

impl Readings {
    /// Getting initial data - refreshing whole system struct.
    pub fn new(sys: &mut System, pid: Pid) -> Self {
        sys.refresh_all();
        let process = sys.process(pid).unwrap();
        Readings {
            pid: process.pid().as_u32(),
            process: String::from(process.name()),
            cpu: process.cpu_usage(),
            mem: (process.memory() + process.virtual_memory()) / 1024,
            total: sys.total_memory() / 1024,
            cpus_no: sys.cpus().len(),
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

    pub fn get_cpus_count(&self) -> usize {
        self.cpus_no
    }

    /// Refreshing only data provided by traced process
    pub fn refresh(&mut self, process: &Process) {
        self.cpu = process.cpu_usage();
        self.mem = process.memory() / 1024;
        self.pid = process.pid().as_u32();
        self.process = String::from(process.name());
    }
}

pub trait SysDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn poll(&mut self, system_info: &Readings);
}
