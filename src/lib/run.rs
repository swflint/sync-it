use crate::lib::config::Config;

use string_template::Template;

use std::process::Command;
use std::path::Path;
use std::collections::HashMap;

pub fn run(config: &Config, names: Vec<&str>) {
    for name in names {
        if config.repositories.contains_key(name) {
            run_repository_sync(&config, name.to_string());
        } else if config.groups.contains_key(name) {
            run_group(&config, name.to_string());
        } else if name == "all" {
            for repo_name in config.repositories.keys() {
                run_repository_sync(&config, repo_name.to_string());
            }
        } else {
            println!("\"{}\" is neither a group nor a repository.", name);
        }
    }
}

pub fn run_with_command(config: &Config, command: &String, names: Vec<&str>) {
    for name in names {
        if config.repositories.contains_key(name) {
            run_named_command(&config, name.to_string(), &command);
        } else if config.groups.contains_key(name) {
            run_group_with_command(&config, name.to_string(), &command);
        } else if name == "all" {
            for repo_name in config.repositories.keys() {
                run_named_command(&config, repo_name.to_string(), &command);
            }
        } else {
            println!("\"{}\" is neither a group nor a repository.", name);
        }
    }
}

fn run_command(command: String) {
    if !command.is_empty() {
        match Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn() {
                Ok(mut child) => {
                    let mut status = child.try_wait();
                    loop {
                        match status {
                            Ok(Some(_)) => break,
                            Ok(None) => {},
                            _ => {}
                        }
                        status = child.try_wait();
                    }
                },
                _ => {}
            }
    }
}

fn run_command_in_directory(directory: String, command: String) {
    if !command.is_empty() {
        match Command::new("sh")
            .current_dir(directory)
            .arg("-c")
            .arg(command)
            .spawn() {
                Ok(mut child) => {
                    let mut status = child.try_wait();
                    loop {
                        match status {
                            Ok(Some(_)) => break,
                            Ok(None) => {},
                            _ => {}
                        }
                        status = child.try_wait();
                    }
                },
                _ => {}
            }
    }
}

pub fn run_action(config: &Config, name: String) {
    let action = config.actions.get(&name.to_string());
    match action {
        Some(action) => {
            if !action.disabled {
                run_command(action.command.to_string());
            }
        },
        None => panic!("No known action named \"{}\".", name)
    }
}

pub fn run_named_command(config: &Config, repo: String, command: &String) {
    match config.repositories.get(&repo) {
        Some(repository) => {
            if !repository.disabled {
                let location = match config.is_not_default {
                    true => {
                        let thing = config.base_path.join(Path::new(&repository.location.to_string()));
                        String::from(thing.to_str().unwrap())
                    },
                    _ => repository.location.clone()
                };
                if Path::new(&location).exists() {
                    let mut options: HashMap<&str, &str> = HashMap::new();
                    for (key, value) in &repository.options {
                        options.insert(key, value);
                    }
                    options.insert("location", &location);
                    match config.repo_types.get(&repository.repo_type) {
                        Some(repo_type) => {
                            match repo_type.commands.get(command) {
                                Some(command_string) => {
                                    println!("\n\nRepository {} ({}):", repo, location);
                                    run_command_in_directory(location.to_string(),
                                                             Template::new(&command_string).render(&options));
                                },
                                None => {}
                            }
                        },
                        None => panic!("No repository type named \"{}\".", repository.repo_type)
                    }
                }
            }
        }
        None => panic!("No known repository named \"{}\".", repo)
    }
}

pub fn run_repository_sync(config: &Config, name: String) {
    let repository = config.repositories.get(&name.to_string());
    match repository {
        Some(repository) => {
            if !repository.disabled {
                let location = match config.is_not_default {
                    true => {
                        let thing = config.base_path.join(Path::new(&repository.location.to_string()));
                        String::from(thing.to_str().unwrap())
                    },
                    _ => repository.location.clone()
                };
                if !Path::new(&location).exists() {
                    if repository.auto_create {
                        run_repository_creation(config, name);
                    }
                } else {
                    let mut options: HashMap<&str, &str> = HashMap::new();
                    for (key, value) in &repository.options {
                        options.insert(key, value);
                    }
                    options.insert("location", &location);
                    let repo_type = config.repo_types.get(&repository.repo_type);
                    match repo_type {
                        Some(repo_type) => {
                            println!("\n\nRepository {} ({}):", name, location);
                            run_command_in_directory(location.to_string(),
                                                     Template::new(&repo_type.pre_inward).render(&options));
                            run_command_in_directory(location.to_string(),
                                                     Template::new(&repo_type.inward).render(&options));
                            run_command_in_directory(location.to_string(),
                                                     Template::new(&repo_type.post_inward).render(&options));
                            run_command_in_directory(location.to_string(),
                                                     Template::new(&repo_type.outward).render(&options));
                            run_command_in_directory(location.to_string(),
                                                     Template::new(&repo_type.post_outward).render(&options));
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
                        run_command(Template::new(&repository_type.create).render(&options));
                    }
                },
                None => panic!("No known repository type named \"{}\".", repository_type_name)
            }
        },
        None => panic!("No known repository named \"{}\".", name)
    }
}

pub fn run_group(config: &Config, name: String) {
    let group = config.groups.get(&name.to_string());
    match group {
        Some(group) => {
            for member in &group.members {
                if config.repositories.contains_key(member) {
                    run_repository_sync(&config, member.to_string());
                } else if config.groups.contains_key(member) {
                    run_group(&config, member.to_string());
                } else {
                    println!("\"{}\" is neither a group nor a repository.", member);
                }
            }
            for action in &group.actions_after {
                run_action(&config, action.to_string());
            }
        },
        None => panic!("No known group named \"{}\".", name)
    }
}


pub fn run_group_with_command(config: &Config, name: String, command: &String) {
    let group = config.groups.get(&name.to_string());
    match group {
        Some(group) => {
            for member in &group.members {
                if config.repositories.contains_key(member) {
                    run_named_command(&config, member.to_string(), &command);
                } else if config.groups.contains_key(member) {
                    run_group_with_command(&config, member.to_string(), &command);
                } else {
                    println!("\"{}\" is neither a group nor a repository.", member);
                }
            }
            for action in &group.actions_after {
                run_action(&config, action.to_string());
            }
        },
        None => panic!("No known group named \"{}\".", name)
    }
}
