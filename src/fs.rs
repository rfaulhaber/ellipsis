use crate::config::Config;
use anyhow::Result;
use serde::Deserialize;
use std::{
    borrow::Cow,
    fs::{self, DirEntry},
};
use std::{env, path::Path};
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
    let contents = match path {
        Some(s) => fs::read_to_string(Path::new(&s))?,
        None => {
            let current_dir = env::current_dir()?;
            println!("reading {:?}", current_dir);

            let mut contents: Option<String> = None;
            for entry in fs::read_dir(current_dir)? {
                let entry = entry?;
                let (file_name, path) = (entry.file_name(), entry.path());

                let entry_name = match file_name.to_str() {
                    Some(e) => e,
                    None => continue,
                };

                println!("name {:?}", entry_name);
                if String::from(entry_name).contains("ellipsis") {
                    contents = Some(fs::read_to_string(path.clone())?);
                    break;
                } else {
                    continue;
                }
            }

            match contents {
                Some(c) => c,
                None => return Err(LoadConfigError::MissingFile.into()),
            }
        }
    };

    // TODO parse based on extension
    println!("reading as yaml");
    let res: Config = match serde_yaml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            println!("reading as json {:?}", e);
            serde_json::from_str(&contents)?
        }
    };

    Ok(res)
}
