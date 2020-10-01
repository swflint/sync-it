use clap::Values;
use config::{
    Config,
    Repository
}

pub fn run(names: Values<'_>) {
    for name in names {
        println!("Name: {}", name)
    }
}
