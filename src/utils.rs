use crate::error::{Result, TraceError};
use chrono::prelude::*;
use env_logger::fmt::Formatter;
use env_logger::{Builder, WriteStyle};
use log::{Level, LevelFilter, Record};
use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, thread};
use termion::color::{self, Fg};

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

        // Define color styles using termion
        let (level_color, level_style) = match record.level() {
            Level::Error => (format!("{}", Fg(color::LightRed)), "ERROR".to_string()),
            Level::Warn => (format!("{}", Fg(color::Yellow)), "WARN".to_string()),
            Level::Info => (format!("{}", Fg(color::LightGreen)), "INFO".to_string()),
            Level::Debug => (format!("{}", Fg(color::LightBlue)), "DEBUG".to_string()),
            Level::Trace => (format!("{}", Fg(color::Magenta)), "TRACE".to_string()),
        };

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();

        writeln!(
            formatter,
            "{}{}{} {}{}{} {}{}{} [{}] - {}",
            Fg(color::LightCyan), time_str, Fg(color::Reset),
            Fg(color::LightMagenta), thread_name, Fg(color::Reset),
            level_color, level_style, Fg(color::Reset),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter_level(LevelFilter::Info)
        .write_style(WriteStyle::Always);

    if let Some(conf) = rust_log {
        builder.parse_env(conf);
    }

    builder.init();
}
