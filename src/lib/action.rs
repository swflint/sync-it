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
    command: String,
    #[serde(default)]
    disabled: bool,
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
