use crate::trace::datastreams::gpu_data::{GpuReadings, GpuInfo};
use crate::trace::datastreams::data_stream::SysDataStream;
use crate::trace::datastreams::utils;

/// GPU Memory Monitor - tracks GPU memory usage over time
pub struct GpuMemoryMonitor {
    pub memory_usage_history: Vec<f64>, // Memory usage percentage
    pub memory_used_history: Vec<u64>,  // Absolute memory used in MB
    pub memory_total: u64,
    pub current_usage: f64,
    pub current_used: u64,
    pub current_free: u64,
    max_history_len: usize,
    interpolation_len: u16,
    gpu_index: u32,
}

impl SysDataStream for GpuMemoryMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            memory_usage_history: vec![0.0; max_hist_len],
            memory_used_history: vec![0; max_hist_len],
            memory_total: 0,
            current_usage: 0.0,
            current_used: 0,
            current_free: 0,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index: 0,
        }
    }

    fn poll(&mut self, _system_info: &crate::trace::datastreams::data_stream::Readings) {
        // This will be updated to work with GPU data
        // For now, we'll implement a placeholder
    }
}

impl GpuMemoryMonitor {
    pub fn new_for_gpu(max_hist_len: usize, inter_len: u16, gpu_index: u32) -> Self {
        Self {
            memory_usage_history: vec![0.0; max_hist_len],
            memory_used_history: vec![0; max_hist_len],
            memory_total: 0,
            current_usage: 0.0,
            current_used: 0,
            current_free: 0,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index,
        }
    }

    pub fn poll_gpu(&mut self, gpu_info: &GpuInfo) {
        self.memory_total = gpu_info.memory.total;
        self.current_used = gpu_info.memory.used;
        self.current_free = gpu_info.memory.free;
        self.current_usage = if gpu_info.memory.total > 0 {
            (gpu_info.memory.used as f64 / gpu_info.memory.total as f64) * 100.0
        } else {
            0.0
        };

        // Update history
        while self.memory_usage_history.len() >= self.max_history_len {
            self.memory_usage_history.remove(0);
        }
        while self.memory_used_history.len() >= self.max_history_len {
            self.memory_used_history.remove(0);
        }

        let last_usage = self.memory_usage_history.last().copied().unwrap_or(0.0);
        let last_used = self.memory_used_history.last().copied().unwrap_or(0);

        self.memory_usage_history.extend_from_slice(
            utils::interpolate(
                last_usage,
                self.current_usage / 100.0,
                self.interpolation_len,
            )
            .as_slice(),
        );

        self.memory_used_history.extend_from_slice(
            utils::interpolate(
                last_used as f64,
                self.current_used as f64,
                self.interpolation_len,
            )
            .iter()
            .map(|&x| x as u64)
            .collect::<Vec<u64>>()
            .as_slice(),
        );
    }
}

/// GPU Utilization Monitor - tracks GPU utilization over time
pub struct GpuUtilizationMonitor {
    pub gpu_utilization_history: Vec<f32>,
    pub memory_utilization_history: Vec<f32>,
    pub current_gpu_util: u32,
    pub current_memory_util: u32,
    max_history_len: usize,
    interpolation_len: u16,
    gpu_index: u32,
}

impl SysDataStream for GpuUtilizationMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            gpu_utilization_history: vec![0.0; max_hist_len],
            memory_utilization_history: vec![0.0; max_hist_len],
            current_gpu_util: 0,
            current_memory_util: 0,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index: 0,
        }
    }

    fn poll(&mut self, _system_info: &crate::trace::datastreams::data_stream::Readings) {
        // This will be updated to work with GPU data
    }
}

impl GpuUtilizationMonitor {
    pub fn new_for_gpu(max_hist_len: usize, inter_len: u16, gpu_index: u32) -> Self {
        Self {
            gpu_utilization_history: vec![0.0; max_hist_len],
            memory_utilization_history: vec![0.0; max_hist_len],
            current_gpu_util: 0,
            current_memory_util: 0,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index,
        }
    }

    pub fn poll_gpu(&mut self, gpu_info: &GpuInfo) {
        self.current_gpu_util = gpu_info.utilization.gpu;
        self.current_memory_util = gpu_info.utilization.memory;

        // Update GPU utilization history
        while self.gpu_utilization_history.len() >= self.max_history_len {
            self.gpu_utilization_history.remove(0);
        }
        let last_gpu_util = self.gpu_utilization_history.last().copied().unwrap_or(0.0);
        self.gpu_utilization_history.extend_from_slice(
            utils::interpolate(
                last_gpu_util,
                self.current_gpu_util as f32 / 100.0,
                self.interpolation_len,
            )
            .as_slice(),
        );

        // Update memory utilization history
        while self.memory_utilization_history.len() >= self.max_history_len {
            self.memory_utilization_history.remove(0);
        }
        let last_mem_util = self.memory_utilization_history.last().copied().unwrap_or(0.0);
        self.memory_utilization_history.extend_from_slice(
            utils::interpolate(
                last_mem_util,
                self.current_memory_util as f32 / 100.0,
                self.interpolation_len,
            )
            .as_slice(),
        );
    }
}

/// GPU Temperature Monitor - tracks GPU temperature over time
pub struct GpuTemperatureMonitor {
    pub temperature_history: Vec<f32>,
    pub current_temperature: Option<i32>,
    pub max_temperature: Option<i32>,
    pub memory_temperature: Option<i32>,
    max_history_len: usize,
    interpolation_len: u16,
    gpu_index: u32,
}

impl SysDataStream for GpuTemperatureMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            temperature_history: vec![0.0; max_hist_len],
            current_temperature: None,
            max_temperature: None,
            memory_temperature: None,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index: 0,
        }
    }

    fn poll(&mut self, _system_info: &crate::trace::datastreams::data_stream::Readings) {
        // This will be updated to work with GPU data
    }
}

impl GpuTemperatureMonitor {
    pub fn new_for_gpu(max_hist_len: usize, inter_len: u16, gpu_index: u32) -> Self {
        Self {
            temperature_history: vec![0.0; max_hist_len],
            current_temperature: None,
            max_temperature: None,
            memory_temperature: None,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index,
        }
    }

    pub fn poll_gpu(&mut self, gpu_info: &GpuInfo) {
        self.current_temperature = gpu_info.temperature.gpu;
        self.max_temperature = gpu_info.temperature.gpu_tlimit;
        self.memory_temperature = gpu_info.temperature.memory;

        if let Some(temp) = self.current_temperature {
            while self.temperature_history.len() >= self.max_history_len {
                self.temperature_history.remove(0);
            }
            let last_temp = self.temperature_history.last().copied().unwrap_or(temp as f32);
            self.temperature_history.extend_from_slice(
                utils::interpolate(
                    last_temp,
                    temp as f32,
                    self.interpolation_len,
                )
                .as_slice(),
            );
        }
    }
}

/// GPU Power Monitor - tracks GPU power consumption over time
pub struct GpuPowerMonitor {
    pub power_draw_history: Vec<f32>,
    pub current_power_draw: Option<f32>,
    pub power_limit: Option<f32>,
    pub power_management: String,
    max_history_len: usize,
    interpolation_len: u16,
    gpu_index: u32,
}

impl SysDataStream for GpuPowerMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            power_draw_history: vec![0.0; max_hist_len],
            current_power_draw: None,
            power_limit: None,
            power_management: String::new(),
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index: 0,
        }
    }

    fn poll(&mut self, _system_info: &crate::trace::datastreams::data_stream::Readings) {
        // This will be updated to work with GPU data
    }
}

impl GpuPowerMonitor {
    pub fn new_for_gpu(max_hist_len: usize, inter_len: u16, gpu_index: u32) -> Self {
        Self {
            power_draw_history: vec![0.0; max_hist_len],
            current_power_draw: None,
            power_limit: None,
            power_management: String::new(),
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index,
        }
    }

    pub fn poll_gpu(&mut self, gpu_info: &GpuInfo) {
        self.current_power_draw = gpu_info.power.draw;
        self.power_limit = gpu_info.power.limit;
        self.power_management = gpu_info.power.management.clone();

        if let Some(power) = self.current_power_draw {
            while self.power_draw_history.len() >= self.max_history_len {
                self.power_draw_history.remove(0);
            }
            let last_power = self.power_draw_history.last().copied().unwrap_or(power);
            self.power_draw_history.extend_from_slice(
                utils::interpolate(
                    last_power,
                    power,
                    self.interpolation_len,
                )
                .as_slice(),
            );
        }
    }
}

/// GPU Clock Monitor - tracks GPU clock speeds over time
pub struct GpuClockMonitor {
    pub graphics_clock_history: Vec<f32>,
    pub memory_clock_history: Vec<f32>,
    pub current_graphics_clock: Option<u32>,
    pub current_memory_clock: Option<u32>,
    pub max_graphics_clock: Option<u32>,
    pub max_memory_clock: Option<u32>,
    max_history_len: usize,
    interpolation_len: u16,
    gpu_index: u32,
}

impl SysDataStream for GpuClockMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            graphics_clock_history: vec![0.0; max_hist_len],
            memory_clock_history: vec![0.0; max_hist_len],
            current_graphics_clock: None,
            current_memory_clock: None,
            max_graphics_clock: None,
            max_memory_clock: None,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index: 0,
        }
    }

    fn poll(&mut self, _system_info: &crate::trace::datastreams::data_stream::Readings) {
        // This will be updated to work with GPU data
    }
}

impl GpuClockMonitor {
    pub fn new_for_gpu(max_hist_len: usize, inter_len: u16, gpu_index: u32) -> Self {
        Self {
            graphics_clock_history: vec![0.0; max_hist_len],
            memory_clock_history: vec![0.0; max_hist_len],
            current_graphics_clock: None,
            current_memory_clock: None,
            max_graphics_clock: None,
            max_memory_clock: None,
            max_history_len: max_hist_len,
            interpolation_len: inter_len,
            gpu_index,
        }
    }

    pub fn poll_gpu(&mut self, gpu_info: &GpuInfo) {
        self.current_graphics_clock = gpu_info.clocks.graphics;
        self.current_memory_clock = gpu_info.clocks.memory;
        self.max_graphics_clock = gpu_info.clocks.max_graphics;
        self.max_memory_clock = gpu_info.clocks.max_memory;

        // Update graphics clock history
        if let Some(clock) = self.current_graphics_clock {
            while self.graphics_clock_history.len() >= self.max_history_len {
                self.graphics_clock_history.remove(0);
            }
            let last_clock = self.graphics_clock_history.last().copied().unwrap_or(clock as f32);
            self.graphics_clock_history.extend_from_slice(
                utils::interpolate(
                    last_clock,
                    clock as f32,
                    self.interpolation_len,
                )
                .as_slice(),
            );
        }

        // Update memory clock history
        if let Some(clock) = self.current_memory_clock {
            while self.memory_clock_history.len() >= self.max_history_len {
                self.memory_clock_history.remove(0);
            }
            let last_clock = self.memory_clock_history.last().copied().unwrap_or(clock as f32);
            self.memory_clock_history.extend_from_slice(
                utils::interpolate(
                    last_clock,
                    clock as f32,
                    self.interpolation_len,
                )
                .as_slice(),
            );
        }
    }
}

/// GPU Process Monitor - tracks GPU processes
pub struct GpuProcessMonitor {
    pub processes: Vec<crate::trace::datastreams::gpu_data::GpuProcess>,
    pub gpu_index: u32,
}

impl SysDataStream for GpuProcessMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {
        Self {
            processes: Vec::new(),
            gpu_index: 0,
        }
    }

    fn poll(&mut self, _system_info: &crate::trace::datastreams::data_stream::Readings) {
        // This will be updated to work with GPU data
    }
}

impl GpuProcessMonitor {
    pub fn new_for_gpu(gpu_index: u32) -> Self {
        Self {
            processes: Vec::new(),
            gpu_index,
        }
    }

    pub fn poll_gpu(&mut self, gpu_readings: &GpuReadings) {
        self.processes.clear();
        self.processes.extend(
            gpu_readings
                .get_gpu_processes(self.gpu_index)
                .iter()
                .map(|p| (*p).clone())
        );
    }
}
