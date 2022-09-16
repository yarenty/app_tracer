mod trace;
mod utils;
mod error;
mod args;

use chrono::prelude::*;
use std::io::Read;
use std::process::{Command, exit, Stdio};
use std::thread;
use std::time::Duration;
use log::info;
use sysinfo::{Pid, PidExt, Process, System, ProcessExt, SystemExt, ProcessRefreshKind};
use clap::Parser;

use error::{Result, TraceError};
use utils::setup_logger;
use crate::trace::Record;
use crate::utils::{create_file, create_output_file, get_current_working_dir, check_in_current_dir};
use crate::args::Args;


#[tokio::main]
async fn main() -> Result<()> {
    exit(match _main() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn _main() -> Result<()> {
    let args = Args::parse();
    setup_logger(true, Some(&args.log));

    let id: i32;
    if let Some(app) = &args.application {
        info!("Application to be benchmark is: {}", app);
        info!("Refresh rate: {}", &args.refresh);

        let (path, app) = check_in_current_dir(app)?;
        info!("App:: {}  in dir {}",app,path);

        let cmd = Command::new(&path)
            .current_dir(get_current_working_dir())
            // .stdin(Stdio::null())
            // .stdout(Stdio::piped())
            .spawn().expect("Failed to run ");

        id = cmd.id() as i32;
        info!("CMD::{:?}", cmd);
    } else if let Some(pid) = &args.pid {
        info!("Application by PID to be benchmark is: {:?}", pid);
        id = *pid;
    } else {
        return Err(TraceError::Unknown("Not sure what supposed to trace. Please provide application to run on PID. [Use -h for help]".to_string()));
    }

    let refresh_millis = args.refresh;
    info!("Refresh rate: {}", refresh_millis);

    let mut wtr = csv::Writer::from_writer(create_file(&args.output).inner);

    let pid: Pid = Pid::from(id);
    info!("Starting with PID::{}", pid);
    let mut s = System::new_all();
    let mut process = s.process(pid).unwrap();


    loop {
        let mut r = Record::default();
        thread::sleep(Duration::from_millis(refresh_millis));
        s.refresh_process(pid);
        let process = s.process(pid).unwrap();

        let t = format!("{}", process.run_time());
        let t = format!("{}", Utc::now().time());
        let c = format!("{}", process.cpu_usage());
        let m = format!("{}", process.memory());
        info!("CPU: {}, MEM: {}",  c, m, );
        let mut r = Record::new(&t, &m, &c);
        wtr.serialize(r).expect("Error serializing outputs to csv");
        match wtr.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(TraceError::IoError(format!(
                "Could not create output csv: {:?}",
                e
            ))),
        };
    }
}
