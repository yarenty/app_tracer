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

    /// Optional program arguments (ignored with PID option)
    #[arg(last = true)]
    pub args: Vec<String>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
