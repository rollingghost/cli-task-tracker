# CLI Task Tracker

The following rust project utilizes two main important
technologies, rust and surrealDB. I have been learning
Rust and SurrealDB recently and when I came across
this project on [Roadmap Sh](https://roadmap.sh) it just
rang back to my new knowledge. So lets put it into test.

## Project

[CLI Task Tracker](https://roadmap.sh/projects/task-tracker)

## Release

[CLI Task Tracker](https://github.com/rollingghost/cli-task-tracker/releases/download/Productivity/cli-task-tracker)

## Requirements

- [Rust](https://rustup.rs) `1.80.1`

## Usage

After installing rust and confirmed that they
installed correctly

- Ensure rust is working properly by running
  - `rustc --version`
- After cloning navigate to the directory and run
  - `cargo run <options>` or use `cargo run --help` for help
- Or download the binary and run directly on the terminal
- If you are using linux you can move the binary to `/usr/bin`
  for a global effect

## Supported commands

- `add <task description>` creates new task and by default it is marked as todo
- `stage <task id>` puts the task into progress
- `update <task id description>` updates a task
- `done <task id>` marks a task as done
- `delete <task id>` deletes a task
- `list --status all` lists all tasks
- `list --status staged` lists tasks that are marked as in progress
- `list --status done` lists tasks that are marked as done
- `list --status todo` lists task that are marked as todo

## Contributions

Feel free to contribute to this project by forking and creating a pull request
