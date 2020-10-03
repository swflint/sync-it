use serde::{Deserialize, Serialize};
use std::fmt;

use crate::lib::config::Config;

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
    #[serde(default)]
    status: String,
    #[serde(default)]
    pre_inward: String,
    #[serde(default)]
    post_inward: String,
    #[serde(default)]
    post_outward: String,
}

pub fn add(config: &mut Config, name: &String, description: &String, create: &String, inward: &String, outward: &String, status: &String, pre_inward: &String, post_inward: &String, post_outward: &String) {
    let repo_type = RepoType {
        name: name.to_string(),
        description: description.to_string(),
        create: create.to_string(),
        inward: inward.to_string(),
        outward: outward.to_string(),
        status: status.to_string(),
        pre_inward: pre_inward.to_string(),
        post_inward: post_inward.to_string(),
        post_outward: post_outward.to_string()
    };
    config.repo_types.insert(name.to_string(), repo_type);
}

pub fn update_description(config: &mut Config, name: &String, description: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.description = description.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_create(config: &mut Config, name: &String, create: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.create = create.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_inward(config: &mut Config, name: &String, inward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.inward = inward.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_outward(config: &mut Config, name: &String, outward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.outward = outward.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_status(config: &mut Config, name: &String, status: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.status = status.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_pre_inward(config: &mut Config, name: &String, pre_inward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.pre_inward = pre_inward.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_post_inward(config: &mut Config, name: &String, post_inward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.post_inward = post_inward.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_post_outward(config: &mut Config, name: &String, post_outward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => repo_type.post_outward = post_outward.to_string(),
        None => panic!("No known repository type named \"{}\".", name)
    }
}

impl fmt::Display for RepoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Repository type {}:\n\t\"{}\"\n\tCreation Command: {}\n\tInward Sync: {}\n\tOutward Sync: {}",
               self.name,
               self.description,
               self.create,
               self.inward,
               self.outward)
    }
}
