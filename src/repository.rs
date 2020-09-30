use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::config::Config;

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

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Repository {}:\n\tPath: {}\n\tType: {}\n\tDisabled: {}\n\tOptions:\n",
               self.name,
               self.location,
               self.repo_type,
               self.disabled);
        for (key, value) in &self.options {
            write!(f, "\t\t{}: {}\n", key, value);
        }
        write!(f, "")
    }
}
