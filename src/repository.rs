use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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
