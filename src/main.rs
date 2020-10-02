#[macro_use]
extern crate clap;
use clap::App;

use std::env;

mod lib;

use crate::lib::{
    config::{
        find_config_file,
        read_configuration_file,
        write_configuration_file,
        Config
    },
    repository,
    action,
    group
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config_file = find_config_file(matches.value_of("config"));
    let mut configuration: Config = read_configuration_file(&config_file);

    match matches.subcommand_name() {
        Some("run") => println!("Running..."),
        Some("repository") => if let Some(matches) = matches.subcommand_matches("repository") {
            match matches.subcommand_name() {
                Some("register") => if let Some(matches) = matches.subcommand_matches("register") {
                    let type_name = matches.value_of("type").unwrap().to_string();
                    let name = matches.value_of("name").unwrap().to_string();
                    let option_strings_in: Vec<&str> = matches.values_of("options").unwrap().collect();
                    let mut option_strings: Vec<String> = Vec::new();
                    for str_thing in option_strings_in {
                        option_strings.push(str_thing.to_string())
                    }
                    let location = env::current_dir().unwrap().to_str().unwrap().to_string();
                    repository::register(&mut configuration, &name, location, type_name, option_strings);
                },
                Some("config") => if let Some(matches) = matches.subcommand_matches("config") {
                    let name = matches.value_of("name").unwrap().to_string();
                    if let Some(options) = matches.values_of("options") {
                        let mut option_strings: Vec<String> = Vec::new();
                        for str_thing in options {
                            option_strings.push(str_thing.to_string())
                        }
                        repository::update_options(&mut configuration, &name, option_strings);
                    }
                    match matches.value_of("autocreate") {
                        Some("YES") => repository::update_autocreate(&mut configuration, &name, true),
                        Some("NO") => repository::update_autocreate(&mut configuration, &name, false),
                        _ => {}
                    }
                    match matches.value_of("disable") {
                        Some("YES") => repository::update_disabled(&mut configuration, &name, true),
                        Some("NO") => repository::update_disabled(&mut configuration, &name, false),
                        _ => {}
                    }
                },
                Some("show") => if let Some(matches) = matches.subcommand_matches("show") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let repository = configuration.repositories.get(&name);
                    match repository {
                        Some(repository) => println!("{}", repository),
                        None => eprintln!("No known repository named \"{}\".", name)
                    }
                }
                _ => panic!("Something has gone horribly wrong...")
            }
        },
        Some("action") => if let Some(matches) = matches.subcommand_matches("action") {
            match matches.subcommand_name() {
                Some("create") => if let Some(matches) = matches.subcommand_matches("create") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let command = matches.value_of("command").unwrap().to_string();
                    let description = match matches.value_of("description") {
                        Some(string) => string.to_string(),
                        _ => String::from("")
                    };
                    action::add(&mut configuration, &name, &command, &description);
                },
                Some("config") => if let Some(matches) = matches.subcommand_matches("config") {
                    let name = matches.value_of("name").unwrap().to_string();
                    match matches.value_of("disabled") {
                        Some("YES") => action::update_disabled(&mut configuration, &name, true),
                        Some("NO") => action::update_disabled(&mut configuration, &name, false),
                        _ => {}
                    }
                    match matches.value_of("command") {
                        Some(command) => action::update_command(&mut configuration, &name, &command.to_string()),
                        _ => {}
                    }
                    match matches.value_of("description") {
                        Some(description) => action::update_description(&mut configuration, &name, &description.to_string()),
                        _ => {}
                    }
                },
                Some("show") => if let Some(matches) = matches.subcommand_matches("show") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let action = configuration.actions.get(&name);
                    match action {
                        Some(action) => println!("{}", action),
                        None => eprintln!("No known action named \"{}\".", name)
                    }
                },
                _ => panic!("Something has gone horribly wrong...")
            }
        },
        Some("group") => if let Some(matches) = matches.subcommand_matches("group") {
            match matches.subcommand_name() {
                Some("create") => if let Some(matches) = matches.subcommand_matches("create") {
                    let name = matches.value_of("name").unwrap().to_string();
                    group::add(&mut configuration, &name);
                },
                Some("add") => if let Some(matches) = matches.subcommand_matches("add") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let repo = matches.value_of("repo").unwrap().to_string();
                    group::add_repo(&mut configuration, &name, &repo);
                },
                Some("act") => if let Some(matches) = matches.subcommand_matches("act") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let action = matches.value_of("action").unwrap().to_string();
                    group::add_action(&mut configuration, &name, &action);
                },
                Some("remove") => if let Some(matches) = matches.subcommand_matches("remove") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let repo = matches.value_of("repo").unwrap().to_string();
                    group::remove_repo(&mut configuration, &name, &repo);
                },
                Some("show") => if let Some(matches) = matches.subcommand_matches("show") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let group = configuration.groups.get(&name);
                    match group {
                        Some(group) => println!("{}", group),
                        None => eprintln!("No known group named \"{}\".", name)
                    }
                },
                _ => panic!("Something has gone horribly wrong...")
            }
        },
        Some(thing) => println!("{}", thing),
        _ => println!("No subcommand."),
    }

    match write_configuration_file(config_file, configuration) {
        Err(err) => panic!("Error writing configuration: {}.", err),
        _ => {}
    }
}
