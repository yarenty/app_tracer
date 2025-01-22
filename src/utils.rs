use crate::error::{Result, TraceError};
use chrono::prelude::*;
use env_logger::fmt::Formatter;
use env_logger::{Builder, WriteStyle};
use log::{Level, LevelFilter, Record};
use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, thread};
use termion::color::{Fg, Reset, Rgb};
use termion::style::{Bold, Reset as StyleReset};

/// Current output directory
pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

/// Checking if application is in current dir or is the full path.
/// Returns full paths and short name of app.
/// Error otherwise.
pub fn check_in_current_dir(app: &str) -> Result<(String, String)> {
    let (full, short) = if app.contains(std::path::MAIN_SEPARATOR) {
        (
            app.to_string(),
            app.split(std::path::MAIN_SEPARATOR)
                .last()
                .unwrap()
                .to_string(),
        )
    } else {
        (
            format!(
                "{}{}{}",
                get_current_working_dir(),
                std::path::MAIN_SEPARATOR,
                app
            ),
            app.to_string(),
        )
    };

    let checker = if cfg!(target_os = "windows") {
        "dir"
    } else {
        "ls"
    };

    let cmd = Command::new(checker)
        .arg(&full)
        .current_dir(get_current_working_dir())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output();

    match cmd {
        Ok(out) => {
            if out.status.code() == Some(0) {
                Ok((full, short))
            } else {
                Err(TraceError::AppNotFound(format!(
                    "Could not find application: {}.",
                    short
                )))
            }
        }
        Err(e) => Err(TraceError::Unknown(format!(
            "Wrong system utils - are you on windows? {:?}",
            e
        ))),
    }
}

/// Creates output file for tracing.
pub fn create_file(filename: &str) -> tagger::Adaptor<std::fs::File> {
    let file = std::fs::File::create(filename)
        .unwrap_or_else(|_| panic!("Cannot create output file: {}", filename));
    tagger::upgrade_write(file)
}

pub fn setup_logger(log_thread: bool, rust_log: Option<&str>) {
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        let thread_name = if log_thread {
            format!("(t: {}) ", thread::current().name().unwrap_or("unknown"))
        } else {
            "".to_string()
        };

        let level_style = match record.level() {
            Level::Error => format!("{}{}", Fg(Rgb(255,0,0)), Bold),
            Level::Warn => format!("{}", Fg(Rgb(255,0,0))),
            Level::Info => format!("{}", Fg(Rgb(0,255,0))),
            Level::Debug => format!("{}", Fg(Rgb(0,0,255))),
            Level::Trace => format!("{}", Fg(Rgb(255,0,255))),
        };

        let thread_style = format!("{}", Fg(Rgb(255,0,255)));
        let reset = format!("{}{}", Fg(Reset), StyleReset);

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        
        writeln!(
            formatter,
            "{} {}{}{} {}{}{} - {} - {}",
            time_str,
            thread_style, thread_name, reset,
            level_style, record.level(), reset,
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    
    // Set default filter level
    if let Some(conf) = rust_log {
        builder.parse_filters(conf);
    } else {
        builder.filter_level(LevelFilter::Info);
    }

    // Configure formatting
    builder.format(output_format)
           .write_style(WriteStyle::Always);

    // Initialize the logger
    builder.init();
}
