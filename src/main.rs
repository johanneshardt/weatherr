#[allow(non_snake_case)]
mod report;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let lund = Position {
        lat: 55.7058,
        long: 13.1932,
    };
    let res = match response(lund) {
        Ok(r) => r,
        Err(e) => panic!("Invalid response: {}", e),
    };

    write_file(&res);

    let MAPS_API_KEY: String = load_maps_api_key(Path::new(".env.example"));

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

struct Position {
    lat: f64,
    long: f64,
}

fn response(pos: Position) -> Result<String, attohttpc::Error> {
    let link = format!("http://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/{}/lat/{}/data.json",
                        pos.long, pos.lat);
    attohttpc::get(link).send()?.text()
}

fn write_file(f: &str) -> std::io::Result<()> {
    let mut file = File::create("result.json")?;
    file.write_all(f.as_bytes())?;
    Ok(())
}

/// Returns a secret from the file at the specified path p.
///
/// Secrets in files are specified such that:
/// '''SOME_KEY=value'''
/// This will then return '''value'''.
/// Lines beginning with '#' are ignored.
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
