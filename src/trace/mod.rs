use serde_derive::{Deserialize, Serialize};

pub mod app;
pub mod cmd;
pub mod datastreams;
pub mod event;
pub mod ui;

mod app_data_streams;

/// CSV output record for CPU monitoring (legacy)
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Record<'a> {
    pub time: &'a str,
    pub cpu: &'a str,
    pub mem: &'a str,
}

impl Record<'static> {
    pub fn new<'a>(time: &'a str, cpu: &'a str, mem: &'a str) -> Record<'a> {
        Record { time, cpu, mem }
    }
}

/// CSV output record for GPU monitoring
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GpuCsvRecord {
    pub timestamp: String,
    pub gpu_index: u32,
    pub gpu_name: String,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_free_mb: u64,
    pub memory_usage_percent: f64,
    pub gpu_utilization_percent: u32,
    pub memory_utilization_percent: u32,
    pub temperature_celsius: Option<i32>,
    pub power_draw_watts: Option<f32>,
    pub graphics_clock_mhz: Option<u32>,
    pub memory_clock_mhz: Option<u32>,
    pub processes_count: usize,
    pub pstate: String,
    pub driver_version: String,
    pub compute_capability: String,
}

impl GpuCsvRecord {
    pub fn from_gpu_info(gpu_info: &crate::trace::datastreams::GpuInfo, processes_count: usize) -> Self {
        let memory_usage_percent = if gpu_info.memory.total > 0 {
            (gpu_info.memory.used as f64 / gpu_info.memory.total as f64) * 100.0
        } else {
            0.0
        };

        Self {
            timestamp: gpu_info.timestamp.clone(),
            gpu_index: gpu_info.index,
            gpu_name: gpu_info.name.clone(),
            memory_used_mb: gpu_info.memory.used,
            memory_total_mb: gpu_info.memory.total,
            memory_free_mb: gpu_info.memory.free,
            memory_usage_percent,
            gpu_utilization_percent: gpu_info.utilization.gpu,
            memory_utilization_percent: gpu_info.utilization.memory,
            temperature_celsius: gpu_info.temperature.gpu,
            power_draw_watts: gpu_info.power.draw,
            graphics_clock_mhz: gpu_info.clocks.graphics,
            memory_clock_mhz: gpu_info.clocks.memory,
            processes_count,
            pstate: gpu_info.pstate.clone(),
            driver_version: gpu_info.driver_version.clone(),
            compute_capability: gpu_info.compute_cap.clone(),
        }
    }
}
