use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

use crate::lib::config::Config;

#[derive(Serialize, Deserialize)]
pub struct RepoType {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    pub create: String,
    #[serde(default)]
    pub inward: String,
    #[serde(default)]
    pub outward: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    pub pre_inward: String,
    #[serde(default)]
    pub post_inward: String,
    #[serde(default)]
    pub post_outward: String,
    #[serde(default)]
    pub commands: BTreeMap<String,String>
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
        post_outward: post_outward.to_string(),
        commands: BTreeMap::new()
    };
    config.repo_types.insert(name.to_string(), repo_type);
    config.is_changed = true;
}

pub fn update_description(config: &mut Config, name: &String, description: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.description = description.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_create(config: &mut Config, name: &String, create: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.create = create.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_inward(config: &mut Config, name: &String, inward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.inward = inward.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_outward(config: &mut Config, name: &String, outward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.outward = outward.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_status(config: &mut Config, name: &String, status: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.status = status.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_pre_inward(config: &mut Config, name: &String, pre_inward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.pre_inward = pre_inward.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_post_inward(config: &mut Config, name: &String, post_inward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.post_inward = post_inward.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn update_post_outward(config: &mut Config, name: &String, post_outward: &String) {
    match config.repo_types.get_mut(&name.to_string()) {
        Some(repo_type) => {
            repo_type.post_outward = post_outward.to_string();
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", name)
    }
}

pub fn add_command(config: &mut Config, type_name: &String, name: &String, command: &String) {
    match config.repo_types.get_mut(&type_name.to_string()) {
        Some(repo_type) => {
            repo_type.commands.insert(name.to_string(), command.to_string());
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", type_name)
    }
}

pub fn change_command(config: &mut Config, type_name: &String, name: &String, command: &String) {
    match config.repo_types.get_mut(&type_name.to_string()) {
        Some(repo_type) => {
            repo_type.commands.insert(name.to_string(), command.to_string());
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", type_name)
    }
}

pub fn remove_command(config: &mut Config, type_name: &String, name: &String) {
    match config.repo_types.get_mut(&type_name.to_string()) {
        Some(repo_type) => {
            repo_type.commands.remove(&name.to_string());
            config.is_changed = true;
        },
        None => panic!("No known repository type named \"{}\".", type_name)
    }
}

impl fmt::Display for RepoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Repository type {}:\n\t\"{}\"\n\tCreation Command: {}\n\tInward Sync: {}\n\tOutward Sync: {}\n\tStatus: {}\n\tPre-inward: {}\n\tPost-inward: {}\n\tPost-outward: {}",
               self.name,
               self.description,
               self.create,
               self.inward,
               self.outward,
               self.status,
               self.pre_inward,
               self.post_inward,
               self.post_outward)?;
        write!(f, "Additional Commands:\n")?;
        for command in self.commands.keys() {
            write!(f, "\t - {}: {}\n", command, &self.commands.get(command).unwrap())?;
        }
        write!(f, "")
    }
}
