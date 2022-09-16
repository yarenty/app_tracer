use sysinfo::{Pid, System as SysInfoSystem};
#[allow(unused_imports)]
use crate::error::{Result, TraceError};

pub trait SysDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn poll(&mut self, system_info: &mut SysInfoSystem, pid: &Pid);
}

