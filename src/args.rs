use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
#[clap(about = "Tracing / benchmarking long running applications (ie: streaming).", long_about = None)]
pub struct Args {
    /// Application to be run as child process (alternatively provide PID of running app).
    #[clap(value_parser)]
    pub application: Option<String>,

    /// PID of external process.
    #[clap(short, long, value_parser)]
    pub pid: Option<i32>,

    /// Switch off UI - csv style output
    #[clap(short, long, action)]
    pub noui: bool,

    /// Switch off auto-scale - this will use all available CPU/MEM in the graphs.
    #[clap(short, long, action)]
    pub autoscale: bool,

    /// Refresh rate in milliseconds.
    #[clap(short, long)]
    #[clap(default_value_t = 1000)]
    pub refresh: u64,

    /// CSV output file
    #[clap(short, long)]
    pub output: Option<String>,

    /// Custom log level: info, debug, trace
    #[clap(short, long, default_value = "info")]
    pub log: String,

    /// GPU-specific options
    /// Select specific GPU(s) to monitor (comma-separated indices, e.g., "0,1,2")
    #[clap(long, value_parser)]
    pub gpu_indices: Option<String>,

    /// Monitor all available GPUs (default behavior)
    #[clap(long, action)]
    pub all_gpus: bool,

    /// Filter metrics to display (comma-separated: memory,utilization,temperature,power,clocks,processes)
    #[clap(long, value_parser)]
    pub metrics: Option<String>,

    /// Enable GPU process monitoring
    #[clap(long, action)]
    pub gpu_processes: bool,

    /// Set temperature warning threshold in Celsius
    #[clap(long, default_value_t = 80)]
    pub temp_warning: i32,

    /// Set temperature critical threshold in Celsius
    #[clap(long, default_value_t = 90)]
    pub temp_critical: i32,

    /// Set memory usage warning threshold in percentage
    #[clap(long, default_value_t = 80)]
    pub mem_warning: u32,

    /// Set memory usage critical threshold in percentage
    #[clap(long, default_value_t = 95)]
    pub mem_critical: u32,

    /// Set GPU utilization warning threshold in percentage
    #[clap(long, default_value_t = 90)]
    pub util_warning: u32,

    /// Set GPU utilization critical threshold in percentage
    #[clap(long, default_value_t = 95)]
    pub util_critical: u32,

    /// Enable alerts for threshold violations
    #[clap(long, action)]
    pub alerts: bool,

    /// Optional program arguments (ignored with PID option)
    #[arg(last = true)]
    pub args: Vec<String>,
}

impl Args {
    /// Parse GPU indices from the command line argument
    pub fn get_gpu_indices(&self) -> Vec<u32> {
        if let Some(indices_str) = &self.gpu_indices {
            indices_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u32>().ok())
                .collect()
        } else {
            Vec::new() // Empty means all GPUs
        }
    }

    /// Parse metrics filter from the command line argument
    pub fn get_metrics_filter(&self) -> Vec<String> {
        if let Some(metrics_str) = &self.metrics {
            metrics_str
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .collect()
        } else {
            vec![
                "memory".to_string(),
                "utilization".to_string(),
                "temperature".to_string(),
                "power".to_string(),
                "clocks".to_string(),
                "processes".to_string(),
            ]
        }
    }

    /// Check if a specific metric should be displayed
    pub fn should_show_metric(&self, metric: &str) -> bool {
        let filter = self.get_metrics_filter();
        filter.is_empty() || filter.contains(&metric.to_lowercase())
    }

    /// Get the selected GPU indices or all available if none specified
    pub fn get_selected_gpu_indices(&self, available_gpus: u32) -> Vec<u32> {
        let indices = self.get_gpu_indices();
        if indices.is_empty() || self.all_gpus {
            (0..available_gpus).collect()
        } else {
            indices.into_iter().filter(|&idx| idx < available_gpus).collect()
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
