#[macro_use]
extern crate clap;
use clap::App;

mod config;
use config::{
    find_config_file,
    read_configuration_file,
    write_configuration_file,
    Config
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config_file = find_config_file(matches.value_of("config"));
    let mut configuration: Config = read_configuration_file(&config_file);
    match matches.subcommand_name() {
        Some("run") => println!("Running..."), // if let Some(matches) = matches.subcommand_matches("run") {
        //     let names = matches.values_of("name").unwrap();
        //     // names.mambo
        //     // println!();
        //     run(names);
        //     // for thing in names {}
        // },
        Some(thing) => println!("{}", thing),
        _ => println!("No subcommand."),
    }

    write_configuration_file(config_file, configuration);
}
