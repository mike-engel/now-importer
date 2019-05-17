use clap::{App, Arg};
use dirs::home_dir;
use log::{self, info};
use now_importer::{import_website, ImportError};
use serde::Deserialize;
use serde_json::from_str;
use simplelog::{Config, Level, LevelFilter, TermLogger};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct NowAuth {
    token: String,
}

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
                .takes_value(true)
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
    let now_token = match matches.value_of("NOW_TOKEN") {
        Some(token) => token.to_owned(),
        None => {
            let home = home_dir().unwrap();
            let now_auth_path = home.join(Path::new(".now/auth.json"));
            let file = File::open(now_auth_path);
            let mut auth_contents = String::new();

            if let Err(_) = file {
                panic!("You're not logged into now and no token was provided. Either log in with `now login` or provide a token.");
            }

            file.unwrap().read_to_string(&mut auth_contents).unwrap();

            match from_str(&auth_contents) {
                Ok(NowAuth { token }) => token,
                _ => panic!("Unable to read/parse the now auth configuration. Please try again."),
            }
        }
    };
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

    match import_website(url, &now_token, "./dist") {
        Ok(deploy_url) => {
            info!("Project successully deployed to {}", deploy_url);

            Ok(())
        }
        Err(ImportError::DeployFailed(_)) => {
            Err(String::from("Deploying to now failed. Try again!"))
        }
        Err(ImportError::DownloadFailed(_)) => Err(String::from("Download failed. Try again!")),
        Err(ImportError::InternalError(_)) => Err(String::from("Something went wrong")),
        Err(ImportError::InvalidUrl(_)) => Err(String::from("Invalid URL. Try again!")),
    }
}
