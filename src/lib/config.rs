use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{
    File,
    read_to_string
};
use std::io::Write;

use home::home_dir;


use crate::lib::repository::{
    Repository
};

use crate::lib::repotype::{
    RepoType
};

use crate::lib::action::{
    Action
};

use crate::lib::group::{
    Group
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename(serialize = "repo_type", deserialize = "repo_type"), default)]
    pub repo_types: HashMap<String, RepoType>,
    #[serde(rename(serialize = "repository", deserialize = "repository"), default)]
    pub repositories: HashMap<String, Repository>,
    #[serde(rename(serialize = "action", deserialize = "action"), default)]
    pub actions: HashMap<String, Action>,
    #[serde(rename(serialize = "group", deserialize = "group"), default)]
    pub groups: HashMap<String, Group>,
}

pub fn find_config_file(original: Option<&str>) -> PathBuf {
    match original {
        None => {
            let config_name = PathBuf::from(".config/sync-it/config.toml");
            let mut path_name = home_dir().unwrap().join(config_name);
            if path_name.exists() {
                return path_name;
            }
            path_name = home_dir().unwrap().join(PathBuf::from(".sync-it.toml"));
            return path_name;
        },
        Some(p) => return PathBuf::from(&p),
    }
}

pub fn read_configuration_file(filename: &PathBuf) -> Config {
    let text = read_to_string(filename);
    match text {
        Err(_) => {
            let config = Config {
                repo_types: HashMap::new(),
                repositories: HashMap::new(),
                actions: HashMap::new(),
                groups: HashMap::new()
            };
            return config;
        },
        Ok(s) => return toml::from_str(&s).unwrap()
    }
}

pub fn write_configuration_file(filename: PathBuf, configuration: Config) -> std::io::Result<()> {
    let toml = toml::to_string(&configuration).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}
