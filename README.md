# weatherr

A command line interface for obtaining weather reports from the Swedish meteorological institute (SMHI). 

## Description

This project uses the [SMHI open data meteorological forecasst API](https://opendata.smhi.se/apidocs/metfcst/index.html) 
for generating accurate weather reports up to 10 days in advance. Note that this is limited to geographical points located within, or close to Sweden. Geocoding is implemented using the Google Maps geocoding API. That is, the CLI accepts a string describing a location such as "Stockholm, Sweden" as well as coordinates. 

## Getting Started

### Dependencies

* A recent version of [Rust](https://www.rust-lang.org/tools/install) (1.56.0 or later). 
* For enabling geocoding, a Google Cloud Platform project connected to a billing account.

### Installing

* Clone this repo and run ```cargo install --path .``` inside the directory.
* Setup a project thorugh the [Google Cloud Console](https://developers.google.com/maps/gmp-get-started)  and enable billing.
* Enable the *geocoding API* for the project and copy the api key ([instructions here](https://developers.google.com/maps/gmp-get-started)).
* Create a file named ```.env.secrets``` with a single line: ```MAPS_API_KEY=your_api_key_here```.

TODO

### Executing program

* How to run the program
* Step-by-step bullets
```

TODO

code blocks for commands
```

## Help

Any advise for common problems or issues.
```
command to run if program contains helper info

```

TODO

## Authors

[Me (Johannes Hardt](https://github.com/johanneshardt)

## Version History

This project uses SEMVER for versioning.

TODO

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

* [Minimal README template](https://gist.github.com/DomPizzie/7a5ff55ffa9081f2de27c315f5018afc)
