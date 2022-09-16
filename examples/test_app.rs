use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;


/// This is example testing app that could be processed by any of tracers
/// ```shell
/// 
/// ```
/// Apps runs for 60 secs.
#[tokio::main]
async fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    
    let runs = 60;
    for i in 0..runs {
        println!("this is long process {}  of {}", i, runs);
        thread::sleep(Duration::from_secs(1));
    }

    println!("Exiting...");
}