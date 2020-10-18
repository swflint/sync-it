use serde::{Deserialize, Serialize};
use std::fmt;

use crate::lib::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Action {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub disabled: bool,
}

pub fn add(config: &mut Config, name: &String, description: &String, command: &String) {
    let action = Action {
        name : name.to_string(),
        description: description.to_string(),
        command: command.to_string(),
        disabled: false
    };
    config.actions.insert(name.to_string(), action);
}

pub fn update_disabled(config: &mut Config, name: &String, value: bool) {
    let action = config.actions.get_mut(&name.to_string());
    match action {
        Some(action) => action.disabled = value,
        None => panic!("No known action named \"{}\".", name)
    }
}

pub fn update_description(config: &mut Config, name: &String, description: &String) {
    let action = config.actions.get_mut(&name.to_string());
    match action {
        Some(action) => action.description = description.to_string(),
        None => panic!("No known action named \"{}\".", name)
    }
}

pub fn update_command(config: &mut Config, name: &String, command: &String) {
    let action = config.actions.get_mut(&name.to_string());
    match action {
        Some(action) => action.command = command.to_string(),
        None => panic!("No known action named \"{}\".", name)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Action {}:\n\t\"{}\"\n\tCommand: \"{}\"\n\tDisabled: {}",
               self.name,
               self.description,
               self.command,
               self.disabled
        )
    }
}
