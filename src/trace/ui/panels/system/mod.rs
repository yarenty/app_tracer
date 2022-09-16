mod cpuusage;
mod processes;
mod memoryusage;

pub use self::cpuusage::cpu_usage_history_panel as cpu_usage_history_panel;
pub use self::processes::processes_panel as processes_panel;
pub use self::memoryusage::mem_and_swap_history_panel as mem_and_swap_history_panel;
