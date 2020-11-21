#[macro_use]
extern crate clap;
use clap::App;

use std::env;
use std::path::Path;

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
    group,
    repotype,
    run
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let config_file = find_config_file(matches.value_of("config"));
    let mut configuration: Config = read_configuration_file(&config_file);

    if matches.is_present("config") {
        configuration.is_not_default = true;
        configuration.base_path = Path::new(matches.value_of("config").unwrap()).canonicalize().unwrap().parent().unwrap().to_path_buf();
    }

    match matches.subcommand_name() {
        Some("run") => run::run(&configuration, matches.subcommand_matches("run").unwrap().values_of("name").unwrap()),
        Some("repository") => if let Some(matches) = matches.subcommand_matches("repository") {
            match matches.subcommand_name() {
                Some("register") => if let Some(matches) = matches.subcommand_matches("register") {
                    let type_name = matches.value_of("type").unwrap().to_string();
                    let location = match configuration.is_not_default {
                        true => env::current_dir().unwrap().strip_prefix(&configuration.base_path).unwrap().to_path_buf(),
                        _ => env::current_dir().unwrap(),
                    };
                    let location_string = location.to_str().unwrap().to_string();
                    let name = match matches.value_of("name") {
                        Some(string) => string.to_string(),
                        None => location.file_name().unwrap().to_str().unwrap().to_string()
                    };
                    let mut option_strings: Vec<String> = Vec::new();
                    match matches.values_of("options") {
                        Some(option_strings_in) => {
                            for str_thing in option_strings_in {
                                option_strings.push(str_thing.to_string())
                            }
                        },
                        None => {}
                    }
                    repository::register(&mut configuration, &name, location_string, type_name, option_strings);
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
                Some("remove") => if let Some(matches) = matches.subcommand_matches("remove") {
                    let name = matches.value_of("name").unwrap().to_string();
                    repository::remove_repo(&mut configuration, &name);
                },
                Some("show") => if let Some(matches) = matches.subcommand_matches("show") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let repository = configuration.repositories.get(&name);
                    match repository {
                        Some(repository) => println!("{}", repository),
                        None => eprintln!("No known repository named \"{}\".", name)
                    }
                },
                Some("list") => {
                    for key in configuration.repositories.keys() {
                        println!(" - {}", key);
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
                    action::add(&mut configuration, &name, &description, &command);
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
                Some("list") => {
                    for key in configuration.actions.keys() {
                        println!(" - {}", key);
                    }
                }
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
                Some("drop") => if let Some(matches) = matches.subcommand_matches("drop") {
                    let name = matches.value_of("name").unwrap().to_string();
                    group::remove_group(&mut configuration, &name);
                }
                Some("show") => if let Some(matches) = matches.subcommand_matches("show") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let group = configuration.groups.get(&name);
                    match group {
                        Some(group) => println!("{}", group),
                        None => eprintln!("No known group named \"{}\".", name)
                    }
                },
                Some("list") => {
                    for key in configuration.groups.keys() {
                        println!(" - {}", key);
                    }
                },
                _ => panic!("Something has gone horribly wrong...")
            }
        },
        Some("type") => if let Some(matches) = matches.subcommand_matches("type") {
            match matches.subcommand_name() {
                Some("create") => if let Some(matches) = matches.subcommand_matches("create") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let description = match matches.value_of("description") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    let create = match matches.value_of("create") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    let inward = match matches.value_of("inward") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    let outward = match matches.value_of("outward") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    let status = match matches.value_of("status") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    let pre_inward = match matches.value_of("pre_inward") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    let post_inward = match matches.value_of("post_inward") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    let post_outward = match matches.value_of("post_outward") {
                        Some(thing) => thing.to_string(),
                        None => "".to_string()
                    };
                    repotype::add(&mut configuration, &name, &description, &create, &inward, &outward, &status, &pre_inward, &post_inward, &post_outward);
                },
                Some("config") => if let Some(matches) = matches.subcommand_matches("config") {
                    let name = matches.value_of("name").unwrap().to_string();
                    match matches.value_of("description") {
                        Some(description) => repotype::update_description(&mut configuration, &name, &description.to_string()),
                        _ => {}
                    }
                    match matches.value_of("create") {
                        Some(create) => repotype::update_create(&mut configuration, &name, &create.to_string()),
                        _ => {}
                    }
                    match matches.value_of("inward") {
                        Some(inward) => repotype::update_inward(&mut configuration, &name, &inward.to_string()),
                        _ => {}
                    }
                    match matches.value_of("outward") {
                        Some(outward) => repotype::update_outward(&mut configuration, &name, &outward.to_string()),
                        _ => {}
                    }
                    match matches.value_of("status") {
                        Some(status) => repotype::update_status(&mut configuration, &name, &status.to_string()),
                        _ => {}
                    }
                    match matches.value_of("pre_inward") {
                        Some(pre_inward) => repotype::update_pre_inward(&mut configuration, &name, &pre_inward.to_string()),
                        _ => {}
                    }
                    match matches.value_of("post_inward") {
                        Some(post_inward) => repotype::update_post_inward(&mut configuration, &name, &post_inward.to_string()),
                        _ => {}
                    }
                    match matches.value_of("post_outward") {
                        Some(post_outward) => repotype::update_post_outward(&mut configuration, &name, &post_outward.to_string()),
                        _ => {}
                    }
                }
                Some("show") => if let Some(matches) = matches.subcommand_matches("show") {
                    let name = matches.value_of("name").unwrap().to_string();
                    let repo_type = configuration.repo_types.get(&name);
                    match repo_type {
                        Some(repo_type) => println!("{}", repo_type),
                        None => eprintln!("No known repo type named \"{}\".", name)
                    }
                },
                Some("list") => {
                    for key in configuration.repo_types.keys() {
                        println!(" - {}", key);
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
