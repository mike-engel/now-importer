use clap::{App, Arg};
use log::{self, info};
use now_importer::{import_website, ImportError};
use simplelog::{Config, Level, LevelFilter, TermLogger};

fn main() -> Result<(), String> {
    let matches = App::new("now importer")
        .version("0.1")
        .about("Import your current website into the now platform")
        .arg(
            Arg::with_name("DEBUG")
                .short("-d")
                .long("--debug")
                .help("Print extra information to the console")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("NOW_TOKEN")
                .short("-t")
                .long("--token")
                .help("The now authentication token to deploy to now with")
                .required(false),
        )
        .arg(
            Arg::with_name("URL")
                .help("The URL of your website to import")
                .required(true)
                .index(1),
        )
        .get_matches();
    let debug = matches.is_present("DEBUG");
    let url = matches.value_of("URL").unwrap();
    let now_token = matches.value_of("NOW_TOKEN");
    let log_config = Config {
        time: Some(Level::Debug),
        level: Some(Level::Debug),
        target: None,
        location: None,
        time_format: Some("%T"),
    };

    match debug {
        true => TermLogger::init(LevelFilter::Debug, log_config).unwrap(),
        false => TermLogger::init(LevelFilter::Info, log_config).unwrap(),
    };

    match import_website(url, now_token) {
        Ok(deploy_url) => {
            info!("Project successully deployed to {}", deploy_url);

            Ok(())
        }
        Err(error) => match error {
            ImportError::DeployFailed => Err(String::from("Deploying to now failed. Try again!")),
            ImportError::DownloadFailed => Err(String::from("Download failed. Try again!")),
            ImportError::InternalError => Err(String::from("Something went wrong")),
            ImportError::InvalidUrl => Err(String::from("Invalid URL. Try again!")),
        },
    }
}
