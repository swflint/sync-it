use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{
    File,
    read_to_string
};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename(serialize = "repo_type", deserialize = "repo_type"), default)]
    repo_types: HashMap<String, RepoType>,
    #[serde(rename(serialize = "repository", deserialize = "repository"), default)]
    repositories: HashMap<String, Repository>,
    #[serde(rename(serialize = "action", deserialize = "action"), default)]
    actions: HashMap<String, Action>,
    #[serde(rename(serialize = "group", deserialize = "group"), default)]
    groups: HashMap<String, Group>,
}

#[derive(Serialize, Deserialize)]
pub struct RepoType {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    create: String,
    #[serde(default)]
    inward: String,
    #[serde(default)]
    outward: String,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    #[serde(default)]
    name: String,
    #[serde(default)]
    location: String,
    #[serde(default)]
    repo_type: String,
    #[serde(default)]
    auto_create: bool,
    #[serde(default)]
    disabled: bool,
    #[serde(default)]
    options: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    #[serde(default)]
    name: String,
    #[serde(default)]
    command: String,
    #[serde(default)]
    disabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    #[serde(default)]
    name: String,
    #[serde(default)]
    actions_after: Vec<String>,
    #[serde(default)]
    members: Vec<String>,
}

pub fn find_config_file(original: Option<&str>) -> PathBuf {
    match original {
        None => {
            if PathBuf::from("~/.config/sync-it/config.toml").exists() {
                return PathBuf::from("~/.config/sync-it/config.toml");
            } else {
                return PathBuf::from("~/.sync-it");
            }
        },
        Some(p) => return PathBuf::from(&p),
    }
}

pub fn read_configuration_file(filename: &PathBuf) -> Config {
    let text = read_to_string(filename);
    match text {
        Err(_) => {
            let mut config = Config {
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
