use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::lib::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Repository {
    #[serde(default)]
    name: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub repo_type: String,
    #[serde(default)]
    pub auto_create: bool,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub options: HashMap<String, String>,
}

pub fn register(config: &mut Config, name: &String, location: String, repo_type: String, options_strings: Vec<String>) {
    let mut options_map: HashMap<String, String> = HashMap::new();
    for option in options_strings {
        let option_pair: Vec<&str> = option.split("=").collect();
        options_map.insert(option_pair[0].to_string(), option_pair[1].to_string());
    }
    let repo = Repository {
        name: name.to_string(),
        location: location,
        repo_type: repo_type,
        auto_create: true,
        disabled: false,
        options: options_map
    };
    config.repositories.insert(name.to_string(), repo);
}

pub fn update_disabled(config: &mut Config, name: &String, value: bool) {
    let repo = config.repositories.get_mut(&name.to_string());
    match repo {
        Some(repo) => repo.disabled = value,
        None => panic!("No known repository named \"{}\".", name)
    }
}

pub fn update_autocreate(config: &mut Config, name: &String, value: bool) {
    let repo = config.repositories.get_mut(&name.to_string());
    match repo {
        Some(repo) => repo.auto_create = value,
        None => panic!("No known repository named \"{}\".", name)
    }
}

pub fn update_options(config: &mut Config, name: &String, options_strings: Vec<String>) {
    let repo: Option<&mut Repository> = config.repositories.get_mut(&name.to_string());
    match repo {
        Some(repo) => for option in options_strings {
            let option_pair: Vec<&str> = option.split("=").collect();
            repo.options.insert(option_pair[0].to_string(), option_pair[1].to_string());
        }
        None => panic!("No known repository named \"{}\".", name)
    }
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Repository {}:\n\tPath: {}\n\tType: {}\n\tDisabled: {}\n\tOptions:\n",
               self.name,
               self.location,
               self.repo_type,
               self.disabled)?;
        for (key, value) in &self.options {
            write!(f, "\t\t{}: {}\n", key, value)?;
        }
        write!(f, "")
    }
}
