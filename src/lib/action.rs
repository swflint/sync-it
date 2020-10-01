use serde::{Deserialize, Serialize};
use std::fmt;

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
