[![REUSE status](https://api.reuse.software/badge/git.sr.ht/~swflint/sync-it)](https://api.reuse.software/info/git.sr.ht/~swflint/sync-it)

# sync-it

`sync-it` is a command-line tool written in Rust to help easily synchronize repositories.  It allows grouping of repositories, definition of different kinds of repository, and definition of pre-/post-group actions.

## Usage

### `type`

This subcommand is used to create and manage repository types.

### `repository`

This subcommand is used to create and manage repositories.

### `group`

This subcommand is used to create and manage groups of repositories.

### `action`

This subcommand is used to create and manage actions.

### `run`

This subcommand runs a defined command or synchronization procedure for a repository, group, or some combination.  The `-c` argument may be used to specify a defined command instead of the default "synchronization" sequence.  The `all` "group" will run all repositories.

### `completion`

The subcommand `sync-it completion` takes a single argument, the name of a shell (`bash`, `elvish`, `fish`, `powershell`, and `zsh` are currently supported by [`clap_completion`](https://docs.rs/clap_complete/latest/clap_complete/), and outputs a completion definition file to STDOUT.
