# Gitbox

Gitbox is the spiritual successor and rewrite of [get](https://github.com/asperan/get).

It leverages the git CLI tool to enable fast and simple semantic versioning and conventional commits.

## Installation
As of now, the only way to install this software is by cloning the repository and installing it with cargo:
```
git clone <origin-URL> gitbox
cd gitbox
cargo install --path .
```

[Cargo installation instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Features
Gitbox contains multiple subcommands which execute different tasks.

The help message `gb help` (or `gb --help`):
```
Gitbox (gb) is wrapper for git and it enhance some functionalities.

Usage: gb <COMMAND>

Commands:
  changelog  Generate a changelog
  commit     Create a commit with a conventional message
  complete   Print a completion script
  describe   Calculate the next version
  init       Initialize a git repository
  license    Create a license file
  tree       Print a fancy view of the commit tree
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Help
This subcommand will print help messages for the other subcommand specified:
```
gb help <subcommand>
```

It is equivalent to `gb <subcommand> --help` or `gb <subcommand> -h`.

### Changelog
This subcommand generates a list of changes from the last release (or the last version, if a flag is enabled).

The list can be formatted using options (see `gb changelog --help`).

### Commit
`gb commit` allows to easily create a commit which follows the conventional commit standard.

If no option is specified, it will proceed by asking to the user the commit type, its scope, whether it is a breaking change, the summary and the body of the commit. With options, these questions can be skipped (by providing a value).

For a simple commit (not breaking and without body), the command suggested is `gb commit --no-breaking -m ''`.

For the complete list of options, run `gb help commit` (or `gb commit --help`).

### Complete
This subcommand prints a completion script for Gitbox. Follow the instruction of your shell (if supported) to install it.

### Describe
`gb describe` calculates the new semantic version from the list of commits since the last release.

You can configure the triggers for a specific change (i.e. an update to a core dependency, if properly configured with type and scope, can trigger a patch update).

For all configuration options, see `gb help describe`.

There is also a `docker` subcommand which prints a list of tag versions for docker.

### Init
This simple subcommand initialize a git repository with an empty commit, to allow the early usage of the other subcommands.

### License
The `license` subcommand retrieve the list of opensource licenses from https://choosealicense.com/ and allow the user to download the chosen license text.

### Tree
`gb tree` pretty prints the output of `git log`. To navigate it you can pipe its output to a pager: `gb tree | less`.

## License
This software is distributed according to the [MIT license](https://mit-license.org/).

