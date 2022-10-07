use crate::trace::datastreams::{
    data_stream::{Readings, SysDataStream},
    utils,
};

pub struct CPUMonitor {
    pub cpu_usage: f32,
    pub cpu_usage_history: Vec<f32>,
    max_history_len: usize,
    interpolation_len: u16,
}

impl SysDataStream for CPUMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            cpu_usage: 0.0,
            cpu_usage_history: vec![0.0; max_hist_len],
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
        }
    }

    fn poll(&mut self, system_info: &Readings) {
        self.cpu_usage = system_info.get_cpu();

        while self.cpu_usage_history.len() >= self.max_history_len {
            self.cpu_usage_history.remove(0);
        }
        let last_mem = match self.cpu_usage_history.last() {
            Some(l) => *l,
            None => 0.0,
        };
        self.cpu_usage_history.extend_from_slice(
            utils::interpolate(last_mem, self.cpu_usage / 100.0, self.interpolation_len).as_slice(),
        );
    }
}
