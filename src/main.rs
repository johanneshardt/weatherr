mod cli;
#[allow(non_snake_case)]
mod report;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

use clap::StructOpt;

fn main() {
    let cli = cli::Opts::parse();

    let maps_api_key: String = load_maps_api_key(Path::new(".env.secrets"));

    if let Some(desc) = cli.description.as_deref() {
        let pos = match pos_from_string(desc, &maps_api_key) {
            Ok(p) => p,
            Err(e) => panic!("Invalid response: {}", e),
        };

        let res = match smhi_response(pos) {
            Ok(r) => {
                if r.contains("Requested point is out of bounds") {
                    eprintln!("Location not in SMHI's database.");
                    exit(1);
                } else {
                    r
                }
            }
            Err(e) => panic!("Invalid response: {}", e),
        };

        // For debugging purposes
        write_file(&res);
        match report::Report::new(res) {
            Ok(r) => {
                let events = r.get_events();
                for e in &events[0..3] {
                    println!("\n{}", e)
                }
            }
            Err(e) => panic!("Couldn't deserialize: {}", e),
        }
    }

    if let Some(coords) = cli.coordinates.as_deref() {
        println!("Not implemented yet.")
    }
}

#[derive(Debug)]
struct Position {
    lat: f64,
    long: f64,
}
// TODO: More robust JSON parsing, remove expect?
// TODO: Detect wrong API key?
fn pos_from_string(s: &str, key: &str) -> Result<Position, serde_json::Error> {
    use serde_json::Value;
    let res = maps_response(s, key).expect("Maps request failed.");
    let json: Value = serde_json::from_str(&res)?;
    let location: Option<&Value> = json
        .get("results")
        .and_then(|v| v.get(0))
        .and_then(|v| v.get("geometry"))
        .and_then(|v| v.get("location"));

    let lat = location
        .and_then(|v| v.get("lat"))
        .and_then(|v| v.as_f64())
        .expect("Missing location.");

    let long = location
        .and_then(|v| v.get("lng"))
        .and_then(|v| v.as_f64())
        .expect("Missing location.");

    Ok(Position { lat, long })
}

fn maps_response(s: &str, key: &str) -> Result<String, attohttpc::Error> {
    let location = s.replace("", "+");
    let link = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
        location, key
    );
    attohttpc::get(link).send()?.text()
}

fn smhi_response(pos: Position) -> Result<String, attohttpc::Error> {
    // Rounding to four decimals, longer coords generate invalid SMHI response.
    fn round(n: f64) -> f64 {
        (n * 10f64.powi(4)).round() / 10f64.powi(4)
    }

    let pos = Position {
        lat: round(pos.lat),
        long: round(pos.long),
    };

    let link = format!("http://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/{}/lat/{}/data.json",
                        pos.long, pos.lat);
    attohttpc::get(link).send()?.text()
}

// For debugging purposes
fn write_file(f: &str) -> std::io::Result<()> {
    let mut file = File::create("result.json")?;
    file.write_all(f.as_bytes())?;
    Ok(())
}

/// Returns a secret from the file at the specified path p. [^dotenv]
///
/// Secrets in files are specified such that:
/// '''SOME_KEY=value'''
/// This will then return '''value'''.
/// Lines beginning with '#' are ignored.
///
/// [^dotenv]: Inspired by dotenv secret handling
fn load_secret(p: &Path, key: &str) -> String {
    let key = format!("{}{}", key, "=");
    let contents =
        fs::read_to_string(p.as_os_str()).expect(&format!("Couldn't read file: {}", p.display()));
    contents
        .lines()
        .find(|&s| s.contains(&key) && !s.starts_with("#"))
        .expect(&format!("Secret [{}] not found", key));
    contents[contents.find("=").unwrap() + 1..].to_owned() // Should be safe since we find "=" and it is always present
}

fn load_maps_api_key(p: &Path) -> String {
    load_secret(p, "MAPS_API_KEY")
}