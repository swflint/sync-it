#[macro_use]
extern crate clap;
use clap::App;

mod config;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

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
}
