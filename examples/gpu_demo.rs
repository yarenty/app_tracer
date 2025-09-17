use app_tracer::{NvidiaSmiMonitor, GpuReadings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GPU Tracer Demo - nvidia-smi Integration");
    println!("==========================================");
    
    // Initialize nvidia-smi monitor
    let monitor = NvidiaSmiMonitor::new();
    
    if !monitor.is_available() {
        println!("❌ nvidia-smi is not available on this system");
        println!("   This demo requires NVIDIA GPUs with nvidia-smi installed");
        return Ok(());
    }
    
    println!("✅ nvidia-smi is available");
    
    // Get GPU count
    match monitor.get_gpu_count() {
        Ok(count) => {
            println!("🔍 Found {} GPU(s)", count);
            
            if count == 0 {
                println!("   No GPUs detected");
                return Ok(());
            }
        }
        Err(e) => {
            println!("❌ Failed to get GPU count: {}", e);
            return Ok(());
        }
    }
    
    // Get detailed GPU information
    println!("\n🔍 Fetching detailed GPU information...");
    match monitor.get_gpu_info() {
        Ok(readings) => {
            println!("\n📊 GPU Information:");
            println!("===================");
            
            for gpu in &readings.gpus {
                println!("\n🖥️  GPU {}: {}", gpu.index, gpu.name);
                println!("   UUID: {}", gpu.uuid);
                println!("   Driver: {}", gpu.driver_version);
                println!("   Compute Capability: {}", gpu.compute_cap);
                println!("   P-State: {}", gpu.pstate);
                
                // Memory information
                println!("   💾 Memory: {} MB / {} MB ({:.1}% used)", 
                    gpu.memory.used, 
                    gpu.memory.total,
                    if gpu.memory.total > 0 {
                        (gpu.memory.used as f64 / gpu.memory.total as f64) * 100.0
                    } else { 0.0 }
                );
                
                // Utilization
                println!("   ⚡ Utilization: GPU {}%, Memory {}%", 
                    gpu.utilization.gpu, 
                    gpu.utilization.memory
                );
                
                // Temperature
                if let Some(temp) = gpu.temperature.gpu {
                    println!("   🌡️  Temperature: {}°C", temp);
                }
                
                // Power
                if let Some(power) = gpu.power.draw {
                    println!("   ⚡ Power: {}W", power);
                }
                
                // Clocks
                if let Some(graphics_clock) = gpu.clocks.graphics {
                    println!("   🕐 Graphics Clock: {} MHz", graphics_clock);
                }
                if let Some(memory_clock) = gpu.clocks.memory {
                    println!("   🕐 Memory Clock: {} MHz", memory_clock);
                }
            }
            
            // GPU processes
            if !readings.processes.is_empty() {
                println!("\n🔄 GPU Processes:");
                println!("=================");
                for process in &readings.processes {
                    println!("   PID {}: {} ({} MB)", 
                        process.pid, 
                        process.process_name, 
                        process.used_memory
                    );
                }
            } else {
                println!("\n🔄 No GPU processes currently running");
            }
            
            // Summary statistics
            println!("\n📈 Summary:");
            println!("===========");
            println!("   Total GPUs: {}", readings.gpu_count);
            println!("   Total Memory: {} MB", readings.get_total_memory());
            println!("   Used Memory: {} MB", readings.get_total_memory_used());
            println!("   Average GPU Utilization: {:.1}%", readings.get_average_gpu_utilization());
            
            if let Some(avg_temp) = readings.get_average_temperature() {
                println!("   Average Temperature: {:.1}°C", avg_temp);
            }
            
        }
        Err(e) => {
            println!("❌ Failed to get GPU info: {}", e);
        }
    }
    
    println!("\n✅ Demo completed successfully!");
    println!("\nTo use the full TUI interface, run:");
    println!("   cargo run -- --help");
    
    Ok(())
}
