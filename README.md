# CLI Task Tracker

The following rust project utilizes two main important
technologies, rust and surrealDB. I have been learning
Rust and SurrealDB recently and when I came across
this project on [Roadmap Sh](https://roadmap.sh) it just
rang back to my new knowledge. So lets pu it into test

## Requirements

- [Rust](https://rustup.rs) `1.80.1`
- [SurrealDB](https://surrealdb.com/docs/surrealdb/installation) `1.5.4`

## Usage

After installing rust and surrealdb and confirmed that they
installed correctly

- In the terminal start the surreal instance by running:
    - On linux
      - `surreal start memory -A --user root --pass root`
    - On windows
      - `surreal.exe start memory -A --user root --pass root`

- Clone this repository
  - `https://github.com/rollingghost/cli-task-tracker.git`

- After cloning navigate to the directory and run
  - `cargo run <options>` or use `cargo run --help` for help

## Supported commands

- `add <task>` creates new task and by default it is marked as todo
- `stage <task>` puts the task into progress
- `update <task>` updates a task
- `delete <task>` deletes a task
- `list all` lists all tasks
- `list staged` lists tasks that are marked as in progress
- `list done` lists tasks that are marked as done
- `list todo` lists task that are marked as todo


## Contributions

Feel free to create an issue or submit a pull request