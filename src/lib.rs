use serde_json::json;
use url::Url;

enum ImportError {
  InvalidUrl,
  DownloadFailed,
  DeployFailed,
  InternalError,
}

fn build_now_config<S: Into<String>>(name: S) -> String {
  let config = json!({
    "version": 2,
    "name": name.into(),
    "builds": [{ "src": "*", "use": "@now/rust@canary", "config": { "newPipeline": true } }]
  });

  config.to_string()
}

fn create_name(url: &str) -> Result<String, ImportError> {
  let parse_result = Url::parse(url);

  match parse_result {
    Ok(attributes) => match attributes.host_str() {
      Some(host) => Ok(host.to_owned()),
      None => Err(ImportError::InvalidUrl),
    },
    Err(_) => Err(ImportError::InvalidUrl),
  }
}
