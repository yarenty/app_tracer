use crate::trace::datastreams::gpu_data::{
    GpuClocks, GpuEcc, GpuEccErrors, GpuInfo, GpuMemory, GpuPcie, GpuPower, 
    GpuProcess, GpuReadings, GpuTemperature, GpuUtilization,
};
use std::process::Command;

/// nvidia-smi integration for GPU monitoring
pub struct NvidiaSmiMonitor {
    /// Path to nvidia-smi executable
    nvidia_smi_path: String,
    /// Whether nvidia-smi is available
    available: bool,
    /// Last error message
    last_error: Option<String>,
}

impl NvidiaSmiMonitor {
    /// Create a new NvidiaSmiMonitor instance
    pub fn new() -> Self {
        let path = Self::find_nvidia_smi_path();
        let available = Self::check_availability(&path);
        
        Self {
            nvidia_smi_path: path,
            available,
            last_error: None,
        }
    }

    /// Find nvidia-smi executable path
    fn find_nvidia_smi_path() -> String {
        // Try common paths for nvidia-smi
        let common_paths = vec![
            "nvidia-smi",
            "/usr/bin/nvidia-smi",
            "/usr/local/bin/nvidia-smi",
            "/opt/nvidia/bin/nvidia-smi",
            "C:\\Program Files\\NVIDIA Corporation\\NVSMI\\nvidia-smi.exe",
            "C:\\Windows\\System32\\nvidia-smi.exe",
        ];

        for path in common_paths {
            if Self::check_command_exists(path) {
                return path.to_string();
            }
        }

        "nvidia-smi".to_string() // Fallback to PATH
    }

    /// Check if a command exists and is executable
    fn check_command_exists(command: &str) -> bool {
        let result = if cfg!(target_os = "windows") {
            Command::new("where")
                .arg(command)
                .output()
        } else {
            Command::new("which")
                .arg(command)
                .output()
        };

        result.map(|output| output.status.success()).unwrap_or(false)
    }

    /// Check if nvidia-smi is available and working
    fn check_availability(path: &str) -> bool {
        let result = Command::new(path)
            .arg("--query-gpu=count")
            .arg("--format=csv,noheader,nounits")
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    count_str.parse::<u32>().is_ok()
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    /// Execute nvidia-smi command and return output
    fn execute_command(&self, args: &[&str]) -> Result<String, String> {
        if !self.available {
            return Err("nvidia-smi is not available".to_string());
        }

        let output = Command::new(&self.nvidia_smi_path)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute nvidia-smi: {}", e))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            let stdout_msg = String::from_utf8_lossy(&output.stdout);
            return Err(format!("nvidia-smi command failed: stderr='{}', stdout='{}', exit_code={:?}", 
                error_msg, stdout_msg, output.status.code()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get GPU count
    pub fn get_gpu_count(&self) -> Result<u32, String> {
        let output = self.execute_command(&["--query-gpu=count", "--format=csv,noheader,nounits"])?;
        output.trim().parse::<u32>()
            .map_err(|e| format!("Failed to parse GPU count: {}", e))
    }

    /// Get basic GPU information
    pub fn get_gpu_info(&self) -> Result<GpuReadings, String> {
        let mut readings = GpuReadings::new();
        
        // Get GPU count
        readings.gpu_count = self.get_gpu_count()?;
        
        if readings.gpu_count == 0 {
            return Ok(readings);
        }

        // Query all GPU metrics (removed invalid fields)
        let gpu_query = [
            "timestamp", "name", "uuid", "pci.bus_id", "driver_version", "vbios_version",
            "compute_cap", "pstate", "memory.total", "memory.used", "memory.free", "memory.reserved",
            "utilization.gpu", "utilization.memory", "utilization.encoder", "utilization.decoder",
            "utilization.jpeg", "utilization.ofa", "temperature.gpu", "temperature.gpu.tlimit",
            "temperature.memory", "power.draw", "power.draw.average", "power.draw.instant",
            "power.limit", "enforced.power.limit", "power.default_limit", "power.min_limit",
            "power.max_limit", "power.management", "clocks.current.graphics", "clocks.current.sm",
            "clocks.current.memory", "clocks.current.video", "clocks.max.graphics", "clocks.max.sm",
            "clocks.max.memory", "clocks.applications.graphics", "clocks.applications.memory",
            "ecc.mode.current", "ecc.mode.pending", "pcie.link.gen.current", "pcie.link.gen.max",
            "pcie.link.width.current", "pcie.link.width.max", "fan.speed", "display_mode", 
            "persistence_mode", "compute_mode", "index"
        ];

        let query_str = gpu_query.join(",");
        let output = self.execute_command(&[
            "--query-gpu",
            &query_str,
            "--format=csv,noheader,nounits"
        ])?;

        // Parse CSV output
        let lines: Vec<&str> = output.trim().lines().collect();
        for line in lines {
            if let Ok(gpu_info) = self.parse_gpu_line(line, gpu_query.len()) {
                readings.gpus.push(gpu_info);
            }
        }

        // Get GPU processes
        readings.processes = self.get_gpu_processes()?;
        
        readings.timestamp = chrono::Utc::now().format("%Y/%m/%d %H:%M:%S%.3f").to_string();
        
        Ok(readings)
    }

    /// Parse a single GPU line from CSV output
    fn parse_gpu_line(&self, line: &str, expected_fields: usize) -> Result<GpuInfo, String> {
        let fields: Vec<&str> = line.split(',').collect();
        
        if fields.len() < expected_fields {
            return Err(format!("Expected {} fields, got {}", expected_fields, fields.len()));
        }

        let mut field_iter = fields.iter();
        
        // Helper function to parse optional numeric values
        let parse_optional_u32 = |s: &str| -> Option<u32> {
            if s.is_empty() || s == "N/A" { None } else { s.parse().ok() }
        };
        
        let parse_optional_f32 = |s: &str| -> Option<f32> {
            if s.is_empty() || s == "N/A" { None } else { s.parse().ok() }
        };
        
        let parse_optional_i32 = |s: &str| -> Option<i32> {
            if s.is_empty() || s == "N/A" { None } else { s.parse().ok() }
        };

        let timestamp = field_iter.next().map_or("", |v| v).to_string();
        let name = field_iter.next().map_or("Unknown", |v| v).to_string();
        let uuid = field_iter.next().map_or("", |v| v).to_string();
        let pci_bus_id = field_iter.next().map_or("", |v| v).to_string();
        let driver_version = field_iter.next().map_or("", |v| v).to_string();
        let vbios_version = field_iter.next().map_or("", |v| v).to_string();
        let compute_cap = field_iter.next().map_or("", |v| v).to_string();
        let pstate = field_iter.next().map_or("", |v| v).to_string();
        
        // Memory fields
        let memory_total = parse_optional_u32(field_iter.next().map_or("0", |v| v)).unwrap_or(0) as u64;
        let memory_used = parse_optional_u32(field_iter.next().map_or("0", |v| v)).unwrap_or(0) as u64;
        let memory_free = parse_optional_u32(field_iter.next().map_or("0", |v| v)).unwrap_or(0) as u64;
        let memory_reserved = parse_optional_u32(field_iter.next().map_or("0", |v| v)).unwrap_or(0) as u64;
        
        // Utilization fields
        let gpu_util = parse_optional_u32(field_iter.next().map_or("0", |v| v)).unwrap_or(0);
        let mem_util = parse_optional_u32(field_iter.next().map_or("0", |v| v)).unwrap_or(0);
        let encoder_util = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let decoder_util = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let jpeg_util = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let ofa_util = parse_optional_u32(field_iter.next().map_or("", |v| v));
        
        // Temperature fields
        let temp_gpu = parse_optional_i32(field_iter.next().map_or("", |v| v));
        let temp_gpu_tlimit = parse_optional_i32(field_iter.next().map_or("", |v| v));
        let temp_memory = parse_optional_i32(field_iter.next().map_or("", |v| v));
        
        // Power fields
        let power_draw = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_draw_avg = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_draw_instant = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_limit = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_enforced_limit = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_default_limit = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_min_limit = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_max_limit = parse_optional_f32(field_iter.next().map_or("", |v| v));
        let power_management = field_iter.next().map_or("", |v| v).to_string();
        
        // Clock fields
        let clock_graphics = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_sm = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_memory = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_video = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_max_graphics = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_max_sm = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_max_memory = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_app_graphics = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let clock_app_memory = parse_optional_u32(field_iter.next().map_or("", |v| v));
        
        // ECC fields
        let ecc_mode_current = field_iter.next().map_or("", |v| v).to_string();
        let ecc_mode_pending = field_iter.next().map_or("", |v| v).to_string();
        
        // PCIe fields (removed domain, bus, device as they're not valid query fields)
        let pcie_gen_current = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let pcie_gen_max = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let pcie_width_current = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let pcie_width_max = parse_optional_u32(field_iter.next().map_or("", |v| v));
        
        // Other fields
        let fan_speed = parse_optional_u32(field_iter.next().map_or("", |v| v));
        let display_mode = field_iter.next().map_or("", |v| v).to_string();
        let persistence_mode = field_iter.next().map_or("", |v| v).to_string();
        let compute_mode = field_iter.next().map_or("", |v| v).to_string();
        let index = parse_optional_u32(field_iter.next().map_or("0", |v| v)).unwrap_or(0);

        Ok(GpuInfo {
            index,
            name,
            uuid,
            pci_bus_id,
            driver_version,
            vbios_version,
            compute_cap,
            pstate,
            memory: GpuMemory {
                total: memory_total,
                used: memory_used,
                free: memory_free,
                reserved: memory_reserved,
                protected_total: None,
                protected_used: None,
                protected_free: None,
            },
            utilization: GpuUtilization {
                gpu: gpu_util,
                memory: mem_util,
                encoder: encoder_util,
                decoder: decoder_util,
                jpeg: jpeg_util,
                ofa: ofa_util,
            },
            temperature: GpuTemperature {
                gpu: temp_gpu,
                gpu_tlimit: temp_gpu_tlimit,
                memory: temp_memory,
            },
            power: GpuPower {
                draw: power_draw,
                draw_average: power_draw_avg,
                draw_instant: power_draw_instant,
                limit: power_limit,
                enforced_limit: power_enforced_limit,
                default_limit: power_default_limit,
                min_limit: power_min_limit,
                max_limit: power_max_limit,
                management: power_management,
            },
            clocks: GpuClocks {
                graphics: clock_graphics,
                sm: clock_sm,
                memory: clock_memory,
                video: clock_video,
                max_graphics: clock_max_graphics,
                max_sm: clock_max_sm,
                max_memory: clock_max_memory,
                app_graphics: clock_app_graphics,
                app_memory: clock_app_memory,
            },
            ecc: GpuEcc {
                mode_current: ecc_mode_current,
                mode_pending: ecc_mode_pending,
                errors_corrected_volatile: GpuEccErrors::default(),
                errors_uncorrected_volatile: GpuEccErrors::default(),
                errors_corrected_aggregate: GpuEccErrors::default(),
                errors_uncorrected_aggregate: GpuEccErrors::default(),
            },
            pcie: GpuPcie {
                gen_current: pcie_gen_current,
                gen_max: pcie_gen_max,
                width_current: pcie_width_current,
                width_max: pcie_width_max,
                domain: None, // Not available via nvidia-smi query
                bus: None,    // Not available via nvidia-smi query
                device: None, // Not available via nvidia-smi query
            },
            fan_speed,
            display_mode,
            persistence_mode,
            compute_mode,
            timestamp,
        })
    }

    /// Get GPU processes
    fn get_gpu_processes(&self) -> Result<Vec<GpuProcess>, String> {
        let output = self.execute_command(&[
            "--query-compute-apps=pid,process_name,gpu_uuid,used_memory",
            "--format=csv,noheader,nounits"
        ])?;

        let mut processes = Vec::new();
        let lines: Vec<&str> = output.trim().lines().collect();
        
        for line in lines {
            if line.is_empty() {
                continue;
            }
            
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() >= 4 {
                if let (Ok(pid), Ok(used_memory)) = (
                    fields[0].parse::<u32>(),
                    fields[3].parse::<u64>()
                ) {
                    processes.push(GpuProcess {
                        pid,
                        process_name: fields[1].to_string(),
                        gpu_uuid: fields[2].to_string(),
                        used_memory,
                        gpu_index: 0, // Will be updated later
                    });
                }
            }
        }

        Ok(processes)
    }

    /// Check if nvidia-smi is available
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Get the last error message
    pub fn get_last_error(&self) -> Option<&String> {
        self.last_error.as_ref()
    }
}

impl Default for NvidiaSmiMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvidia_smi_availability() {
        let monitor = NvidiaSmiMonitor::new();
        // This test will pass if nvidia-smi is available, fail if not
        // In CI/CD environments without GPUs, this test should be skipped
        if monitor.is_available() {
            println!("nvidia-smi is available for testing");
        } else {
            println!("nvidia-smi is not available - skipping GPU tests");
        }
    }

    #[test]
    fn test_gpu_count() {
        let monitor = NvidiaSmiMonitor::new();
        if monitor.is_available() {
            match monitor.get_gpu_count() {
                Ok(count) => {
                    println!("Found {} GPU(s)", count);
                    assert!(count >= 0);
                }
                Err(e) => {
                    println!("Failed to get GPU count: {}", e);
                    // Don't fail the test if nvidia-smi is not working
                }
            }
        }
    }

    #[test]
    fn test_gpu_info_parsing() {
        let monitor = NvidiaSmiMonitor::new();
        if monitor.is_available() {
            match monitor.get_gpu_info() {
                Ok(readings) => {
                    println!("Successfully parsed GPU info for {} GPU(s)", readings.gpu_count);
                    assert_eq!(readings.gpus.len(), readings.gpu_count as usize);
                    
                    for gpu in &readings.gpus {
                        println!("GPU {}: {} (UUID: {})", gpu.index, gpu.name, gpu.uuid);
                        println!("  Memory: {} MB / {} MB", gpu.memory.used, gpu.memory.total);
                        println!("  Utilization: GPU {}%, Memory {}%", gpu.utilization.gpu, gpu.utilization.memory);
                        if let Some(temp) = gpu.temperature.gpu {
                            println!("  Temperature: {}°C", temp);
                        }
                        if let Some(power) = gpu.power.draw {
                            println!("  Power: {}W", power);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to get GPU info: {}", e);
                    // Don't fail the test if nvidia-smi is not working
                }
            }
        }
    }
}
