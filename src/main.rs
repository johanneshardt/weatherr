
#[allow(non_snake_case)]
mod report;

fn main() {
    let lund = Position {
        lat: 55.7058,
        long: 13.1932,
    };
    let res = match response(lund) {
        Ok(r) => r,
        Err(e) => panic!("Invalid response: {}", e),
    };

    match report::Report::new(res) {
        Ok(r) => println!("Report: \n {:#?}", r.get_events()[0]),
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
    Ok(attohttpc::get(link).send()?.text()?)
}