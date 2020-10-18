use serde::{Deserialize, Serialize};
use std::fmt;

use crate::lib::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Group {
    #[serde(default)]
    name: String,
    #[serde(default)]
    pub actions_after: Vec<String>,
    #[serde(default)]
    pub members: Vec<String>,
}

pub fn add(config: &mut Config, name: &String) {
    let group = Group {
        name: name.to_string(),
        actions_after: Vec::new(),
        members: Vec::new()
    };
    config.groups.insert(name.to_string(), group);
}

pub fn add_repo(config: &mut Config, name: &String, repo: &String) {
    match config.groups.get_mut(&name.to_string()) {
        Some(group) => group.members.push(repo.to_string()),
        None => panic!("No known group named \"{}\".", name)
    }
}

pub fn add_action(config: &mut Config, name: &String, action: &String) {
    match config.groups.get_mut(&name.to_string()) {
        Some(group) => group.actions_after.push(action.to_string()),
        None => panic!("No known group named \"{}\".", name)
    }
}

pub fn remove_repo(config: &mut Config, name: &String, repo: &String) {
    match config.groups.get_mut(&name.to_string()) {
        Some(group) => group.members.retain(|r| r != repo),
        None => panic!("No known group named \"{}\".", name)
    }
}

pub fn remove_group(config: &mut Config, name: &String) {
    config.groups.remove(&name.to_string());
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
