#[macro_use]
extern crate log;

use clap::Parser;
use std::io;
use std::process::{exit, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time;
use sysinfo::Pid;

use termion::{
    event,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};

use tui::backend::TermionBackend;
use tui::Terminal;

mod args;
mod error;
mod trace;
mod utils;

use crate::trace::{app::App, cmd::Cmd, event::Event, ui::renderer::render};
use utils::{check_in_current_dir, get_current_working_dir, setup_logger};

use crate::args::Args;
use crate::error::{Result, TraceError};

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

    debug!("Start");
    // let app = "/opt/workspace/app_banchmark/target/debug/examples/test_app";

    let id: i32;
    if let Some(app) = &args.application {
        info!("Application to be benchmark is: {}", app);
        info!("Refresh rate: {}", &args.refresh);

        let (path, app) = check_in_current_dir(app)?;
        info!("App:: {}  in dir {}", app, path);

        let cmd = Command::new(&path)
            .current_dir(get_current_working_dir())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            // .stderr((Stdio::piped())
            .spawn()
            .expect("Failed to run ");

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

    // let mut wtr = csv::Writer::from_writer(create_file(&args.output).inner);

    let pid: Pid = Pid::from(id);
    info!("Starting with PID::{}", pid);

    //Program
    let mut app = App::new(5000, 50, pid)?;
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

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
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let clk_split = 0;

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
                    }
                }
            }
        }

        render(&mut terminal, &app)?;
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
    Ok(())
}
