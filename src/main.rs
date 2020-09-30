#[macro_use]
extern crate clap;
use clap::App;

use std::env;

mod config;
use config::{
    find_config_file,
    read_configuration_file,
    write_configuration_file,
    Config
};

mod repository;

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
        }
        Some(thing) => println!("{}", thing),
        _ => println!("No subcommand."),
    }

    write_configuration_file(config_file, configuration);
}
