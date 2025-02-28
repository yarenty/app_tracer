use sysinfo::{Pid, System as SysInfoSystem};
use sysinfo::{ProcessExt, SystemExt};

use crate::trace::datastreams::{
    CPUMonitor, MemoryMonitor, ProcessMonitor, Readings, SysDataStream,
};

use crate::error::Result;

/// AppDataStreams - because calling it "StuffThatHappensInTheApp" was too obvious.
/// "I came, I saw, I conquered." - Julius Caesar, probably not talking about data streams.
pub struct AppDataStreams {
    pub pid: Pid, // The process ID. Because everything needs an ID. Even your toaster.
    pub cpu_info: CPUMonitor, // We're monitoring CPU, because it's not like we have anything better to do.
    pub mem_info: MemoryMonitor, // We're monitoring memory, to understand the mess.
    pub process_info: ProcessMonitor, // The most important thing is process info. Or is it not?
    pub sys_info_src: SysInfoSystem, //  SysInfoSystem, because we needed a source of truth. Or just more data.
    pub readings: Readings, // The readings. Because we're reading. Or something.
}

impl AppDataStreams {
    /// Creates a new AppDataStreams. Hopefully, it streams data.
    /// "If a tree falls in a forest and no one is around to hear it, does it make a sound?" - George Berkeley, probably about debug logs.
    pub fn new(history_len: usize, interpolation_len: u16, pid: Pid) -> Result<Self> {
        let mut sys = SysInfoSystem::new(); // Creating a system object. Because why not?
        let readings = Readings::new(&mut sys, pid); // Reading some data. Can it read our minds?
        Ok(Self {
            pid,
            cpu_info: SysDataStream::new(history_len, interpolation_len), // Another stream. It's like a river of data.
            mem_info: SysDataStream::new(history_len, interpolation_len), // Another stream. We are full of streams!
            process_info: SysDataStream::new(history_len, interpolation_len), // Another stream. We are swimming in streams!
            sys_info_src: sys, // Here's our source. Unfiltered. Probably.
            readings, // And here are the readings. What do they mean?
        })
    }

    /// Updates all our streams. Because data doesn't just update itself.
    /// "The only true wisdom is in knowing you know nothing." - Socrates, probably while updating a data stream.
    pub fn update(&mut self) -> Result<()> {
        self.sys_info_src.refresh_process(self.pid); // Refreshing. Because the data goes bad, or something.

        let p = self.sys_info_src.process(self.pid).unwrap(); // We're unwrapping. Bravely.

        // We don't know what we're doing.
        let mut cpu = p.cpu_usage(); // Are we sure it's CPU?
        let mut mem = p.memory(); // And memory?

        //  subs PIDs
        let ps = self.sys_info_src.processes(); // Get all processes

        // Here we are iterating over processes, because why not?
        let mut subs: Vec<Pid> = vec![];
        for p in ps
            .iter()
            .map(|(_k, v)| v)
            .filter(|p| p.parent() == Some(self.pid)) // Filtering by parent - sounds familiar
        {
            subs.push(p.pid()); // Adding PIDs
            cpu += p.cpu_usage(); // because more is better
            mem += p.memory(); // and same here
        }

        // TODO: 2 levels of parents PIDs at the moment - need to fix it / check if needed more
        // Because two levels is never enough. Or is it?
        for p in ps
            .iter()
            .map(|(_k, v)| v)
            .filter(|p| subs.contains(&p.parent().unwrap_or_else(|| Pid::from(0)))) // Checking parent PIDs - what can go wrong?
        {
            cpu += p.cpu_usage(); // More cpu - more better!
            mem += p.memory(); // Same with memory - better!
        }

        self.readings.refresh(cpu, mem); // Reading are refreshed!
        self.cpu_info.poll(&self.readings); // Polling. Because we can't just ask nicely for the data.
        self.mem_info.poll(&self.readings); // Poll poll poll
        self.process_info.poll(&self.readings); // And here we are polling again.
        Ok(()) // Yeah, everything is Ok. Probably.
    }
}
