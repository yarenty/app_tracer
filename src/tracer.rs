#[macro_use]
extern crate log;

use crate::args::Args;
use crate::trace::{app::App, cmd::Cmd, event::Event, ui::renderer::render, Record};
use crate::utils::create_file;
use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use csv::Writer;
use itertools::Itertools;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{
    fs::File,
    io,
    process::{Command, Stdio},
    sync::mpsc,
    thread, time,
    time::Duration,
};
use sysinfo::{Pid, ProcessExt, System, SystemExt};
use termion::{event, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen};
use tokio::{signal, spawn};
use utils::{check_in_current_dir, get_current_working_dir, setup_logger};

mod args;
mod error;
mod trace;
mod utils;

/// Main function. Because every program needs one.
/// "The journey of a thousand miles begins with a single step." - Lao Tzu, who probably never wrote multithreaded applications.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse(); // Oh, we have arguments? How fancy!
    setup_logger(true, Some(&args.log)); // because logging is essential.

    debug!("Start"); // Of course, we're starting.

    let mut kill = false; // Kill it with fire! Or, you know, a boolean.
    let id: i32; // Because every process needs an ID. Or an identity crisis.

    if let Some(app) = args.application {
        // What application?
        let with_params = app.split(' ').collect_vec(); // Let's split the app name from params
        let apt = app.as_str(); // And we need it as str? why?
        let (app, params) = if let Some((a, p)) = with_params.split_first() {
            // Split first
            (a, p)
        } else {
            (&apt, [""].as_slice()) // nothing to split
        };

        let mut p = args.args.to_vec(); // get arguments
        for d in params {
            p.push(String::from(*d)); // add additional arguments if needed.
        }

        let (path, app) = check_in_current_dir(app)?; // Let's check app again
        info!(
            "Application to be monitored is: {}, in dir {} , with params: {:?}",
            app,
            path,
            p // Yeah, we have it all!
        );

        let output_file = File::create(format!("{}.out", app))?; // Create output file
        let error_file = File::create(format!("{}.err", app))?; // and error file

        let cmd = Command::new(&path) // create command
            .current_dir(get_current_working_dir()) // set current dir
            .args(p) // and params
            .stdin(Stdio::null()) // no input
            .stdout(Stdio::from(output_file)) // save output to file
            .stderr(Stdio::from(error_file)) // save error to file
            .spawn() // spawn it !
            .expect("Failed to run "); // if it is failed

        kill = true; // we will kill it later
        id = cmd.id() as i32; // get the id
    } else if let Some(pid) = &args.pid {
        // What about PID ?
        info!("Application to be monitored is: [PID] {:?}", pid); // just log
        id = *pid; // save pid
    } else {
        return Err(eyre!("Not sure what supposed to trace. Please provide application path or PID. [Use -h for help]".to_string()));
        // well...
    }

    let refresh_millis = args.refresh;
    info!("Refresh rate: {} ms.", refresh_millis); // How fast do you want it to be?

    let mut writer: Option<Writer<File>> = args
        .output
        .as_ref()
        .map(|path| csv::Writer::from_writer(create_file(path).inner)); // CSV time
    match writer {
        Some(_) => info!(
            "Output readings persisted into \"{}\".",
            args.output.unwrap()
        ), // it will be saved
        None => info!("No output persistence."), // it will be lost
    }

    let pid: Pid = Pid::from(id);
    info!("Starting with PID::{}", pid); // start with pid

    if args.noui {
        let mut system = System::new_all();

        info!("Running in TXT mode.");
        // TODO add Ctrl+C - kill child process !!
        loop {
            thread::sleep(Duration::from_millis(refresh_millis));
            system.refresh_process(pid);
            let process = system.process(pid).unwrap();
            let t = format!("{}", chrono::Utc::now().time());
            let c = format!("{}", process.cpu_usage());
            let m = format!("{}", process.memory() / 1024);
            info!("CPU: {} [%],  memory: {} [kB]", c, m,);
            if let Some(wtr) = &mut writer {
                let r = Record::new(&t, &c, &m);
                wtr.serialize(r).expect("Error serializing outputs to csv");
                wtr.flush()?;
            }
        }
    } else {
        info!("Running in TUI mode.");

        //Program
        let mut app = App::new(5000, 50, pid, !args.autoscale, refresh_millis)?;
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();
        let ticker_tx = tx.clone();

        debug!("Channels registered");
        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                let evt = c.unwrap();
                input_tx.send(Event::Input(evt)).unwrap();
                if evt == event::Key::Char('q') {
                    break;
                }
            }
        });

        debug!("Ticker starting");
        thread::spawn(move || loop {
            ticker_tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(refresh_millis));
        });

        let stdout = io::stdout().into_raw_mode()?;
        let stdout = stdout.into_alternate_screen()?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        debug!("Cleaning and into terminal mode");
        terminal.clear()?;
        terminal.hide_cursor()?;

        // Setup Ctrl+C handler
        let ctrl_c_tx = tx.clone();

        spawn(async move {
            if let Ok(()) = signal::ctrl_c().await {
                ctrl_c_tx.send(Event::Quit).unwrap_or_default();
            }
        });

        debug!("Into loop");
        loop {
            let evt = rx.recv().unwrap();
            {
                match evt {
                    Event::Input(key) => {
                        if key == event::Key::Char('q') {
                            break;
                        }
                        if let Some(cmd) = app.input_handler(key) {
                            match cmd {
                                Cmd::Quit => break,
                            }
                        }
                    }
                    Event::Tick => {
                        app.update()?;
                        if let Some(wtr) = &mut writer {
                            let t = format!("{}", chrono::Utc::now().time());
                            let c = format!("{}", app.datastreams.readings.get_cpu());
                            let m = format!("{}", app.datastreams.readings.get_mem());
                            let r = Record::new(&t, &c, &m);
                            wtr.serialize(r).expect("Error serializing outputs to csv");
                            wtr.flush()?;
                        }
                    }
                    Event::Quit => {
                        break;
                    }
                }
            }

            render(&mut terminal, &app)?;
        }

        debug!("Back with cursor and original terminal");
        terminal.clear()?;
        terminal.show_cursor()?;

        // TODO: Kill the monitored process if it's still running, need to rethink whole no UI stuff as well
        // if let Ok(mut process) = sysinfo::System::new_all().processes().get(&pid.as_u32()) {
        //     process.kill();
        // }
    }
    if let Some(wtr) = &mut writer {
        wtr.flush()?;
    }
    // in case of exit from application that was not terminated by user
    if kill {
        let _ = Command::new("kill")
            .arg("-9")
            .arg(format!("{}", id))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();
    }

    Ok(())
}
