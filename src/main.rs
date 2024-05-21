use chrono::format::ParseError;
use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
use clap::{Arg, Command};
use task_manager::{load_from_file, save_to_file, Status, Task, TaskManager};

fn main() {
    let app = Command::new("Task Manager")
        .version("1.0")
        .author("Christoffer Perch & Oscar Tuff")
        .about("Manages your tasks")
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(
                    Arg::new("title")
                        .help("The title of the task")
                        .long("title")
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .help("The description of the task")
                        .long("description")
                        .required(true),
                )
                .arg(
                    Arg::new("priority")
                        .help("The priority of the task")
                        .long("priority")
                        .default_value("1"),
                )
                .arg(
                    Arg::new("deadline")
                        .help("The deadline for the task (format: YYYY-MM-DD)")
                        .long("deadline")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("delete").about("Deletes a task").arg(
                Arg::new("id")
                    .help("The ID of the task to delete")
                    .long("id")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("update")
                .about("Updates the status of a task")
                .arg(
                    Arg::new("id")
                        .help("The ID of the task to update")
                        .long("id")
                        .required(true),
                )
                .arg(
                    Arg::new("status")
                        .help("The new status of the task (Todo, InProgress, Done)")
                        .long("status")
                        .required(true),
                ),
        )
        .subcommand(Command::new("list").about("Lists all tasks"));

    let matches = app.get_matches();

    let file_path = "tasks.json";
    let mut task_manager = load_from_file(file_path).unwrap_or_else(|_| TaskManager::new());

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let title = sub_m.get_one::<String>("title").unwrap();
            let description = sub_m.get_one::<String>("description").unwrap();
            let priority = sub_m
                .get_one::<String>("priority")
                .unwrap()
                .parse::<u8>()
                .unwrap();
            let deadline_str = sub_m.get_one::<String>("deadline").unwrap();
            let deadline = parse_date_string(deadline_str).expect("Invalid date format");

            task_manager.add_task(
                title.clone(),
                description.clone(),
                priority,
                Status::Todo,
                deadline,
            );
            save_to_file(&task_manager, file_path).expect("Failed to save tasks");
        }
        Some(("delete", sub_m)) => {
            let id = sub_m
                .get_one::<String>("id")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            task_manager.remove_task(id);
            save_to_file(&task_manager, file_path).expect("Failed to save tasks");
        }
        Some(("update", sub_m)) => {
            let id = sub_m
                .get_one::<String>("id")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let status_str = sub_m.get_one::<String>("status").unwrap().to_lowercase();
            let status = match status_str.as_str() {
                "todo" => Status::Todo,
                "inprogress" => Status::InProgress,
                "done" => Status::Done,
                _ => panic!("Invalid status"),
            };
            task_manager.update_task(id, status);
            save_to_file(&task_manager, file_path).expect("Failed to save tasks");
        }
        Some(("list", _)) => {
            for task in task_manager.list_tasks() {
                println!(
                    "ID: {}, Title: {}, Status: {:?}, Description: {}",
                    task.id, task.title, task.status, task.description
                );
            }
        }
        _ => unreachable!(), // Handles unmatched cases
    }
}

fn parse_date_string(date_str: &str) -> Result<DateTime<Utc>, ParseError> {
    let local_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    Ok(Local
        .from_local_date(&local_date)
        .unwrap()
        .and_hms(0, 0, 0)
        .with_timezone(&Utc))
}
