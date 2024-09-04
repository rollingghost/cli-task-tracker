use chrono::{DateTime, Utc};
use comfy_table::Table;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::fmt::Formatter;
use std::fs::OpenOptions;
use std::io::Write;
use std::string::String;
use std::time::SystemTime;
use std::{fmt, fmt::Display, io};

pub struct AllTasks<'a> {
    pub tmp_tasks: &'a mut Vec<TaskStruct>,
    pub saved_tasks: &'a mut Vec<TaskStruct>,
}

#[derive(PartialEq, Debug)]
pub enum Space {
    SavedTasks,
    TmpTasks,
}

#[derive(PartialEq)]
pub enum IfTaskFound {
    TaskFound,
    TaskNotFound,
}

pub trait Task {
    fn delete(self, index: usize, space: Space, all_tasks: &mut AllTasks);
    fn update(
        self,
        new_task: TaskStruct,
        all_tasks: &mut AllTasks,
        space: Space,
        index: usize,
    ) -> TaskStruct;
    fn new(self, all_tasks: &mut AllTasks);
    fn stage(self, index: usize);
    fn done(self, index: usize);
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
pub enum TaskStatus {
    Done,
    InProgress,
    ToDo,
    All,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let status_string = match self {
            TaskStatus::All => "All",
            TaskStatus::Done => "Done",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::ToDo => "To Do",
        };

        write!(f, "{}", status_string)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct TaskStruct {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Task for TaskStruct {
    fn delete(self, index: usize, space: Space, all_tasks: &mut AllTasks) {
        let task_space = match space {
            Space::SavedTasks => &mut all_tasks.saved_tasks,
            Space::TmpTasks => &mut all_tasks.tmp_tasks,
        };
        println!("Len before removing: {}", task_space.len());
        println!("Deleting task with id: {}", self.id);
        task_space.remove(index);
        println!("Len after {}", task_space.len())
    }

    fn update(
        self,
        new_task: TaskStruct,
        all_tasks: &mut AllTasks,
        space: Space,
        index: usize,
    ) -> TaskStruct {
        let task_space = match space {
            Space::SavedTasks => &mut all_tasks.saved_tasks,
            Space::TmpTasks => &mut all_tasks.saved_tasks,
        };

        println!("{}", index);
        println!("Removing: {:?}", task_space[index]);
        task_space.remove(index);
        task_space.push(new_task.clone());

        new_task
    }

    fn new(self, all_tasks: &mut AllTasks) {
        let tmp_tasks = &mut all_tasks.tmp_tasks;
        println!("Before adding: {}", tmp_tasks.len());
        println!("Creating: {:?}", self);
        tmp_tasks.push(self.clone());
        println!("After pushing: {}", tmp_tasks.len());
        println!("{:?}", tmp_tasks);
    }

    fn stage(self, index: usize) {
        let mut new_array = load_json_tasks().unwrap();
        let mut stage_task = new_array.remove(index);
        stage_task.status = TaskStatus::InProgress;
        stage_task.updated_at = SystemTime::now();

        new_array.push(stage_task);
    }

    fn done(self, _index: usize) {
        todo!()
    }
}

pub fn list_tasks(status: TaskStatus, all_tasks: &AllTasks) -> Vec<TaskStruct> {
    let tasks_to_list = all_tasks_to_list(all_tasks);
    match status {
        TaskStatus::Done => {
            let mut done: Vec<TaskStruct> = Vec::new();

            for task in tasks_to_list {
                if task.status == TaskStatus::Done {
                    done.push(task);
                }
            }

            done
        }
        TaskStatus::InProgress => {
            let mut in_progress: Vec<TaskStruct> = Vec::new();

            for task in tasks_to_list {
                if task.status == TaskStatus::InProgress {
                    in_progress.push(task);
                }
            }

            in_progress
        }

        TaskStatus::ToDo => {
            let mut todo: Vec<TaskStruct> = Vec::new();

            for task in tasks_to_list {
                if task.status == TaskStatus::ToDo {
                    todo.push(task);
                }
            }

            todo
        }

        TaskStatus::All => tasks_to_list,
    }
}
fn all_tasks_to_list(all_tasks: &AllTasks) -> Vec<TaskStruct> {
    let mut joined_vectors = all_tasks.saved_tasks.clone();
    joined_vectors.extend(all_tasks.tmp_tasks.iter().cloned());

    joined_vectors
}

// Find if a task exists in both temporary and saved tasks
pub fn check_task(id: String, all_tasks: Vec<TaskStruct>) -> (Option<TaskStruct>, usize) {
    let index = 0;
    let mut ref_task = None;
    for task in all_tasks {
        if task.id == id {
            ref_task = Some(task);
        }
    }

    match ref_task {
        Some(task) => (Some(task), index),
        None => (None, index),
    }
}

pub fn show_commands() {
    println!(
        r#"
        add <title> <description>       --> Creates a new task
        list --status <STATUS>          --> Lists tasks with specific status (all, done, staged, todo)
        done <ID>                       --> Marks a task with <ID> as done
        delete <ID>                     --> Deletes a task with <ID> permanently
        trash <ID>                      --> Moves a task with <ID> to trash
        help                            --> Shows this help message
        update <ID> <TITLE> <DESC>      --> Updates a task with <ID> with new content <TITLE> for title and <DESC> for description
        stage <ID>                      --> Marks a task with <ID> as InProgress
    "#
    );
}

// Save temporary tasks permanently
pub fn save(all_tasks: &AllTasks) -> Result<(), SerdeError> {
    // Iterate over the tmp_tasks and save them to tasks.json
    let mut new_join = all_tasks.saved_tasks.clone();
    new_join.extend(all_tasks.tmp_tasks.iter().cloned());

    save_tasks_permanently(&new_join).unwrap();
    Ok(())
}

fn save_tasks_permanently(tasks: &Vec<TaskStruct>) -> Result<(), Box<dyn std::error::Error>> {
    // Open the JSON file and append the task
    println!("Saving");
    let file_path = "tasks.json";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer(&mut writer, &tasks)?;
    writer.write_all(b"\n")?;
    writer.flush()?;

    Ok(())
}

// Load the json file and return a vector
pub fn load_json_tasks() -> Result<Vec<TaskStruct>, Box<dyn std::error::Error>> {
    let file_path = "tasks.json";

    // Read the existing file content or use an empty array
    // if file doesn't exist
    let file_content = std::fs::read_to_string(file_path).unwrap_or_else(|_| "[]".to_string());

    let tasks: Vec<TaskStruct> = serde_json::from_str(&file_content)?;

    Ok(tasks)
}

// Make a table when displaying tasks
pub fn pretty_table(tasks: &Vec<TaskStruct>) {
    let mut table = Table::new();

    table.set_header(vec![
        "No",
        "ID",
        "Description",
        "Created",
        "Updated",
        "Status",
    ]);
    let mut index = 1;

    for task in tasks {
        table.add_row(vec![
            index.to_string(),
            task.id.clone(),
            break_string(&task.description),
            system_time_to_string(task.created_at),
            system_time_to_string(task.updated_at),
            task.status.to_string(),
        ]);

        index += 1;
    }

    println!("{table}")
}

// Adds breaks in a long character string
fn break_string(string: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for ch in string.chars() {
        if count == 45 {
            result.push('\n');
            count = 0;
        }

        result.push(ch);
        count += 1;
    }

    result
}

// Converting from system time to a human-readable string
fn system_time_to_string(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = system_time.into();
    datetime.format("%Y-%m-%d \n %H:%M:%S").to_string()
}

// Stage task, move a task from todo to in progress
pub fn stage_task(
    id: String,
    all_tasks: &AllTasks,
) -> Result<Vec<TaskStruct>, Box<dyn std::error::Error>> {
    // Try to find the task and updated it
    let mut task_to_modify = Vec::new();
    let _modified_task: Vec<TaskStruct> = Vec::new();
    let mut joined_vectors = all_tasks.saved_tasks.clone();
    joined_vectors.extend(all_tasks.tmp_tasks.iter().cloned());

    for task in joined_vectors {
        if task.id == id {
            task_to_modify.push(task);
            pretty_table(&task_to_modify);
            break;
        }
    }

    if !task_to_modify.is_empty() {
        task_to_modify[0].status = TaskStatus::InProgress;
    }

    pretty_table(&task_to_modify);

    Ok(task_to_modify)
}

// Search a task
pub fn search_task(
    id: String,
    all_tasks: &AllTasks,
) -> Option<((Space, usize), Option<TaskStruct>)> {
    let mut search_result = ((Space::SavedTasks, 0), None);
    let mut index: usize = 0;
    let mut is_found: IfTaskFound = IfTaskFound::TaskNotFound;

    for tmp_task in all_tasks.tmp_tasks.clone() {
        if tmp_task.id == id {
            search_result = ((Space::TmpTasks, index), Some(tmp_task));
            is_found = IfTaskFound::TaskFound;
            break;
        }

        index += 1;
    }

    if is_found == IfTaskFound::TaskNotFound {
        index = 0;
        for saved_task in all_tasks.saved_tasks.clone() {
            if saved_task.id == id {
                search_result = ((Space::SavedTasks, index), Some(saved_task));
                break;
            }

            index += 1;
        }
    }

    Some(search_result)
}
