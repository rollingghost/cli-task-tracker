use clap::Parser;
use cli_task_tracker::system::{cpu_usage, hello_cpu, sys_summary};
use cli_task_tracker::task::{
    list_tasks, load_json_tasks, pretty_table, save, search_task, show_commands, AllTasks, Task,
    TaskStatus, TaskStruct,
};
use cli_task_tracker::tui_me::my_tui;
use cli_task_tracker::{CliTracker, Commands};
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdin, stdout, Write};
use std::time::SystemTime;
use uuid::Uuid;

fn main() {
    // Clear terminal
    print!("{}", Clear(ClearType::All));
    let _ = my_tui();
    // Welcome message
    println!(
        r#"
        __        __   _
        \ \      / /__| | ___ ___  _ __ ___   ___
         \ \ /\ / / _ \ |/ __/ _ \| '_ ` _ \ / _ \
          \ V  V /  __/ | (_| (_) | | | | | |  __/
           \_/\_/ \___|_|\___\___/|_| |_| |_|\___|
        "#,
    );
    println!("\t\tWelcome to CLITracker!\n");

    let mut temp_tasks = Vec::new();

    let mut all_tasks = AllTasks {
        tmp_tasks: &mut temp_tasks,
        saved_tasks: &mut load_json_tasks().unwrap(),
    };

    loop {
        let mut buf = String::from("");
        print!("\n >>  ");
        stdout().flush().expect("Hard rendering the cli tool");
        stdin().read_line(&mut buf).expect("Could not parse stdin");
        let line = buf.trim();
        let args = shlex::split(line).expect("error: invalid quoting");

        let app_base_command = "cli-task-tracker".to_string();
        let mut args_to_parse = vec![app_base_command];
        args_to_parse.extend(args);

        match CliTracker::try_parse_from(args_to_parse.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.command {
                    Commands::Add { description } => {
                        // Create a new task
                        TaskStruct {
                            id: format!("{}", Uuid::new_v4()),
                            description,
                            created_at: SystemTime::now(),
                            updated_at: SystemTime::now(),
                            status: TaskStatus::ToDo,
                        }
                        .add(&mut all_tasks);
                    }
                    Commands::List { status } => match status.as_str() {
                        "all" => pretty_table(&list_tasks(TaskStatus::All, &all_tasks)),
                        "done" => pretty_table(&list_tasks(TaskStatus::Done, &all_tasks)),
                        "staged" => pretty_table(&list_tasks(TaskStatus::InProgress, &all_tasks)),
                        "todo" => pretty_table(&list_tasks(TaskStatus::ToDo, &all_tasks)),
                        _ => {
                            println!("Use: --status");
                            println!("\t`all`: retrieve all tasks. [default]");
                            println!("\t`done`: for completed tasks");
                            println!("\t`staged`: for tasks that are in progress");
                            println!("\t`todo`: for upcoming tasks")
                        }
                    },
                    Commands::Stage { id } => match search_task(id, &all_tasks) {
                        Some((space_index, task)) => match task {
                            Some(task) => {
                                task.stage(space_index.1, &mut all_tasks, space_index.0);
                            }
                            _ => eprintln!("No task was found"),
                        },

                        _ => eprintln!("Snap it!"),
                    },
                    Commands::Done { id } => match search_task(id, &all_tasks) {
                        Some((space_index, task)) => match task {
                            Some(task) => {
                                task.done(space_index.1, &mut all_tasks, space_index.0);
                            }
                            _ => eprintln!("No task was found"),
                        },
                        _ => eprintln!("Snap it!"),
                    },
                    Commands::Update { id, description } => match search_task(id, &all_tasks) {
                        Some((space_index, task)) => match task {
                            Some(task) => {
                                let mut new_task = task.clone();
                                new_task.description = description;
                                new_task.updated_at = SystemTime::now();

                                task.update(new_task, &mut all_tasks, space_index.0, space_index.1);
                            }
                            _ => eprintln!("No task was found"),
                        },
                        _ => eprintln!("Snap it!"),
                    },
                    Commands::Delete { id } => match search_task(id, &all_tasks) {
                        Some((space_index, task)) => match task {
                            Some(task) => {
                                task.delete(space_index.1, space_index.0, &mut all_tasks);
                            }
                            _ => eprintln!("No task was found"),
                        },
                        _ => eprintln!("Snap it!"),
                    },

                    Commands::Search { search_key, space } => {
                        // Searches
                        println!("Searching, `{search_key}` in space `{space}`")
                    }
                    Commands::Helps => show_commands(),
                    Commands::Save => save(&all_tasks).unwrap(),
                    Commands::Exit => break,

                    Commands::Cpu { usage } => {
                        println!("{}", hello_cpu());
                        match usage {
                            Some(string) => {
                                if string == "load" {
                                    cpu_usage()
                                } else {
                                    sys_summary()
                                }
                            }
                            None => sys_summary(),
                        }
                    }
                    Commands::Hello => println!("Hello fam!"),
                }
            }

            Err(e) => println!("{e}"),
        }
    }
}
