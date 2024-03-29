// SPDX-FileCopyrightText: 2021 - 2022 Samuel W. Flint <swflint@flintfam.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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
    #[serde(skip)]
    pub is_changed: bool,
    #[serde(skip)]
    pub is_not_default: bool,
    #[serde(skip)]
    pub base_path: PathBuf,
    #[serde(rename(serialize = "repo_type", deserialize = "repo_type"), default)]
    pub repo_types: BTreeMap<String, RepoType>,
    #[serde(rename(serialize = "repository", deserialize = "repository"), default)]
    pub repositories: BTreeMap<String, Repository>,
    #[serde(rename(serialize = "action", deserialize = "action"), default)]
    pub actions: BTreeMap<String, Action>,
    #[serde(rename(serialize = "group", deserialize = "group"), default)]
    pub groups: BTreeMap<String, Group>,
}

pub fn find_config_file(original: Option<&String>) -> PathBuf {
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
                is_changed: true,
                is_not_default: false,
                base_path: filename.parent().unwrap().to_path_buf(),
                repo_types: BTreeMap::new(),
                repositories: BTreeMap::new(),
                actions: BTreeMap::new(),
                groups: BTreeMap::new()
            };
            return config;
        },
        Ok(s) => {
            let mut config: Config = toml::from_str(&s).unwrap();
            config.is_changed = false;
            return config;
        }
    }
}

pub fn write_configuration_file(filename: PathBuf, configuration: Config) -> std::io::Result<()> {
    if configuration.is_changed {
        let toml = toml::to_string_pretty(&configuration).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(toml.as_bytes())?;
        Ok(())
    } else {
        Ok(())
    }
}
