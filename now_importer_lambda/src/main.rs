use http::{StatusCode, header};
use now_importer::{import_website, ImportError};
use now_lambda::{error::NowError, lambda, Body, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use simplelog::{Config, Level, LevelFilter, SimpleLogger};
use std::env;
use std::error::Error;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct RequestData {
    url: String,
    debug: bool,
    token: String,
}

#[derive(Serialize, Debug)]
struct ResponseData {
    url: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ExchangeResponse {
    access_token: String,
}

const LOG_CONFIG: Config = Config {
    time: Some(Level::Debug),
    level: Some(Level::Debug),
    target: None,
    location: None,
    time_format: Some("%T"),
};

fn setup_path() -> Result<(), Box<dyn Error>> {
    let existing_path = env::var("PATH")?;
    let mut existing_paths = env::split_paths(&existing_path).collect::<Vec<_>>();
    let current_dir = env::current_dir()?;
    let static_path = format!("{}/static", current_dir.display());

    existing_paths.push(PathBuf::from(static_path));

    let new_path = env::join_paths(existing_paths)?;

    env::set_var("PATH", &new_path);

    Ok(())
}

fn error_response<E: std::fmt::Debug>(message: &str, debug: bool, error: E) -> ResponseData {
    let error_message = match debug {
        true => format!("{}: {:?}", message, error),
        false => format!("{}", message),
    };

    ResponseData {
        url: None,
        error: Some(error_message),
    }
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    match req.body() {
        Body::Text(body) => match from_str(body) {
            Ok(RequestData { url, debug, token }) => {
                match import_website(&url, &token, "/tmp/dist") {
                    Ok(published_url) => {
                        let response_data = ResponseData {
                            url: Some(published_url.to_owned()),
                            error: None,
                        };
                        let json = to_string(&response_data).unwrap();

                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(json)
                            .expect("Internal Server Error"))
                    }
                    Err(ImportError::InvalidUrl(error)) => Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(
                            to_string(&error_response("Invalid argument sent", debug, error))
                                .unwrap(),
                        )
                        .expect("Internal Server Error")),
                    Err(ImportError::InternalError(error)) => Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(
                            to_string(&error_response("Internal server error", debug, error))
                                .unwrap(),
                        )
                        .expect("Internal Server Error")),
                    Err(ImportError::DownloadFailed(error)) => Ok(Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .body(
                            to_string(&error_response(
                                "Unable to download your website",
                                debug,
                                error,
                            ))
                            .unwrap(),
                        )
                        .expect("Internal Server Error")),
                    Err(ImportError::DeployFailed(error)) => Ok(Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .body(
                            to_string(&error_response(
                                "Unable to deploy your website",
                                debug,
                                error,
                            ))
                            .unwrap(),
                        )
                        .expect("Internal Server Error")),
                }
            }
            _ => Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("Invalid argument sent"))
                .expect("Internal Server Error")),
        },
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(format!("Request body can only be a string"))
            .expect("Internal Server Error")),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    match SimpleLogger::init(LevelFilter::Debug, LOG_CONFIG) {
        Ok(_) => {}
        Err(error) => eprintln!("Error setting up SimpleLogger: {:?}", error),
    };

    setup_path()?;

    Ok(lambda!(handler))
}
