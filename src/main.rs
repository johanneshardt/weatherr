
#[allow(non_snake_case)]
mod report;
use std::fs::File;
use std::io::prelude::*;

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

    match report::Report::new(res) {
        Ok(r) => println!("First event: \n{}", r.get_events()[2]),
        Err(e) => panic!("Couldn't deserialize: {}", e),
    }
}

pub struct Position {
    lat: f64,
    long: f64,
}

pub fn response(pos: Position) -> Result<String, attohttpc::Error> {
    let link = format!("http://opendata-download-metfcst.smhi.se/api/category/pmp3g/version/2/geotype/point/lon/{}/lat/{}/data.json",
                        pos.long, pos.lat);
    attohttpc::get(link).send()?.text()
}

pub fn write_file(f: &str) -> std::io::Result<()> {
    let mut file = File::create("result.json")?;
    file.write_all(f.as_bytes())?;
    Ok(())
}