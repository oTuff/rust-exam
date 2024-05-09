// TODO: fix tests
// use super::super::task::{Status, Task};
// use super::super::task_manager::TaskManager;
// use chrono::{DateTime, Utc};
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add_task() {
//         let mut task_manager = TaskManager::new();
//         task_manager.add_task(
//             "Test Task".to_string(),
//             "This is a test task".to_string(),
//             1,
//             Status::Todo,
//             Utc::now() + Duration::days(1),
//         );

//         let tasks = task_manager.list_tasks();
//         assert_eq!(tasks.len(), 1);
//         assert_eq!(tasks[0].title, "Test Task");
//         assert_eq!(tasks[0].description, "This is a test task");
//         assert_eq!(tasks[0].priority, 1);
//         assert_eq!(tasks[0].status, Status::Todo);
//         assert!(!tasks[0].is_cancelled);
//     }

//     #[test]
//     fn test_remove_task() {
//         let mut task_manager = TaskManager::new();
//         task_manager.add_task(
//             "Test Task".to_string(),
//             "This is a test task".to_string(),
//             1,
//             Status::Todo,
//             Utc::now() + Duration::days(1),
//         );

//         let result = task_manager.remove_task(1);
//         assert!(result);

//         let tasks = task_manager.list_tasks();
//         assert_eq!(tasks.len(), 0);
//     }

//     #[test]
//     fn test_update_task() {
//         let mut task_manager = TaskManager::new();
//         task_manager.add_task(
//             "Test Task".to_string(),
//             "This is a test task".to_string(),
//             1,
//             Status::Todo,
//             Utc::now() + Duration::days(1),
//         );

//         task_manager.update_task(1, Status::Done).unwrap();

//         let tasks = task_manager.list_tasks();
//         assert_eq!(tasks.len(), 1);
//         assert_eq!(tasks[0].status, Status::Done);
//     }
// }
