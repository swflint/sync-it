#[macro_use]
// extern crate clap;
use clap::App;

pub fn build_cli() -> App<'static, 'static> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    app
}
