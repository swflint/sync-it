#[macro_use]
extern crate clap;
use clap::App;

mod run;
use run::run;

mod config;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("run") => if let Some(matches) = matches.subcommand_matches("run") {
            let names = matches.values_of("name").unwrap();
            // names.mambo
            // println!();
            run(names);
            // for thing in names {}
        },
        _ => println!("Nothing")
    }
    
}
