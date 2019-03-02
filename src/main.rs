mod importer;

use crate::importer::{import_website, ImportError};
use http::StatusCode;
use now_lambda::{error::NowError, lambda, Body, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use simplelog::{Config, Level, LevelFilter, TermLogger};
use std::error::Error;

#[derive(Deserialize, Debug)]
struct RequestData {
    url: String,
    debug: bool,
    token: String,
}

#[derive(Serialize, Debug)]
struct ResponseData {
    url: String,
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    match req.body() {
        Body::Text(body) => match from_str(body) {
            Ok(RequestData { url, debug, token }) => {
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

                match import_website(&url, Some(&token)) {
                    Ok(published_url) => {
                        let response_data = ResponseData {
                            url: published_url.to_owned(),
                        };
                        let json = to_string(&response_data).unwrap();

                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .body(json)
                            .expect("Internal Server Error"))
                    }
                    Err(error) => match error {
                        ImportError::InvalidUrl => Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(format!("Invalid argument sent"))
                            .expect("Internal Server Error")),
                        ImportError::InternalError => Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(format!("Internal server error"))
                            .expect("Internal Server Error")),
                        _ => Ok(Response::builder()
                            .status(StatusCode::BAD_GATEWAY)
                            .body(format!("Unable to download or deploy your website"))
                            .expect("Internal Server Error")),
                    },
                }
            }
            Err(_) => Ok(Response::builder()
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
    Ok(lambda!(handler))
}
