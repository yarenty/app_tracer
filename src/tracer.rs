#[macro_use]
extern crate log;

use clap::Parser;
use std::fs::File;
use std::io;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time;
use std::time::Duration;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

use termion::{
    event,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};

use tui::backend::CrosstermBackend;
use tui::Terminal;

use color_eyre::eyre::{eyre, Result};
use csv::Writer;
use itertools::Itertools;

mod args;
mod error;
mod trace;
mod utils;

use crate::trace::{app::App, cmd::Cmd, event::Event, ui::renderer::render, Record};

use utils::{check_in_current_dir, get_current_working_dir, setup_logger};

use crate::args::Args;
use crate::utils::create_file;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    setup_logger(true, Some(&args.log));

    debug!("Start");

    let mut kill = false;
    let id: i32;
    if let Some(app) = args.application {
        let with_params = app.split(' ').collect_vec();
        let apt = app.as_str();
        let (app, params) = if let Some((a, p)) = with_params.split_first() {
            (a, p)
        } else {
            (&apt, [""].as_slice())
        };

        let mut p = args.args.to_vec();
        for d in params {
            p.push(String::from(d.to_string()));
        }

        let (path, app) = check_in_current_dir(app)?;
        info!("Application to be monitored is: {}, in dir {} , with params: {:?}", app, path, p);

        let output_file = File::create(format!("{}.out", app))?;
        let error_file = File::create(format!("{}.err", app))?;
        
        let cmd = Command::new(&path)
            .current_dir(get_current_working_dir())
            .args(p)
            .stdin(Stdio::null())
            .stdout(Stdio::from(output_file))
            .stderr(Stdio::from(error_file))
            .spawn()
            .expect("Failed to run ");

        kill = true;
        id = cmd.id() as i32;
    } else if let Some(pid) = &args.pid {
        info!("Application to be monitored is: [PID] {:?}", pid);
        id = *pid;
    } else {
        return Err(eyre!("Not sure what supposed to trace. Please provide application path or PID. [Use -h for help]".to_string()));
    }

    let refresh_millis = args.refresh;
    info!("Refresh rate: {} ms.", refresh_millis);

    let mut writer: Option<Writer<File>> = args
        .output
        .as_ref()
        .map(|path| csv::Writer::from_writer(create_file(path).inner));
    match writer {
        Some(_) => info!(
            "Output readings persisted into \"{}\".",
            args.output.unwrap()
        ),
        None => info!("No output persistence."),
    }

    let pid: Pid = Pid::from(id);
    info!("Starting with PID::{}", pid);

    if args.noui {
        let mut system = System::new_all();

        info!("Running in TXT mode.");
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
        thread::spawn(move || {
            let tx = tx;
            loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(time::Duration::from_millis(refresh_millis));
            }
        });

        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        debug!("Cleaning and into terminal mode");
        terminal.clear()?;
        terminal.hide_cursor()?;

        let clk_split = 0;

        debug!("Into loop");
        loop {
            let evt = rx.recv().unwrap();
            {
                match evt {
                    Event::Input(input) => {
                        if let Some(command) = app.input_handler(input) {
                            match command {
                                Cmd::Quit => {
                                    break;
                                } //_ => (),
                            }
                        }
                    }
                    Event::Tick => {
                        if clk_split % 2 == 0 {
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
                    }
                }
            }

            render(&mut terminal, &app)?;
        }

        debug!("Back with cursor and original terminal");
        terminal.show_cursor().unwrap();
        terminal.clear().unwrap();
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
