use serde::{Deserialize, Serialize};
use chrono::prelude::*;

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

    pub fn info(&self) -> String {
        format!("Report from {}, approved at {}", self.referenceTime, self.approvedTime)
    }
}
