use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(
    about = "A command-line app for getting weather reports from SMHI",
    version = "0.3",
    author = "Johannes H."
)]
#[clap(group(
        ArgGroup::new("location")
            .required(true)
            .args(&["description", "coordinates"]),
))]
pub struct Opts {
    /// Specify location by text
    #[clap(short, long)]
    pub description: Option<String>,

    /// Specify location by coordinates
    #[clap(short, long)] // TODO specify coordinate format?
    pub coordinates: Option<String>,
}

#[cfg(test)]
mod tests {
    use clap::IntoApp;
    use super::Opts;
    #[test]
    fn verify_app() {
        Opts::into_app().debug_assert()
    }
}
