use serde::{Deserialize, Serialize};
use std::fmt;

use crate::lib::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Group {
    #[serde(default)]
    name: String,
    #[serde(default)]
    actions_after: Vec<String>,
    #[serde(default)]
    members: Vec<String>,
}

pub fn add(config: &mut Config, name: &String) {
    let group = Group {
        name: name.to_string(),
        actions_after: Vec::new(),
        members: Vec::new()
    };
    config.groups.insert(name.to_string(), group);
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group {}:\n\tRepos:\n", self.name)?;
        for repo in &self.members {
            write!(f, "\t\t - {}\n", repo)?;
        }
        write!(f, "\tActions\n")?;
        for action in &self.actions_after {
            write!(f, "\t\t - {}\n", action)?;
        }
        write!(f, "")
    }
}
