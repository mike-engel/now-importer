use log::debug;
use regex::Regex;
use serde_json::{json, to_string_pretty};
use std::fs;
use std::process::Command;
use url::{self, Url};

pub enum ImportError {
  InvalidUrl(Option<String>),
  DownloadFailed(Option<String>),
  DeployFailed(Option<String>),
  InternalError(Option<String>),
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

        Err(ImportError::InvalidUrl(Some("URL has no host".to_owned())))
      }
    },
    Err(error) => {
      debug!("URL can't be parsed: {:?}", error);

      Err(ImportError::InvalidUrl(Some(format!("{:?}", error))))
    }
  }
}

fn build_now_config(name: &str) -> String {
  debug!("creating now config with name {}", name);

  let config = json!({
    "version": 2,
    "name": name,
    "builds": [{ "src": "**/*", "use": "@now/static" }]
  });

  to_string_pretty(&config).unwrap()
}

fn prepare_dir(destination: &str) -> Result<(), ImportError> {
  debug!("preparing the download directory");

  match fs::create_dir_all(destination) {
    Ok(_) => {
      Ok(())
    },
    Err(error) => {
      debug!("unable to create download dir: {:?}", error);

      Err(ImportError::InternalError(Some(format!("unable to create download dir: {:?}", error))))
    }
  }
}

fn download_website(url: &str, destination: &str) -> Result<(), ImportError> {
  debug!("starting website download for url {}", url);

  let wget = Command::new("wget")
    .arg("--recursive")
    .arg("--no-clobber")
    .arg("--page-requisites")
    .arg("--tries=3")
    .arg("--no-host-directories")
    .arg("--quiet")
    .arg(url)
    .current_dir(destination)
    .status();

  match wget {
    Ok(result) => match result.code() {
      Some(0) => {
        debug!("wget finished website download");

        Ok(())
      }
      _ => {
        debug!("wget exited with non-zero exit code");

        Err(ImportError::DownloadFailed(Some(
          "wget exited with non-zero exit code".to_owned(),
        )))
      }
    },
    Err(error) => {
      debug!("wget command failed with error: {}", error);

      Err(ImportError::DownloadFailed(Some(format!("{:?}", error))))
    }
  }
}

fn save_now_config(config: String, destination: &str) -> Result<(), ImportError> {
  debug!("saving now config");

  match fs::write(format!("{}/now.json", destination), &config) {
    Ok(_) => {
      debug!("now config added to {}", destination);

      Ok(())
    }
    Err(error) => {
      debug!("Failed to save now config to {}: {}", destination, error);

      Err(ImportError::InternalError(Some(format!("{:?}", error))))
    }
  }
}

fn deploy_site(token: Option<&str>, config_dir: &str, destination: &str) -> Result<String, ImportError> {
  debug!("deploying website to now");

  let mut now = Command::new("now");

  if let Some(now_token) = token {
    now.arg(format!("--token={}", now_token));
  }

  let now_output = now.current_dir(destination).arg(format!("--global-config={}", config_dir)).output();
  let url_regex = Regex::new(r"\b(https://.+\.now\.sh)\b").unwrap();

  match now_output {
    Ok(result) => {
      if !result.status.success() {
        debug!("now cli failed with the following code and output: `{:?}` {:?}", result.status.code().unwrap_or(1), String::from_utf8(result.stderr));

        return Err(ImportError::DeployFailed(Some(
          "now exited with a non-zero exit code".to_owned(),
        )));
      }

      match url_regex.captures(&String::from_utf8(result.stdout).unwrap()) {
        Some(matches) => {
          let deploy_url = matches.get(1);

          match deploy_url {
            Some(url) => Ok(url.as_str().to_owned()),
            None => {
              debug!("Now output didn't contain a URL that matched the pattern");

              Err(ImportError::DeployFailed(Some(
                "Now output didn't contain a URL that matched the pattern".to_owned(),
              )))
            }
          }
        }
        _ => {
          debug!("The output from now couldn't be tested with a regex");

          Err(ImportError::DeployFailed(Some(
            "The output from now couldn't be tested with a regex".to_owned(),
          )))
        }
      }
    }
    Err(error) => {
      debug!("Error deploying the site to now: {:?}", error);

      Err(ImportError::DeployFailed(Some(format!("{:?}", error))))
    }
  }
}

fn cleanup(destination: &str) -> Result<(), ImportError> {
  match fs::remove_dir_all(destination) {
    Ok(_) => Ok(()),
    Err(error) => {
      debug!("Error cleaning up after the deploy: {:?}", error);

      Err(ImportError::InternalError(Some(format!("{:?}", error))))
    }
  }
}

pub fn import_website(url: &str, now_token: Option<&str>, now_config_directory: &str, folder_path: &str) -> Result<String, ImportError> {
  let project_name = create_name(url)?;
  let destination_dir = format!("{}/{}", folder_path, &project_name);
  let now_config = build_now_config(&project_name);

  prepare_dir(&destination_dir)?;

  download_website(url, &destination_dir)?;

  save_now_config(now_config, &destination_dir)?;

  let published_url = deploy_site(now_token, now_config_directory, &destination_dir)?;

  cleanup(&destination_dir)?;

  Ok(published_url)
}
