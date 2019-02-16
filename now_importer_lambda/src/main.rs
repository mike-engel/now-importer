use http::StatusCode;
use now_lambda::{error::NowError, lambda, Body, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct RequestData {
    url: String,
}

#[derive(Serialize, Debug)]
struct ResponseData {
    url: String,
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    match req.body() {
        Body::Text(body) => {
            let data: RequestData = from_str(body).unwrap();
            let response = Response::builder()
                .status(StatusCode::OK)
                .body("OK")
                .expect("Internal Server Error");

            Ok(response)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Request body can only be a string")
            .expect("Internal Server Error")),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
