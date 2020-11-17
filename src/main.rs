fn main() {
    println!("test");
    let res = match response() {
        Ok(r) => r,
        Err(e) => panic!("Invalid response: {}", e),
    };

    match report::generate_report(res) {
        Ok(r) => println!("Report: {:#?}", r.geometry),
        Err(e) => panic!("Couldn't deserialize: {}", e),
    }
}

struct Position {
    lat: f64,
    long: f64,
}

pub fn response() -> Result<String, attohttpc::Error> {
    let pos = Position {
        lat: 55.7058,
        long: 13.1932,
    };
    let link = format!("http://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/{}/lat/{}/data.json",
                        pos.long, pos.lat);
    Ok(attohttpc::get(link).send()?.text()?)
}

mod report {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Datapoint {
        name: String,
        levelType: String,
        level: i32,
        unit: String,
        values: Vec<f64>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Event {
        validTime: String,
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
        approvedTime: String,
        referenceTime: String,
        pub geometry: Location,
        timeSeries: Vec<Event>,
    }
    pub fn generate_report(r: String) -> Result<Report, serde_json::Error> {
        serde_json::from_str(&r)
    }
}
