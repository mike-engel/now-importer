mod file_upload;

use self::file_upload::{collect_files, create_deployment, get_deployment_alias, upload_files};
use log::debug;
use std::fs;
use std::path::Path;
use std::process::Command;
use url::{self, Url};

#[derive(Debug)]
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

fn prepare_dir(destination: &str) -> Result<(), ImportError> {
  debug!("preparing the download directory");

  match fs::create_dir_all(destination) {
    Ok(_) => Ok(()),
    Err(error) => {
      debug!("unable to create download dir: {:?}", error);

      Err(ImportError::InternalError(Some(format!(
        "unable to create download dir: {:?}",
        error
      ))))
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

fn deploy_site(name: &str, token: &str, destination: &str) -> Result<String, ImportError> {
  debug!("deploying website to now");

  let path = Path::new(destination);
  let result = collect_files(&path, &path)
    .and_then(|files| upload_files(files, token))
    .and_then(|files| create_deployment(name, files, token))
    .and_then(|(id, url)| {
      let alias = get_deployment_alias(&id, token);

      Ok(alias.unwrap_or(url))
    });

  match result {
    Ok(url) => Ok(url),
    Err(err) => {
      debug!("Error creating a new deployment: {:?}", err);

      Err(err)
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

pub fn import_website(
  url: &str,
  now_token: &str,
  folder_path: &str,
) -> Result<String, ImportError> {
  let project_name = create_name(url)?;
  let destination_dir = format!("{}/{}", folder_path, &project_name);

  prepare_dir(&destination_dir)?;

  download_website(url, &destination_dir)?;

  let published_url = deploy_site(&project_name, now_token, &destination_dir)?;

  cleanup(&destination_dir)?;

  Ok(published_url)
}
