# CLI Task Tracker

The following rust project utilizes two main important
technologies, rust and surrealDB. I have been learning
Rust and SurrealDB recently and when I came across
this project on [Roadmap Sh](https://roadmap.sh) it just
rang back to my new knowledge. So lets pu it into test

## Requirements

- [Rust](https://rustup.rs) `1.80.1`

## Usage

After installing rust and surrealdb and confirmed that they
installed correctly

- After cloning navigate to the directory and run
  - `cargo run <options>` or use `cargo run --help` for help
- Or if you are working with linux, move the binary in `target/release`
  directory to `/usr/bin` to use the task tracker

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
