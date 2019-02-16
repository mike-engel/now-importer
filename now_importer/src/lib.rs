use clipboard::{ClipboardContext, ClipboardProvider};
use log::debug;
use serde_json::{json, to_string_pretty};
use std::fs;
use std::process::Command;
use url::Url;

pub enum ImportError {
  InvalidUrl,
  DownloadFailed,
  DeployFailed,
  InternalError,
}

fn create_name(url: &str) -> Result<String, ImportError> {
  debug!("creating project name");

  let parse_result = Url::parse(url);

  match parse_result {
    Ok(attributes) => match attributes.host_str() {
      Some(host) => Ok(host.replace(".", "-").to_owned()),
      None => Err(ImportError::InvalidUrl),
    },
    Err(_) => Err(ImportError::InvalidUrl),
  }
}

fn build_now_config<S: Into<String>>(name: S) -> String {
  debug!("creating now config");

  let config = json!({
    "version": 2,
    "name": name.into(),
    "builds": [{ "src": "**/*", "use": "@now/static" }]
  });

  to_string_pretty(&config).unwrap()
}

fn download_website(url: &str) -> Result<(), ImportError> {
  debug!("starting website download");

  let wget = Command::new("wget")
    .arg("--recursive")
    .arg("--no-clobber")
    .arg("--page-requisites")
    .arg("--tries=3")
    .arg("--directory-prefix=dist")
    .arg("--no-host-directories")
    .arg("--quiet")
    .arg(url)
    .status();

  match wget {
    Ok(result) => match result.code() {
      Some(0) => Ok(()),
      _ => Err(ImportError::DownloadFailed),
    },
    Err(_) => Err(ImportError::DownloadFailed),
  }
}

fn save_now_config(config: String) -> Result<(), ImportError> {
  debug!("saving now config");

  match fs::write("dist/now.json", &config) {
    Ok(_) => Ok(()),
    Err(_) => Err(ImportError::InternalError),
  }
}

fn deploy_site() -> Result<String, ImportError> {
  debug!("deploying website to now");

  let mut clipboard_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
  let now = Command::new("now").current_dir("dist").status();

  match now {
    Ok(result) => match result.code() {
      Some(0) => {
        let deploy_url = clipboard_ctx.get_contents();

        match deploy_url {
          Ok(url) => Ok(url),
          Err(_) => Err(ImportError::DeployFailed),
        }
      }
      _ => Err(ImportError::DeployFailed),
    },
    Err(_) => Err(ImportError::DeployFailed),
  }
}

pub fn import_website(url: &str) -> Result<String, ImportError> {
  let project_name = create_name(url)?;
  let now_config = build_now_config(project_name);

  download_website(url)?;

  save_now_config(now_config)?;

  let published_url = deploy_site()?;

  Ok(published_url)
}
