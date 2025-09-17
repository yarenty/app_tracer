pub mod trace;
pub mod args;
pub mod error;
pub mod utils;

// Re-export commonly used types
pub use trace::datastreams::{
    NvidiaSmiMonitor, GpuReadings, GpuInfo, GpuMemory, GpuUtilization, 
    GpuTemperature, GpuPower, GpuClocks, GpuProcess
};
pub use trace::{Record, GpuCsvRecord};
pub use args::Args;