use clap::Values;
use crate::lib::{
    config::Config,
    repository::Repository,
    group::Group,
    repotype::RepoType,
    action::Action
};

use string_template::Template;

use std::process::Command;
use std::path::Path;
use std::collections::HashMap;

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

pub fn run_repository_sync(config: &Config, name: String) {
    let repository = config.repositories.get(&name.to_string());
    match repository {
        Some(repository) => {
            if !repository.disabled {
                let location = &repository.location;
                if !Path::new(&location).exists() {
                    if repository.auto_create {
                        run_repository_creation(config, name);
                    }
                } else {
                    let mut options: HashMap<&str, &str> = HashMap::new();
                    for (key, value) in &repository.options {
                        options.insert(key, value);
                    }
                    let repo_type = config.repo_types.get(&repository.repo_type);
                    match repo_type {
                        Some(repo_type) => {
                            Command::new("sh")
                                .current_dir(location)
                                .arg("-c")
                                .arg(Template::new(&repo_type.pre_inward).render(&options))
                                .spawn();

                            Command::new("sh")
                                .current_dir(location)
                                .arg("-c")
                                .arg(Template::new(&repo_type.inward).render(&options))
                                .spawn();
                            
                            Command::new("sh")
                                .current_dir(location)
                                .arg("-c")
                                .arg(Template::new(&repo_type.post_inward).render(&options))
                                .spawn();
                            
                            Command::new("sh")
                                .current_dir(location)
                                .arg("-c")
                                .arg(Template::new(&repo_type.outward).render(&options))
                                .spawn();
                            
                            Command::new("sh")
                                .current_dir(location)
                                .arg("-c")
                                .arg(Template::new(&repo_type.post_outward).render(&options))
                                .spawn();
                        },
                        None => panic!("No known repository type named \"{}\".", &repository.repo_type)
                    }
                }
            }
        },
        None => panic!("No known repository named \"{}\".", name)
    }
}

pub fn run_repository_creation(config: &Config, name: String) {
    let repository = config.repositories.get(&name.to_string());
    match repository {
        Some(repository) => {
            let repository_type_name = &repository.repo_type;
            let repository_type = config.repo_types.get(repository_type_name);
            match repository_type {
                Some(repository_type) => {
                    if !repository.disabled {
                        let mut options: HashMap<&str, &str> = HashMap::new();
                        for (key, value) in &repository.options {
                            options.insert(key, value);
                        }
                        options.insert("location", &repository.location);
                        let create_command = Template::new(&repository_type.create);
                        Command::new("sh")
                            .arg("-c")
                            .arg(create_command.render(&options))
                            .spawn();
                    }
                },
                None => panic!("No known repository type named \"{}\".", repository_type_name)
            }
        },
        None => panic!("No known repository named \"{}\".", name)
    }
}
