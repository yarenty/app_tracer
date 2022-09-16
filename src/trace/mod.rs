use serde_derive::{Deserialize, Serialize};

pub mod app;
pub mod cmd;
pub mod event;
pub mod ui;

mod app_data_streams;
mod datastreams;

/// CSV output record
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Record<'a> {
    pub time: &'a str,
    pub cpu: &'a str,
    pub mem: &'a str,
}

impl Record<'static> {
    pub fn new<'a>(time: &'a str, cpu: &'a str, mem: &'a str) -> Record<'a> {
        Record { time, cpu, mem }
    }
}
