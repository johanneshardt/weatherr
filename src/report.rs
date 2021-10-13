use chrono::prelude::*;
use core::fmt;
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
    // type is a reserved keyword, but also the name of the field in the json response
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
        write!(
            f,
            "{}\n{}\n{}\n{}",
            self.format_time(),
            Event::format_measurement(self.temperature()),
            Event::format_measurement(self.wind_speed()),
            self.weatherstatus()
        )
    }
}

impl Event {
    pub fn format_measurement(m: Measurement) -> String {
        format!("{} {}: {}{}", m.symbol, m.name, m.value, m.unit)
    }

    pub fn format_time(&self) -> String {
        format!("🕐 Time: {}", self.validTime)
    }
    // TODO: better getter for fields in Event
    pub fn temperature(&self) -> Measurement {
        Measurement {
            symbol: String::from("🌡️"),
            name: String::from("Temperature"),
            value: self.value_of("t"),
            unit: String::from("C"),
        }
    }

    pub fn wind_speed(&self) -> Measurement {
        Measurement {
            symbol: String::from("💨"),
            name: String::from("Wind speed"),
            value: self.value_of("ws"),
            unit: String::from("m/s"),
        }
    }

    fn parse(&self, s: &str) -> Option<&Datapoint> {
        self.parameters.iter().find(|x| x.name == s)
    }

    fn value_of(&self, s: &str) -> f64 {
        self.parse(s).expect("Missing parameter").values[0]
    }

    // TODO: handle unwrap() call
    pub fn weatherstatus(&self) -> WeatherStatus {
        WeatherStatus::from(self.parameters[18].values[0] as usize).unwrap()
    }
}

pub struct Measurement {
    symbol: String,
    name: String,
    value: f64,
    unit: String,
}

// TODO: Implement level enum
#[derive(Clone, Copy)]
pub enum WeatherStatus {
    Clear,
    NearlyClear,
    VariableCloudiness,
    HalfClear,
    Cloudy,
    Overcast,
    Fog,
    LightRainShowers,
    ModerateRainShowers,
    HeavyRainShowers,
    Thunderstorm,
    LightSleetShowers,
    ModerateSleetShowers,
    HeavySleetShowers,
    LightSnowShowers,
    ModerateSnowShowers,
    HeavySnowShowers,
    LightRain,
    ModerateRain,
    HeavyRain,
    Thunder,
    LightSleet,
    ModerateSleet,
    HeavySleet,
    LightSnowfall,
    ModerateSnowfall,
    HeavySnowfall,
}

//TODO: better implementation of WEATHERVARIANTS
impl WeatherStatus {
    pub fn WEATHERVARIANTS() -> Vec<WeatherStatus> {
        vec![
            WeatherStatus::Clear,
            WeatherStatus::NearlyClear,
            WeatherStatus::VariableCloudiness,
            WeatherStatus::HalfClear,
            WeatherStatus::Cloudy,
            WeatherStatus::Overcast,
            WeatherStatus::Fog,
            WeatherStatus::LightRainShowers,
            WeatherStatus::ModerateRainShowers,
            WeatherStatus::HeavyRainShowers,
            WeatherStatus::Thunderstorm,
            WeatherStatus::LightSleetShowers,
            WeatherStatus::ModerateSleetShowers,
            WeatherStatus::HeavySleetShowers,
            WeatherStatus::LightSnowShowers,
            WeatherStatus::ModerateSnowShowers,
            WeatherStatus::HeavySnowShowers,
            WeatherStatus::LightRain,
            WeatherStatus::ModerateRain,
            WeatherStatus::HeavyRain,
            WeatherStatus::Thunder,
            WeatherStatus::LightSleet,
            WeatherStatus::ModerateSleet,
            WeatherStatus::HeavySleet,
            WeatherStatus::LightSnowfall,
            WeatherStatus::ModerateSnowfall,
            WeatherStatus::HeavySnowfall,
        ]
    }
    pub fn from(code: usize) -> Result<WeatherStatus, &'static str> {
        match code {
            1..=27 => Ok(WeatherStatus::WEATHERVARIANTS()[code - 1]),
            _ => Err("Invalid weather symbol code"),
        }
    }
}

impl fmt::Display for WeatherStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            WeatherStatus::Clear => "Clear sky",
            WeatherStatus::NearlyClear => "Nearly clear sky",
            WeatherStatus::VariableCloudiness => "Variable cloudiness",
            WeatherStatus::HalfClear => "Halfclear sky",
            WeatherStatus::Cloudy => "Cloudy sky",
            WeatherStatus::Overcast => "Overcast sky",
            WeatherStatus::Fog => "Fog",
            WeatherStatus::LightRainShowers => "Light rain showers",
            WeatherStatus::ModerateRainShowers => "Moderate rain showers",
            WeatherStatus::HeavyRainShowers => "Heavy rain showers",
            WeatherStatus::Thunderstorm => "Thunderstorm",
            WeatherStatus::LightSleetShowers => "Light sleet showers",
            WeatherStatus::ModerateSleetShowers => "Moderate sleet showers",
            WeatherStatus::HeavySleetShowers => "Heavy sleet showers",
            WeatherStatus::LightSnowShowers => "Light snow showers",
            WeatherStatus::ModerateSnowShowers => "Moderate snow showers",
            WeatherStatus::HeavySnowShowers => "Heavy snow showers",
            WeatherStatus::LightRain => "Light rain",
            WeatherStatus::ModerateRain => "Moderate rain",
            WeatherStatus::HeavyRain => "Heavy rain",
            WeatherStatus::Thunder => "Thunder",
            WeatherStatus::LightSleet => "Light sleet",
            WeatherStatus::ModerateSleet => "Moderate sleet",
            WeatherStatus::HeavySleet => "Heavy sleet",
            WeatherStatus::LightSnowfall => "Light snowfall",
            WeatherStatus::ModerateSnowfall => "Moderate snowfall",
            WeatherStatus::HeavySnowfall => "Heavy snowfall",
        };
        write!(f, "{}", description)
    }
}
