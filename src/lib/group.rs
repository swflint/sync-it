use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Group {
    #[serde(default)]
    name: String,
    #[serde(default)]
    actions_after: Vec<String>,
    #[serde(default)]
    members: Vec<String>,
}

/*impl fmt::Display for Group {
    
}*/
