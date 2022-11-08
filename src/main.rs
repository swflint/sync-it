// SPDX-FileCopyrightText: 2021 - 2022 Samuel W. Flint <swflint@flintfam.org>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Command, command, Arg, value_parser, ArgAction, ValueEnum, builder::PossibleValue};
use clap_complete::{generate, Generator, Shell};

use std::env;
use std::io;

mod lib;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HumanBool {
    Yes,
    No
}

impl ValueEnum for HumanBool {
    fn value_variants<'a>() -> &'a [Self] {
        &[HumanBool::Yes, HumanBool::No]
    }
    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            HumanBool::Yes => PossibleValue::new("YES"),
            HumanBool::No => PossibleValue::new("NO")
        })
    }
}

impl std::fmt::Display for HumanBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("Values cannot be skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for HumanBool {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant)
            }
        }
        Err(format!("Invalid Variant: {}", s))
    }
}


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


fn build_cli() -> Command {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .author("Samuel W. Flint <swflint@flintfam.org>")
	.after_help("License under the GNU GPL v3.0 or later (https://spdx.org/licenses/GPL-3.0-or-later.html)")
	.after_long_help("License under the GNU GPL v3.0 or later (https://spdx.org/licenses/GPL-3.0-or-later.html)")
        .about("Synchronize directories flexibly")
        .arg(Arg::new("config")
             .short('c')
             .long("config")
             .value_name("FILE")
             .help("Set a custom configuration file"))
        .subcommand(Command::new("run")
                    .aliases(["sync", "rr"])
                    .about("Run synchronization or command for repositories and groups.")
                    .arg(Arg::new("repo")
                         .action(ArgAction::Append)
                         .value_name("REPO_OR_GROUP")
                         .help("Name or names of repositories/groups to sync"))
                    .arg(Arg::new("command")
                         .short('C')
                         .long("command")
                         .value_name("COMMAND")
                         .help("Run named COMMAND in each specified repository")))
        .subcommand(Command::new("repository")
                    .about("Create and manage repositories")
                    .visible_aliases(["repo", "r"])
                    .subcommand_required(true)
                    .subcommand(Command::new("list")
                                .about("List repositories"))
                    .subcommand(Command::new("register")
                                .about("Register the current directory as a repository")
                                .arg(Arg::new("type")
                                     .required(true)
                                     .value_name("TYPE")
                                     .help("Type of repository"))
                                .arg(Arg::new("repo")
                                     .long("name")
                                     .short('n')
                                     .value_name("REPO")
                                     .help("Name of repository"))
                                .arg(Arg::new("options")
                                     .action(ArgAction::Append)
                                     .value_name("OPTION=VALUE")
                                     .help("Type-specific options, in option=value form")))
                    .subcommand(Command::new("config")
                                .about("Configure repository")
                                .arg(Arg::new("repo")
                                     .value_name("REPO")
                                     .required(true)
                                     .help("Name of repository to configure"))
                                .arg(Arg::new("autocreate")
                                     .short('a')
                                     .long("autocreate")
                                     .value_name("YES/NO")
                                     .help("Set autocreation")
                                     .value_parser(value_parser!(HumanBool)))
                                .arg(Arg::new("disable")
                                     .short('D')
                                     .long("disable")
                                     .value_name("YES/NO")
                                     .help("Disable repository")
                                     .value_parser(value_parser!(HumanBool)))
                                .arg(Arg::new("options")
                                     .action(ArgAction::Append)
                                     .value_name("OPTION=VALUE")
                                     .help("Type-specific options, in option=value form")))
                    .subcommand(Command::new("remove")
                                .visible_aliases(["rm"])
                                .about("Remove a repository")
                                .arg(Arg::new("repo")
                                     .help("Name of repository")
                                     .value_name("REPO")
                                     .required(true)))
                    .subcommand(Command::new("show")
                                .visible_aliases(["describe"])
                                .about("Show information about a repository")
                                .arg(Arg::new("repo")
                                     .help("Name of repository")
                                     .value_name("REPO")
                                     .required(true))))
        .subcommand(Command::new("group")
                    .about("Create and manage groups of repositories")
                    .subcommand_required(true)
                    .subcommand(Command::new("create")
                                .about("Create a group")
                                .arg(Arg::new("group")
                                     .help("Name of group")
                                     .required(true)
                                     .value_name("GROUP")))
                    .subcommand(Command::new("delete")
                                .visible_aliases(["drop"])
                                .about("Delete a group.")
                                .arg(Arg::new("group")
                                     .help("Name of group")
                                     .required(true)
                                     .value_name("GROUP")))
                    .subcommand(Command::new("add")
                                .about("Add a repo to a group")
                                .arg(Arg::new("group")
                                     .help("Name of group")
                                     .required(true)
                                     .value_name("GROUP"))
                                .arg(Arg::new("repo")
                                     .value_name("NAME")
                                     .required(true)
                                     .help("Name of repository"))
                    )
                    .subcommand(Command::new("act")
                                .about("Add an action to a group")
                                .arg(Arg::new("group")
                                     .help("Name of group")
                                     .required(true)
                                     .value_name("GROUP"))
                                .arg(Arg::new("action")
                                     .help("Name of action")
                                     .required(true)
                                     .value_name("ACTION")))
                    .subcommand(Command::new("remove")
                                .about("Remove a repo from a group")
                                .arg(Arg::new("group")
                                     .help("Name of group")
                                     .required(true)
                                     .value_name("GROUP"))
                                .arg(Arg::new("action")
                                     .help("Name of action")
                                     .required(true)
                                     .value_name("ACTION")))
                    .subcommand(Command::new("show")
                                .about("Show information about a group")
                                .arg(Arg::new("group")
                                     .help("Name of group")
                                     .required(true)
                                     .value_name("GROUP")))
                    .subcommand(Command::new("list")
                                .about("List known groups")))
        .subcommand(Command::new("type")
                    .about("Create and manage repository types")
                    .subcommand_required(true)
                    .subcommand(Command::new("create")
                                .about("Create a new repository type")
                                .arg(Arg::new("type")
                                     .help("Name of type")
                                     .required(true)
                                     .value_name("TYPE"))
                                .arg(Arg::new("description")
                                     .short('d')
                                     .long("description")
                                     .help("Description of repository type")
                                     .value_name("DESCRIPTION"))
                                .arg(Arg::new("create")
                                     .short('c')
                                     .long("create")
                                     .help("Creation command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("inward")
                                     .short('i')
                                     .long("inward")
                                     .help("Inward sync command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("outward")
                                     .short('o')
                                     .long("outward")
                                     .help("Outward sync command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("status")
                                     .short('s')
                                     .long("status")
                                     .help("Status command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("pre_inward")
                                     .long("pre-inward")
                                     .help("Pre-inward command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("post_inward")
                                     .long("post-inward")
                                     .help("Post-inward command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("post_outward")
                                     .long("post-outward")
                                     .help("Post-outward command")
                                     .value_name("COMMAND"))
                    )
                    .subcommand(Command::new("config")
                                .about("Configure a repository type")
                                .arg(Arg::new("type")
                                     .help("Name of type")
                                     .required(true)
                                     .value_name("TYPE"))
                                .arg(Arg::new("description")
                                     .short('d')
                                     .long("description")
                                     .help("Description of repository type")
                                     .value_name("DESCRIPTION"))
                                .arg(Arg::new("create")
                                     .short('c')
                                     .long("create")
                                     .help("Creation command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("inward")
                                     .short('i')
                                     .long("inward")
                                     .help("Inward sync command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("outward")
                                     .short('o')
                                     .long("outward")
                                     .help("Outward sync command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("status")
                                     .short('s')
                                     .long("status")
                                     .help("Status command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("pre_inward")
                                     .long("pre-inward")
                                     .help("Pre-inward command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("post_inward")
                                     .long("post-inward")
                                     .help("Post-inward command")
                                     .value_name("COMMAND"))
                                .arg(Arg::new("post_outward")
                                     .long("post-outward")
                                     .help("Post-outward command")
                                     .value_name("COMMAND")))
                    .subcommand(Command::new("command")
                                .about("Manage commands in a repository type")
                                .subcommand_required(true)
                                .subcommand(Command::new("add")
                                            .about("Add a command to a repository type")
                                            .arg(Arg::new("type")
                                                 .help("Name of type")
                                                 .required(true)
                                                 .value_name("TYPE"))
                                            .arg(Arg::new("name")
                                                 .help("Name of command")
                                                 .required(true)
                                                 .value_name("NAME"))
                                            .arg(Arg::new("command")
                                                 .help("Command")
                                                 .required(true)
                                                 .value_name("COMMAND")))
                                .subcommand(Command::new("change")
                                            .about("Change a command in a repository type")
                                            .arg(Arg::new("type")
                                                 .help("Name of type")
                                                 .required(true)
                                                 .value_name("TYPE"))
                                            .arg(Arg::new("name")
                                                 .help("Name of command")
                                                 .required(true)
                                                 .value_name("NAME"))
                                            .arg(Arg::new("command")
                                                 .help("Command")
                                                 .required(true)
                                                 .value_name("COMMAND")))
                                .subcommand(Command::new("remove")
                                            .about("Remove a command from a repository type")
                                            .arg(Arg::new("type")
                                                 .help("Name of type")
                                                 .required(true)
                                                 .value_name("TYPE"))
                                            .arg(Arg::new("name")
                                                 .help("Name of command")
                                                 .required(true)
                                                 .value_name("NAME")))
                    )
                    .subcommand(Command::new("show")
                                .about("Show information about a repository type")
                                .arg(Arg::new("type")
                                     .help("Name of type")
                                     .required(true)
                                     .value_name("TYPE")))
                    .subcommand(Command::new("list")
                                .about("List known repository types")))
        .subcommand(Command::new("action")
                    .about("Create and manage actions")
                    .subcommand_required(true)
                    .subcommand(Command::new("create")
                                .about("Create a new action")
                                .arg(Arg::new("action")
                                     .help("Name of action")
                                     .required(true)
                                     .value_name("ACTION"))
                                .arg(Arg::new("command")
                                     .help("Command")
                                     .required(true)
                                     .value_name("COMMAND"))
                                .arg(Arg::new("description")
                                     .help("Description of action")
                                     .long("description")
                                     .short('d')
                                     .value_name("DESCRIPTION")))
                    .subcommand(Command::new("config")
                                .about("Configure an action")
                                .arg(Arg::new("action")
                                     .help("Name of action")
                                     .required(true)
                                     .value_name("ACTION"))
                                .arg(Arg::new("disable")
                                     .short('D')
                                     .long("disable")
                                     .value_name("YES/NO")
                                     .help("Disable action")
                                     .value_parser(value_parser!(HumanBool)))
                                .arg(Arg::new("command")
                                     .help("Command")
                                     .long("command")
                                     .short('c')
                                     .value_name("COMMAND"))
                                .arg(Arg::new("description")
                                     .help("Description of action")
                                     .long("description")
                                     .short('d')
                                     .value_name("DESCRIPTION")))
                    .subcommand(Command::new("show")
                                .about("Show information about an action")
                                .arg(Arg::new("action")
                                     .help("Name of action")
                                     .required(true)
                                     .value_name("ACTION")))
                    .subcommand(Command::new("list")
                                .about("List known actions")))
        .subcommand(Command::new("completion")
                    .about("Generate completions for command.")
                    .arg(Arg::new("shell")
                         .value_name("SHELL")
                         .help("Which shell to generate completions for")
                         .required(true)
                         .value_parser(value_parser!(Shell))))
}


fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}


fn main() {

    let matches = build_cli().get_matches();

    let config_file = find_config_file(matches.get_one::<String>("config"));
    let mut configuration: Config = read_configuration_file(&config_file);
    if matches.get_one::<String>("config").is_some() {
        configuration.is_not_default = true;
        configuration.base_path = config_file.canonicalize().unwrap().parent().unwrap().to_path_buf();
    }

    match matches.subcommand() {
        Some(("completion", subm)) => {
            let mut cmd = build_cli();
            if let Some(generator) = subm.get_one::<Shell>("shell").copied() {
                print_completions(generator, &mut cmd);
            }
        }
        Some(("run", subm)) => {
            let repos: Vec<&str> = subm.get_many::<String>("repo")
                .expect("At least one repository/group must be specified.")
                .map(|s| s.as_str()).collect();

            if let Some(command) = subm.get_one::<String>("command") {
                run::run_with_command(&configuration, command, repos);
            } else {
                run::run(&configuration, repos);
            }
        }
        Some(("repository", subm)) => {
            match subm.subcommand() {
                Some(("register", subm)) => {
                    let type_name = subm.get_one::<String>("type").expect("A type name must be provided").to_string();
                    let location = match configuration.is_not_default {
                        true => env::current_dir().unwrap().strip_prefix(&configuration.base_path).unwrap().to_path_buf(),
                        _ => env::current_dir().unwrap()
                    };
                    let location_str = location.to_str().unwrap().to_string();
                    let name = match subm.get_one::<String>("name") {
                        Some(name) => name.to_string(),
                        None => location.file_name().unwrap().to_str().unwrap().to_string()
                    };
                    let mut option_strings: Vec<String> = Vec::new();
                    match subm.get_many::<String>("options") {
                        Some(option) => {
                            for string in option {
                                option_strings.push(string.to_string())
                            }
                        }
                        None => {}
                    }
                    repository::register(&mut configuration, &name, location_str, type_name, option_strings);
                }
                Some(("config", subm)) => {
                    let repo = subm.get_one::<String>("repo").expect("A repository name must be provided").to_string();
                    if let Some(options) = subm.get_many::<String>("options") {
                        let mut option_strings: Vec<String> = Vec::new();
                        for str_thing in options {
                            option_strings.push(str_thing.to_string())
                        }
                        repository::update_options(&mut configuration, &repo, option_strings);
                    }
                    match subm.get_one::<HumanBool>("autocreate") {
                        Some(HumanBool::Yes) => repository::update_autocreate(&mut configuration, &repo, true),
                        Some(HumanBool::No) => repository::update_autocreate(&mut configuration, &repo, false),
                        _ => {}
                    }
                    match subm.get_one::<HumanBool>("disable") {
                        Some(HumanBool::Yes) => repository::update_disabled(&mut configuration, &repo, true),
                        Some(HumanBool::No) => repository::update_disabled(&mut configuration, &repo, false),
                        _ => {}
                    }
                }
                Some(("remove", subm)) => {
                    let repo = subm.get_one::<String>("repo").expect("A repository name must be provided").to_string();
                    repository::remove_repo(&mut configuration, &repo);
                }
                Some(("list", _subm)) => {
                    for key in configuration.repositories.keys() {
                        println!(" - {}", key);
                    }
                }
                Some(("show", subm)) => {
                    let repo = subm.get_one::<String>("repo").expect("A repository name must be provided").to_string();
                    let repository = configuration.repositories.get(&repo);
                    match repository {
                        Some(repository) => println!("{}", repository),
                        None => eprintln!("No known repository named \"{}\".", repo)
                    }
                }
                _ => {
                    panic!("This should never happen...")
                }
            }
        }
        Some(("group", subm)) => {
            match subm.subcommand() {
                Some(("create", subm)) => {
                    let group = subm.get_one::<String>("group").expect("A group name must be provided.").to_string();
                    group::add(&mut configuration, &group);
                }
                Some(("delete", subm)) => {
                    let group = subm.get_one::<String>("group").expect("A group name must be provided.").to_string();
                    group::remove_group(&mut configuration, &group);
                }
                Some(("add", subm)) => {
                    let group = subm.get_one::<String>("group").expect("A group name must be provided.").to_string();
                    let repo = subm.get_one::<String>("repo").expect("A repository name must be provided.").to_string();
                    group::add_repo(&mut configuration, &group, &repo)
                }
                Some(("act", subm)) => {
                    let group = subm.get_one::<String>("group").expect("A group name must be provided.").to_string();
                    let action = subm.get_one::<String>("action").expect("An action name must be provided").to_string();
                    group::add_action(&mut configuration, &group, &action);
                }
                Some(("remove", subm)) => {
                    let group = subm.get_one::<String>("group").expect("A group name must be provided.").to_string();
                    let repo = subm.get_one::<String>("repo").expect("A repository name must be provided.").to_string();
                    group::remove_repo(&mut configuration, &group, &repo)
                }
                Some(("show", subm)) => {
                    let group_name = subm.get_one::<String>("group").expect("A group name must be provided.").to_string();
                    let group = configuration.groups.get(&group_name);
                    match group {
                        Some(group) => println!("{}", group),
                        None => eprintln!("No known group named \"{}\".", group_name)
                    }
                }
                Some(("list", _subm)) => {
                    for key in configuration.groups.keys() {
                        println!(" - {}", key);
                    }
                }
                _ => {
                    panic!("This should never happen...")
                }
            }
        }
        Some(("type", subm)) => {
            match subm.subcommand() {
                Some(("create", subm)) => {
                    let tname = subm.get_one::<String>("type").expect("A type name must be provided").to_string();
                    let temp_string = "".to_string();
                    let description = subm.get_one::<String>("description").unwrap_or(&temp_string);
                    let create = subm.get_one::<String>("create").unwrap_or(&temp_string);
                    let inward = subm.get_one::<String>("inward").unwrap_or(&temp_string);
                    let outward = subm.get_one::<String>("outward").unwrap_or(&temp_string);
                    let status = subm.get_one::<String>("status").unwrap_or(&temp_string);
                    let pre_inward = subm.get_one::<String>("pre_inward").unwrap_or(&temp_string);
                    let post_inward = subm.get_one::<String>("post_inward").unwrap_or(&temp_string);
                    let post_outward = subm.get_one::<String>("post_outward").unwrap_or(&temp_string);
                    repotype::add(&mut configuration, &tname, &description, &create, &inward, &outward, &status, &pre_inward, &post_inward, &post_outward);
                }
                Some(("config", subm)) => {
                    let tname = subm.get_one::<String>("type").expect("A type name must be provided").to_string();
                    match subm.get_one::<String>("description") {
                        Some(description) => repotype::update_description(&mut configuration, &tname, &description.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("create") {
                        Some(create) => repotype::update_create(&mut configuration, &tname, &create.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("inward") {
                        Some(inward) => repotype::update_inward(&mut configuration, &tname, &inward.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("outward") {
                        Some(outward) => repotype::update_outward(&mut configuration, &tname, &outward.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("status") {
                        Some(status) => repotype::update_status(&mut configuration, &tname, &status.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("pre_inward") {
                        Some(pre_inward) => repotype::update_pre_inward(&mut configuration, &tname, &pre_inward.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("post_inward") {
                        Some(post_inward) => repotype::update_post_inward(&mut configuration, &tname, &post_inward.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("post_outward") {
                        Some(post_outward) => repotype::update_post_outward(&mut configuration, &tname, &post_outward.to_string()),
                        _ => {}
                    }
                }
                Some(("command", subm)) => {
                    match subm.subcommand() {
                        Some(("add", subm)) => {
                            let type_name = subm.get_one::<String>("type").expect("A type name is required").to_string();
                            let name = subm.get_one::<String>("name").expect("A name is required").to_string();
                            let command = subm.get_one::<String>("command").expect("A command is required").to_string();
                            repotype::add_command(&mut configuration, &type_name, &name, &command);
                        },
                        Some(("change", subm)) => {
                            let type_name = subm.get_one::<String>("type").expect("A type name is required").to_string();
                            let name = subm.get_one::<String>("name").expect("A name is required").to_string();
                            let command = subm.get_one::<String>("command").expect("A command is required").to_string();
                            repotype::change_command(&mut configuration, &type_name, &name, &command);
                        },
                        Some(("remove", subm)) => {
                            let type_name = subm.get_one::<String>("type").expect("A type name is required").to_string();
                            let name = subm.get_one::<String>("name").expect("A name is required").to_string();
                            repotype::remove_command(&mut configuration, &type_name, &name);
                        },
                        _ => panic!("Something has gone horribly wrong...")
                    }
                }
                Some(("show", subm)) => {
                    let tname = subm.get_one::<String>("type").expect("A type name is required").to_string();
                    let repo_type = configuration.repo_types.get(&tname);
                    match repo_type {
                        Some(repo_type) => println!("{}", repo_type),
                        None => eprintln!("No known repo type named \"{}\".", tname)
                    }
                }
                Some(("list", _subm)) => {
                    for key in configuration.repo_types.keys() {
                        println!(" - {}", key);
                    }
                }
                _ => {
                    panic!("This should never happen...")
                }
            }
        }
        Some(("action", subm)) => {
            match subm.subcommand() {
                Some(("create", subm)) => {
                    let name = subm.get_one::<String>("action").expect("An action name is required").to_string();
                    let command = subm.get_one::<String>("command").expect("A command is required").to_string();
                    let temp_string = "".to_string();
                    let description = subm.get_one::<String>("Description").unwrap_or(&temp_string);
                    action::add(&mut configuration, &name, &description, &command);
                }
                Some(("config", subm)) => {
                    let name = subm.get_one::<String>("action").expect("An action name is required").to_string();
                    match subm.get_one::<HumanBool>("disabled") {
                        Some(HumanBool::Yes) => action::update_disabled(&mut configuration, &name, true),
                        Some(HumanBool::No) => action::update_disabled(&mut configuration, &name, false),
                        _ => {}
                    }
                    match subm.get_one::<String>("command") {
                        Some(command) => action::update_command(&mut configuration, &name, &command.to_string()),
                        _ => {}
                    }
                    match subm.get_one::<String>("description") {
                        Some(description) => action::update_description(&mut configuration, &name, &description.to_string()),
                        _ => {}
                    }
                }
                Some(("show", subm)) => {
                    let name = subm.get_one::<String>("action").expect("An action name is required").to_string();
                    let action = configuration.actions.get(&name);
                    match action {
                        Some(action) => println!("{}", action),
                        None => eprintln!("No known action named \"{}\".", name)
                    }
                }
                Some(("list", _subm)) => {
                    for key in configuration.actions.keys() {
                        println!(" - {}", key);
                    }
                }
                _ => {
                    panic!("This should never happen...")
                }
            }
        }
        _ => {
            panic!("This should never happen...")
        }
    }

    
    match write_configuration_file(config_file, configuration) {
        Err(err) => panic!("Error writing configuration: {}.", err),
        _ => {}
    }
}
