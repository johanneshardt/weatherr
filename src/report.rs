use chrono::{prelude::*, Duration};
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

// TODO: Modular output depending on params, maybe separate output from Event struct
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            self.format_time(),
            self.weatherstatus(),
            Event::format_measurement(self.temperature()),
            Event::format_measurement(self.wind_speed()),
            Event::format_measurement(self.wind_direction()),
            Event::format_measurement(self.humidity()),
            Event::format_measurement(self.precipation()),
        )
    }
}

impl Event {
    pub fn format_measurement(m: Measurement) -> String {
        let displacement = 30 - m.name.len(); // TODO make this easier to modify
        format!(
            "{} {}: {:>d$}{}",
            m.symbol,
            m.name,
            m.value,
            m.unit,
            d = displacement
        )
    }

    // TODO fix time offset
    pub fn format_time(&self) -> String {
        let local_time: DateTime<Local> = DateTime::from(self.validTime);
        let delta: Duration = local_time - Local::now();
        match delta.num_hours() {
            0..=24 => format!("{}\n", local_time.format("🕐 %R\nToday")), // TODO: Improve readability
            25..=48 => format!("{}\n", local_time.format("🕐 %R\nTomorrow")),
            _ => format!("{}\n", local_time.format("🕐 %R\n%A, %B %e")),
        }
    }

    pub fn humidity(&self) -> Measurement {
        Measurement {
            symbol: "💧".to_owned(),
            name: "Humidity".to_owned(),
            value: self.value_of("r"),
            unit: "%".to_owned(),
        }
    }

    pub fn precipation(&self) -> Measurement {
        Measurement {
            symbol: "🌧️".to_owned(),
            name: "Precipation".to_owned(),
            value: self.value_of("pmean"), // TODO: Multiple methods for other measurements of precipation
            unit: "mm/h".to_owned(),
        }
    }
    // TODO: better getter for fields in Event
    pub fn temperature(&self) -> Measurement {
        Measurement {
            symbol: "🌡️".to_owned(),
            name: "Temperature".to_owned(),
            value: self.value_of("t"),
            unit: "C".to_owned(),
        }
    }

    pub fn wind_gust_speed(&self) -> Measurement {
        Measurement {
            symbol: "🌪️".to_owned(),
            name: "Gust speed".to_owned(),
            value: self.value_of("gust"),
            unit: "m/s".to_owned(),
        }
    }

    pub fn wind_speed(&self) -> Measurement {
        Measurement {
            symbol: "💨".to_owned(),
            name: "Wind speed".to_owned(),
            value: self.value_of("ws"),
            unit: "m/s".to_owned(),
        }
    }

    // Counted clockwise from "south", in degrees.
    // TODO implement human-readable formatter
    pub fn wind_direction(&self) -> Measurement {
        Measurement {
            symbol: "🧭".to_owned(),
            name: "Wind direction".to_owned(),
            value: self.value_of("wd"),
            unit: "°".to_owned(),
        }
    }

    fn parse(&self, s: &str) -> Option<&Datapoint> {
        self.parameters.iter().find(|x| x.name == s)
    }

    fn value_of(&self, s: &str) -> f64 {
        self.parse(s).expect("Missing parameter").values[0]
    }

    // TODO: handle expect() call
    pub fn weatherstatus(&self) -> WeatherStatus {
        WeatherStatus::from(self.parameters[18].values[0] as usize).expect("Invalid weather code provided.")
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
    RainShowers(Level),
    Thunderstorm,
    SleetShowers(Level),
    SnowShowers(Level),
    Rain(Level),
    Thunder,
    Sleet(Level),
    Snowfall(Level),
}

#[derive(Clone, Copy)]
pub enum Level {
    Light,
    Moderate,
    Heavy,
}

// TODO: Improve namespacing, maybe refactor into module?
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
            WeatherStatus::RainShowers(Level::Light),
            WeatherStatus::RainShowers(Level::Moderate),
            WeatherStatus::RainShowers(Level::Heavy),
            WeatherStatus::Thunderstorm,
            WeatherStatus::SleetShowers(Level::Light),
            WeatherStatus::SleetShowers(Level::Moderate),
            WeatherStatus::SleetShowers(Level::Heavy),
            WeatherStatus::SnowShowers(Level::Light),
            WeatherStatus::SnowShowers(Level::Moderate),
            WeatherStatus::SnowShowers(Level::Heavy),
            WeatherStatus::Rain(Level::Light),
            WeatherStatus::Rain(Level::Moderate),
            WeatherStatus::Rain(Level::Heavy),
            WeatherStatus::Thunder,
            WeatherStatus::Sleet(Level::Light),
            WeatherStatus::Sleet(Level::Moderate),
            WeatherStatus::Sleet(Level::Heavy),
            WeatherStatus::Snowfall(Level::Light),
            WeatherStatus::Snowfall(Level::Moderate),
            WeatherStatus::Snowfall(Level::Heavy),
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
            WeatherStatus::RainShowers(Level::Light) => "Light rain showers",
            WeatherStatus::RainShowers(Level::Moderate) => "Moderate rain showers",
            WeatherStatus::RainShowers(Level::Heavy) => "Heavy rain showers",
            WeatherStatus::Thunderstorm => "Thunderstorm",
            WeatherStatus::SleetShowers(Level::Light) => "Light sleet showers",
            WeatherStatus::SleetShowers(Level::Moderate) => "Moderate sleet showers",
            WeatherStatus::SleetShowers(Level::Heavy) => "Heavy sleet showers",
            WeatherStatus::SnowShowers(Level::Light) => "Light snow showers",
            WeatherStatus::SnowShowers(Level::Moderate) => "Moderate snow showers",
            WeatherStatus::SnowShowers(Level::Heavy) => "Heavy snow showers",
            WeatherStatus::Rain(Level::Light) => "Light rain",
            WeatherStatus::Rain(Level::Moderate) => "Moderate rain",
            WeatherStatus::Rain(Level::Heavy) => "Heavy rain",
            WeatherStatus::Thunder => "Thunder",
            WeatherStatus::Sleet(Level::Light) => "Light sleet",
            WeatherStatus::Sleet(Level::Moderate) => "Moderate sleet",
            WeatherStatus::Sleet(Level::Heavy) => "Heavy sleet",
            WeatherStatus::Snowfall(Level::Light) => "Light snowfall",
            WeatherStatus::Snowfall(Level::Moderate) => "Moderate snowfall",
            WeatherStatus::Snowfall(Level::Heavy) => "Heavy snowfall",
        };
        write!(f, "{}", description)
    }
}
