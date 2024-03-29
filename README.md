# weatherr 🌤️

A command line interface for obtaining weather reports from the Swedish meteorological institute (SMHI). 

## Description

This project uses the [SMHI open data meteorological forecast API](https://opendata.smhi.se/apidocs/metfcst/index.html) 
for generating accurate weather reports up to 10 days in advance. Note that this is limited to geographical points located within, or close to Sweden. Geocoding is implemented using the Google Maps geocoding API. That is, the CLI accepts a string describing a location such as "Stockholm, Sweden" as well as coordinates. 

### Billing

Since the Google Maps api used is paid, you may be charged for requests. This shouldn't be a problem for personal use, 
as you are provided with [$200 worth of credits a month](https://developers.google.com/maps/documentation/geocoding/usage-and-billing) for free.

## Getting Started

### Dependencies

* A recent version of [Rust](https://www.rust-lang.org/tools/install) (1.56.0 or later). 
* For enabling geocoding, a Google Cloud Platform project connected to a billing account.

### Installation

* Clone this repo and run ```cargo install --path .``` inside the directory.
* Setup a project thorugh the [Google Cloud Console](https://developers.google.com/maps/gmp-get-started)  and enable billing.
* Enable the *geocoding API* for the project and copy the api key ([instructions here](https://developers.google.com/maps/gmp-get-started)).
* Create a file named ```.env.secrets``` with a single line: ```MAPS_API_KEY=your_api_key_here```.

### Executing program

TODO

## Help

TODO

## Authors

[Me (Johannes Hardt)](https://github.com/johanneshardt)

## Version History

Features targeted for 1.0.0 release:
- Generate 1 or 3 day reports
- Accept either coordinates or a physical location as argument
- Some error handling for missing locations and invalid requests
- Completed simple documentation here

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

* [attohttpc](https://github.com/sbstp/attohttpc)
* [chrono](https://github.com/chronotope/chrono)
* [clap](https://github.com/clap-rs/clap)
* [Minimal README template](https://gist.github.com/DomPizzie/7a5ff55ffa9081f2de27c315f5018afc)
* [serde](https://github.com/serde-rs/serde)
* [serde JSON](https://github.com/serde-rs/json)
