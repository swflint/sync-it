use clap::Values;
use crate::lib::{
    config::Config,
    repository::Repository,
    group::Group,
    repotype::RepoType,
    action::Action
};

pub fn run(config: &Config, names: Values<'_>) {
    for name in names {
        println!("Running {}...", name)
    }
}
