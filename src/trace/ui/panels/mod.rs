mod cpu_usage;
mod memory_usage;
mod process;
mod gpu_memory;
mod gpu_utilization;
mod gpu_temperature;
mod gpu_power;
mod gpu_processes;

pub use self::cpu_usage::cpu_usage_history_panel;
pub use self::memory_usage::mem_history_panel;
pub use self::process::process_panel;

// GPU panels
pub use self::gpu_memory::gpu_memory_panel;
pub use self::gpu_utilization::gpu_utilization_panel;
pub use self::gpu_temperature::gpu_temperature_panel;
pub use self::gpu_power::gpu_power_panel;
pub use self::gpu_processes::{gpu_processes_panel, gpu_summary_panel};

mod utils;
