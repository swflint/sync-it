use clap::Values;
use crate::lib::{
    config::Config,
    repository::Repository,
    group::Group,
    repotype::RepoType,
    action::Action
};


use std::process::Command;

pub fn run(config: &Config, names: Values<'_>) {
    for name in names {
        println!("Running {}...", name)
    }
}
pub fn run_action(config: &Config, name: String) {
    let action = config.actions.get(&name.to_string());
    match action {
        Some(action) => {
            if !action.disabled {
                Command::new("sh")
                    .arg("-c")
                    .arg(&action.command)
                    .spawn();
            }
        },
        None => panic!("No known action named \"{}\".", name)
    }
}
