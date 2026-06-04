# `prej`

`prej` is a project workspace manager that lets you register projects and jump between them from anywhere.

## Installation

> [!WARNING]
> This project is in beta. There may be bugs, especially with installation.

### Support

 `prej` currently only supports:
  - OS:
    - macOS
  - Shells:
    - `zsh`

Now, with that out of the way, use this command to install `prej`:

```sh
$ curl -fsSL https://raw.githubusercontent.com/alexsvianzon/prej/refs/heads/beta/scripts/install.sh | sh
```

This script will install `prej` globally on your system and add it to your `$PATH`.

## Features

In its beta state, `prej` can:
 - Register a project with `prej add [project]` in the project directory
 - List all projects with `prej list`
 - Switch to a project with `prej go [project]` from anywhere
 - Remove a project with `prej rm [project]`

 ## Support

 `prej` currently only supports:
  - OS:
    - macOS
  - Shells:
    - `zsh`

## Planned Features

 - YAML task file to define tasks that get run when you switch or as needed
   - Run commands
   - Set environment variables
 - Daemon for background services
 - Expanded OS support
 - Expanded shell support
