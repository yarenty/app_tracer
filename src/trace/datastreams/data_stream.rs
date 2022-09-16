use sysinfo::{Pid, System};

pub trait SysDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn poll(&mut self, system_info: &mut System, pid: &Pid);
}
