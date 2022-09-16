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

    /// Refresh rate in miliseconds.
    #[clap(short, long)]
    #[clap(default_value_t = 1000)]
    pub refresh: u64,

    /// Name of output CSV file with all readings - for future investigations.
    #[clap(short, long)]
    #[clap(default_value = "output.csv")]
    pub output: String,

    ///Set custom log level: info, debug, trace
    #[clap(short, long, default_value = "info")]
    pub log: String,
}
