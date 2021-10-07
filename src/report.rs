use core::fmt;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Datapoint {
    name: String,
    levelType: String,
    level: i32,
    unit: String,
    values: Vec<f64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    validTime: DateTime<Utc>,
    parameters: Vec<Datapoint>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "type")]
    dtype: String,
    coordinates: Vec<Vec<f64>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    approvedTime: DateTime<Utc>,
    referenceTime: DateTime<Utc>,
    pub geometry: Location,
    timeSeries: Vec<Event>,
}

impl Report {
    pub fn new(r: String) -> Result<Report, serde_json::Error> {
        serde_json::from_str(&r)
    }

    pub fn get_events(&self) -> Vec<Event> {
        self.timeSeries.clone()
    }

    pub fn timeinfo(&self) -> String {
        format!(
            "Report from {}, approved at {}",
            self.referenceTime, self.approvedTime
        )
    }

    pub fn get_event(&self, index: usize) -> Result<Event, &str> {
        match self.timeSeries.get(index) {
            Some(event) => Ok(event.clone()),
            None => Err("Event index out of bounds."),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.format_time(), Event::format_measurement(self.temperature()))
    }
}

impl Event {
    pub fn format_measurement(m: Measurement) -> String {
        format!("{} {}: {}{}", m.symbol, m.name, m.value, m.unit)
    }

    pub fn format_time(&self) -> String {
        format!("🕐 time: {}", self.validTime)
    }
    // TODO: better getter for fields in Event
    pub fn temperature(&self) -> Measurement {
        Measurement {
            symbol: String::from("🌡️"),
            name: String::from("Temperature"),
            value: self.parameters[11].values[0],
            unit: String::from("C")
        }
    }
}

pub struct Measurement {
    symbol: String,
    name: String,
    value: f64,
    unit: String
}
