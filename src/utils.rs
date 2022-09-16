use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, thread};
use chrono::prelude::*;
use env_logger::fmt::{Color, Formatter};
use env_logger::{Builder, WriteStyle};
use log::{Level, LevelFilter, Record};
use crate::error::{Result, TraceError};


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

/// Creates output directory for storing csv/graphs outputs.
pub fn create_output_file(app: &str, filename: &str) -> tagger::Adaptor<std::fs::File> {
    std::fs::create_dir_all(format!("bench_{}", app)).expect("Cannot create output directory");
    let file = std::fs::File::create(format!(
        "bench_{}{}{}",
        app,
        std::path::MAIN_SEPARATOR,
        filename
    ))
        .unwrap_or_else(|_| panic!("Cannot create output file: {}", filename));
    tagger::upgrade_write(file)
}

/// Creates output file for tracing.
pub fn create_file(filename: &str) -> tagger::Adaptor<std::fs::File> {
    let file = std::fs::File::create(format!(
        "{}",
        filename
    ))
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

        let mut thread_style = formatter.style();
        let mut level_style = formatter.style();

        match record.level() {
            Level::Error => level_style.set_color(Color::Red).set_bold(true),
            Level::Warn => level_style.set_color(Color::Red),
            Level::Info => level_style.set_color(Color::Green).set_intense(true),
            Level::Debug => level_style.set_color(Color::Blue),
            Level::Trace => level_style.set_color(Color::Magenta),
        };
        thread_style.set_color(Color::Magenta).set_intense(true);

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        writeln!(
            formatter,
            "{} {}{} - {} - {}",
            time_str,
            thread_style.value(thread_name),
            level_style.value(record.level()),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info);
    builder.write_style(WriteStyle::Always);

    rust_log.map(|conf| builder.parse_filters(conf));

    builder.init();
}
