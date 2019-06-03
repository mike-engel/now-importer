use crate::ImportError;
use base64::encode;
use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::iter::Iterator;
use std::path::Path;

const UPLOAD_URL: &'static str = "https://api.zeit.co/v2/now/files";
const DEPLOY_URL: &'static str = "https://api.zeit.co/v9/now/deployments";
const DEPLOYMENT_URL: &'static str = "https://api.zeit.co/v9/now/deployments/";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NowFile {
  pub file: String,
  #[serde(skip_serializing)]
  pub filename: String,
  pub sha: String,
  pub size: usize,
  #[serde(skip_serializing)]
  pub content: String,
}

#[derive(Debug, Deserialize)]
struct NowCreateDeploymentResponse {
  url: String,
  id: String,
}

#[derive(Debug, Deserialize)]
struct NowDeploymentResponse {
  #[serde(alias = "aliasFinal")]
  alias_final: Vec<String>,
}

fn build_now_config(name: &str, files: Vec<NowFile>) -> String {
  debug!("creating now config with name {}", name);

  let config = json!({
    "version": 2,
    "name": name,
    "builds": [{ "src": "**/*", "use": "@now/static" }],
    "files": files,
    "target": "staging",
    "meta": { "imported": "true" }
  });

  to_string(&config).unwrap()
}

pub(crate) fn collect_files(path: &Path, base: &Path) -> Result<Vec<NowFile>, ImportError> {
  let mut files: Vec<NowFile> = vec![];

  if path.is_dir() {
    let entries = fs::read_dir(path);

    if let Err(err) = entries {
      debug!("Unable to read the files for deployment: {:?}", err);

      return Err(ImportError::DownloadFailed(Some(format!(
        "Unable to read the files for deployment: {:?}",
        err
      ))));
    }

    let mut new_files: Vec<NowFile> = entries
      .unwrap()
      .filter(|entry| {
        if entry.is_err() {
          error!("entry error: {:?}", entry);
        }

        entry.is_ok()
      })
      .map(|entry| entry.unwrap())
      .map(|dir_entry| {
        if dir_entry.path().is_dir() {
          debug!("{} is a dir", dir_entry.path().to_str().unwrap_or(""));

          return collect_files(&dir_entry.path(), base);
        }

        let file = File::open(dir_entry.path());

        if let Err(err) = file {
          debug!("Unable to open deployment file: {:?}", err);

          return Err(ImportError::DownloadFailed(Some(format!(
            "Unable to open deployment file: {:?}",
            err
          ))));
        }

        let mut reader = BufReader::new(file.unwrap());
        let mut contents = String::new();
        let mut hasher = Sha1::new();

        let formatted_path = format!(
          "{}",
          dir_entry
            .path()
            .strip_prefix(base)
            .unwrap()
            .to_str()
            .unwrap_or("")
        );
        let filename = dir_entry
          .file_name()
          .into_string()
          .unwrap_or(String::from(""));

        debug!("formatted_path: {:?}", formatted_path);

        match reader.read_to_string(&mut contents) {
          Ok(_) => {
            hasher.input(&contents);

            let hash_result = hasher.result();

            Ok(vec![NowFile {
              file: formatted_path,
              filename: filename,
              size: contents.len(),
              content: contents,
              sha: format!("{:x}", hash_result),
            }])
          }
          Err(_) => {
            let mut contents = Vec::new();

            let read_result = reader.read_to_end(&mut contents);

            if let Err(err) = read_result {
              debug!("Unable to read deployment file into memory: {:?}", err);

              return Err(ImportError::DownloadFailed(Some(format!(
                "Unable to read deployment file into memory: {:?}",
                err
              ))));
            }

            hasher.input(&contents);

            let encoded = encode(&contents);

            let hash_result = hasher.result();

            Ok(vec![NowFile {
              file: formatted_path,
              filename: filename,
              size: contents.len(),
              content: encoded,
              sha: format!("{:x}", hash_result),
            }])
          }
        }
      })
      .filter(|file| {
        if file.is_err() {
          error!("file is an err: {:?}", file);
        }

        file.is_ok()
      })
      .map(|file| file.unwrap())
      .flatten()
      .collect();

    files.append(&mut new_files);
  }

  Ok(files)
}

pub(crate) fn upload_files(files: Vec<NowFile>, token: &str) -> Result<Vec<NowFile>, ImportError> {
  let client = Client::new();

  for file in &files {
    let upload_result = client
      .post(UPLOAD_URL)
      .header("Content-Length", file.size.to_owned())
      .header("x-now-digest", file.sha.to_owned())
      .header("Authorization", format!("Bearer {}", token))
      .body(file.content.to_owned())
      .send();

    if let Err(err) = upload_result {
      debug!("Error uploading a file to now: {:?}", err);

      return Err(ImportError::DeployFailed(Some(format!(
        "Error uploading a file to now: {:?}",
        err
      ))));
    }
  }

  Ok(files)
}

pub(crate) fn create_deployment(
  name: &str,
  files: Vec<NowFile>,
  token: &str,
) -> Result<(String, String), ImportError> {
  let client = Client::new();
  let config = build_now_config(name, files);

  debug!("config: \n\n{:?}\n\n", config);

  let create_result = client
    .post(DEPLOY_URL)
    .header("Authorization", format!("Bearer {}", token))
    .header("Content-Type", "application/json")
    .body(config)
    .send()
    .and_then(|mut res| res.json());

  match create_result {
    Ok(NowCreateDeploymentResponse { id, url }) => Ok((id, url)),
    Err(err) => {
      debug!("Error creating the deploy on now: {:?}", err);

      Err(ImportError::DeployFailed(Some(format!(
        "Error creating the deploy on now: {:?}",
        err
      ))))
    }
  }
}

pub(crate) fn get_deployment_alias(id: &str, token: &str) -> Option<String> {
  let client = Client::new();

  let deployment_result = client
    .get(&format!("{}{}", DEPLOYMENT_URL, id))
    .header("Authorization", format!("Bearer {}", token))
    .send()
    .and_then(|mut res| res.json());

  match deployment_result {
    Ok(NowDeploymentResponse { alias_final }) => alias_final.into_iter().next(),
    _ => None,
  }
}
