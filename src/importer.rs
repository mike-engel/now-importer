use log::debug;
use regex::Regex;
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
      Some(host) => {
        debug!("Host found on the URL");

        Ok(host.replace(".", "-").to_owned())
      }
      None => {
        debug!("URL has no host");

        Err(ImportError::InvalidUrl)
      }
    },
    Err(_) => {
      debug!("URL can't be parsed");

      Err(ImportError::InvalidUrl)
    }
  }
}

fn build_now_config<S: Into<String>>(name: S) -> String {
  let usable_name = name.into();

  debug!("creating now config with name {}", usable_name);

  let config = json!({
    "version": 2,
    "name": usable_name,
    "builds": [{ "src": "**/*", "use": "@now/static" }]
  });

  to_string_pretty(&config).unwrap()
}

fn download_website(url: &str) -> Result<(), ImportError> {
  debug!("starting website download for url {}", url);

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
      Some(0) => {
        debug!("wget finished website download");

        Ok(())
      }
      _ => {
        debug!("wget exited with non-zero exit code");

        Err(ImportError::DownloadFailed)
      }
    },
    Err(error) => {
      debug!("wget command failed with error: {}", error);

      Err(ImportError::DownloadFailed)
    }
  }
}

fn save_now_config(config: String) -> Result<(), ImportError> {
  debug!("saving now config");

  match fs::write("dist/now.json", &config) {
    Ok(_) => {
      debug!("now config added to dist/");

      Ok(())
    }
    Err(error) => {
      debug!("Failed to save now config to dist/: {}", error);

      Err(ImportError::InternalError)
    }
  }
}

fn deploy_site(token: Option<&str>) -> Result<String, ImportError> {
  debug!("deploying website to now");

  let mut now = Command::new("now");

  if let Some(now_token) = token {
    now.arg(format!("--token=\"{}\"", now_token));
  }

  let now_output = now.current_dir("dist").output();
  let url_regex = Regex::new(r"\b(https://.+\.now\.sh)\b").unwrap();

  match now_output {
    Ok(result) => {
      if !result.status.success() {
        return Err(ImportError::DeployFailed);
      }

      match url_regex.captures(&String::from_utf8(result.stdout).unwrap()) {
        Some(matches) => {
          let deploy_url = matches.get(1);

          match deploy_url {
            Some(url) => Ok(url.as_str().to_owned()),
            None => Err(ImportError::DeployFailed),
          }
        }
        _ => Err(ImportError::DeployFailed),
      }
    }
    Err(_) => Err(ImportError::DeployFailed),
  }
}

pub fn import_website(url: &str, now_token: Option<&str>) -> Result<String, ImportError> {
  let project_name = create_name(url)?;
  let now_config = build_now_config(project_name);

  download_website(url)?;

  save_now_config(now_config)?;

  let published_url = deploy_site(now_token)?;

  Ok(published_url)
}
