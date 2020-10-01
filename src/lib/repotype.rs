use serde::{Deserialize, Serialize};
use std::fmt;

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
