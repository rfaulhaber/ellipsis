use crate::config::Config;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::{
    env,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Deserialize, Error, Debug)]
pub enum LoadConfigError {
    #[error("Unsupported file type `{0}`")]
    UnsupportedFileType(String),
    #[error("Missing file type")]
    MissingFileType,
    #[error("Cannot find config file")]
    MissingFile,
}

pub fn read_config_file(path: Option<String>) -> Result<Config> {
    let path = match path {
        Some(s) => Path::new(&s).to_owned(),
        None => find_config_file()?,
    };

    let ext = path.extension();

    let contents = fs::read_to_string(path.to_owned())?;

    let config: Config = match ext {
        Some(e) => match e.to_str() {
            Some("yml") | Some("yaml") => serde_yaml::from_str(&contents)?,
            Some("json") => serde_json::from_str(&contents)?,
            Some(ext) => return Err(LoadConfigError::UnsupportedFileType(String::from(ext)).into()),
            None => serde_yaml::from_str(&contents)?,
        },
        None => serde_yaml::from_str(&contents)?,
    };

    Ok(config)
}

pub fn find_config_file() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let (file_name, path) = (entry.file_name(), entry.path());

        let entry_name = match file_name.to_str() {
            Some(e) => e,
            None => continue,
        };

        if String::from(entry_name).contains("ellipsis") {
            return Ok(path.to_owned());
        } else {
            continue;
        }
    }

    Err(LoadConfigError::MissingFile.into())
}
