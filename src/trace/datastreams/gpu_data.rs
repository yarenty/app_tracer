use serde_derive::{Deserialize, Serialize};

/// Represents a single GPU and its metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// GPU index (0-based)
    pub index: u32,
    /// GPU name/model
    pub name: String,
    /// GPU UUID for unique identification
    pub uuid: String,
    /// PCI bus ID
    pub pci_bus_id: String,
    /// Driver version
    pub driver_version: String,
    /// VBIOS version
    pub vbios_version: String,
    /// Compute capability (e.g., "8.6")
    pub compute_cap: String,
    /// Current performance state (P0-P12)
    pub pstate: String,
    /// Memory information
    pub memory: GpuMemory,
    /// Utilization metrics
    pub utilization: GpuUtilization,
    /// Temperature readings
    pub temperature: GpuTemperature,
    /// Power information
    pub power: GpuPower,
    /// Clock speeds
    pub clocks: GpuClocks,
    /// ECC status
    pub ecc: GpuEcc,
    /// PCIe information
    pub pcie: GpuPcie,
    /// Fan speed
    pub fan_speed: Option<u32>, // Percentage, may not be available on all GPUs
    /// Display status
    pub display_mode: String,
    /// Persistence mode
    pub persistence_mode: String,
    /// Compute mode
    pub compute_mode: String,
    /// Timestamp of the reading
    pub timestamp: String,
}

/// GPU memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuMemory {
    /// Total memory in MB
    pub total: u64,
    /// Used memory in MB
    pub used: u64,
    /// Free memory in MB
    pub free: u64,
    /// Reserved memory in MB
    pub reserved: u64,
    /// Protected memory total in MB (if available)
    pub protected_total: Option<u64>,
    /// Protected memory used in MB (if available)
    pub protected_used: Option<u64>,
    /// Protected memory free in MB (if available)
    pub protected_free: Option<u64>,
}

/// GPU utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuUtilization {
    /// GPU utilization percentage
    pub gpu: u32,
    /// Memory utilization percentage
    pub memory: u32,
    /// Encoder utilization percentage (if available)
    pub encoder: Option<u32>,
    /// Decoder utilization percentage (if available)
    pub decoder: Option<u32>,
    /// JPEG utilization percentage (if available)
    pub jpeg: Option<u32>,
    /// OFA utilization percentage (if available)
    pub ofa: Option<u32>,
}

/// GPU temperature readings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuTemperature {
    /// Core GPU temperature in Celsius
    pub gpu: Option<i32>,
    /// GPU temperature limit in Celsius
    pub gpu_tlimit: Option<i32>,
    /// Memory temperature in Celsius (HBM)
    pub memory: Option<i32>,
}

/// GPU power information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuPower {
    /// Current power draw in watts
    pub draw: Option<f32>,
    /// Average power draw in watts
    pub draw_average: Option<f32>,
    /// Instant power draw in watts
    pub draw_instant: Option<f32>,
    /// Power limit in watts
    pub limit: Option<f32>,
    /// Enforced power limit in watts
    pub enforced_limit: Option<f32>,
    /// Default power limit in watts
    pub default_limit: Option<f32>,
    /// Minimum power limit in watts
    pub min_limit: Option<f32>,
    /// Maximum power limit in watts
    pub max_limit: Option<f32>,
    /// Power management support status
    pub management: String,
}

/// GPU clock speeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuClocks {
    /// Current graphics clock in MHz
    pub graphics: Option<u32>,
    /// Current SM clock in MHz
    pub sm: Option<u32>,
    /// Current memory clock in MHz
    pub memory: Option<u32>,
    /// Current video clock in MHz
    pub video: Option<u32>,
    /// Maximum graphics clock in MHz
    pub max_graphics: Option<u32>,
    /// Maximum SM clock in MHz
    pub max_sm: Option<u32>,
    /// Maximum memory clock in MHz
    pub max_memory: Option<u32>,
    /// Application graphics clock in MHz
    pub app_graphics: Option<u32>,
    /// Application memory clock in MHz
    pub app_memory: Option<u32>,
}

/// GPU ECC (Error Correction Code) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuEcc {
    /// Current ECC mode
    pub mode_current: String,
    /// Pending ECC mode
    pub mode_pending: String,
    /// Corrected volatile errors
    pub errors_corrected_volatile: GpuEccErrors,
    /// Uncorrected volatile errors
    pub errors_uncorrected_volatile: GpuEccErrors,
    /// Corrected aggregate errors
    pub errors_corrected_aggregate: GpuEccErrors,
    /// Uncorrected aggregate errors
    pub errors_uncorrected_aggregate: GpuEccErrors,
}

/// ECC error counts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuEccErrors {
    /// Device memory errors
    pub device_memory: u64,
    /// DRAM errors
    pub dram: u64,
    /// Register file errors
    pub register_file: u64,
    /// L1 cache errors
    pub l1_cache: u64,
    /// L2 cache errors
    pub l2_cache: u64,
    /// Texture memory errors
    pub texture_memory: u64,
    /// CBU errors
    pub cbu: u64,
    /// SRAM errors
    pub sram: u64,
    /// Total errors
    pub total: u64,
}

/// PCIe information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuPcie {
    /// Current PCIe generation
    pub gen_current: Option<u32>,
    /// Maximum PCIe generation
    pub gen_max: Option<u32>,
    /// Current PCIe width
    pub width_current: Option<u32>,
    /// Maximum PCIe width
    pub width_max: Option<u32>,
    /// PCIe domain
    pub domain: Option<u32>,
    /// PCIe bus
    pub bus: Option<u32>,
    /// PCIe device
    pub device: Option<u32>,
}

/// GPU process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuProcess {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub process_name: String,
    /// GPU UUID this process is using
    pub gpu_uuid: String,
    /// Memory used by this process in MB
    pub used_memory: u64,
    /// GPU index
    pub gpu_index: u32,
}

/// Collection of all GPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuReadings {
    /// List of all detected GPUs
    pub gpus: Vec<GpuInfo>,
    /// GPU processes
    pub processes: Vec<GpuProcess>,
    /// Total number of GPUs detected
    pub gpu_count: u32,
    /// Timestamp of the reading
    pub timestamp: String,
}

impl Default for GpuEccErrors {
    fn default() -> Self {
        Self {
            device_memory: 0,
            dram: 0,
            register_file: 0,
            l1_cache: 0,
            l2_cache: 0,
            texture_memory: 0,
            cbu: 0,
            sram: 0,
            total: 0,
        }
    }
}

impl GpuReadings {
    /// Create a new empty GpuReadings structure
    pub fn new() -> Self {
        Self {
            gpus: Vec::new(),
            processes: Vec::new(),
            gpu_count: 0,
            timestamp: chrono::Utc::now().format("%Y/%m/%d %H:%M:%S%.3f").to_string(),
        }
    }

    /// Get GPU by index
    pub fn get_gpu(&self, index: u32) -> Option<&GpuInfo> {
        self.gpus.iter().find(|gpu| gpu.index == index)
    }

    /// Get GPU by UUID
    pub fn get_gpu_by_uuid(&self, uuid: &str) -> Option<&GpuInfo> {
        self.gpus.iter().find(|gpu| gpu.uuid == uuid)
    }

    /// Get processes for a specific GPU
    pub fn get_gpu_processes(&self, gpu_index: u32) -> Vec<&GpuProcess> {
        self.processes
            .iter()
            .filter(|proc| proc.gpu_index == gpu_index)
            .collect()
    }

    /// Get total memory usage across all GPUs
    pub fn get_total_memory_used(&self) -> u64 {
        self.gpus.iter().map(|gpu| gpu.memory.used).sum()
    }

    /// Get total memory across all GPUs
    pub fn get_total_memory(&self) -> u64 {
        self.gpus.iter().map(|gpu| gpu.memory.total).sum()
    }

    /// Get average GPU utilization across all GPUs
    pub fn get_average_gpu_utilization(&self) -> f32 {
        if self.gpus.is_empty() {
            return 0.0;
        }
        let total: u32 = self.gpus.iter().map(|gpu| gpu.utilization.gpu).sum();
        total as f32 / self.gpus.len() as f32
    }

    /// Get average temperature across all GPUs
    pub fn get_average_temperature(&self) -> Option<f32> {
        let temps: Vec<i32> = self
            .gpus
            .iter()
            .filter_map(|gpu| gpu.temperature.gpu)
            .collect();
        
        if temps.is_empty() {
            None
        } else {
            let sum: i32 = temps.iter().sum();
            Some(sum as f32 / temps.len() as f32)
        }
    }
}

impl Default for GpuReadings {
    fn default() -> Self {
        Self::new()
    }
}

/// CSV output record for GPU data
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GpuRecord<'a> {
    pub timestamp: &'a str,
    pub gpu_index: u32,
    pub gpu_name: &'a str,
    pub memory_used: u64,
    pub memory_total: u64,
    pub memory_free: u64,
    pub gpu_utilization: u32,
    pub memory_utilization: u32,
    pub temperature: Option<i32>,
    pub power_draw: Option<f32>,
    pub graphics_clock: Option<u32>,
    pub memory_clock: Option<u32>,
    pub processes_count: usize,
}

impl<'a> GpuRecord<'a> {
    pub fn new(
        timestamp: &'a str,
        gpu: &'a GpuInfo,
        processes_count: usize,
    ) -> Self {
        Self {
            timestamp,
            gpu_index: gpu.index,
            gpu_name: &gpu.name,
            memory_used: gpu.memory.used,
            memory_total: gpu.memory.total,
            memory_free: gpu.memory.free,
            gpu_utilization: gpu.utilization.gpu,
            memory_utilization: gpu.utilization.memory,
            temperature: gpu.temperature.gpu,
            power_draw: gpu.power.draw,
            graphics_clock: gpu.clocks.graphics,
            memory_clock: gpu.clocks.memory,
            processes_count,
        }
    }
}
