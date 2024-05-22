use crate::task::{Status, Task};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub next_id: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: vec![],
            next_id: 1,
        }
    }

    // CRUD Operations
    pub fn add_task(
        &mut self,
        title: String,
        description: String,
        priority: u8,
        status: Status,
        deadline: DateTime<Utc>,
    ) {
        let task = Task {
            id: self.next_id,
            title,
            description,
            priority,
            status,
            is_cancelled: false,
            deadline,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }
    pub fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
        for (new_id, task) in self.tasks.iter_mut().enumerate() {
            task.id = new_id + 1;
        }
        self.next_id = self.tasks.len() + 1;
    }

    pub fn update_task(&mut self, task_id: usize, new_status: Status) -> Option<()> {
        let task = self.tasks.iter_mut().find(|task| task.id == task_id)?;
        task.status = new_status;
        Some(())
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_new_task_manager() {
        let task_manager = TaskManager::new();
        assert_eq!(task_manager.tasks.len(), 0);
        assert_eq!(task_manager.next_id, 1);
    }

    #[test]
    fn test_add_task() {
        let mut task_manager = TaskManager::new();
        task_manager.add_task(
            "Test Task".to_string(),
            "This is a test task".to_string(),
            1,
            Status::Todo,
            Utc::now() + Duration::days(1),
        );
        assert_eq!(task_manager.tasks.len(), 1);
        assert_eq!(task_manager.next_id, 2);
        let task = &task_manager.tasks[0];
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, "This is a test task");
        assert_eq!(task.priority, 1);
        // assert_eq!(task.status, Status::Todo);
        assert_eq!(task.is_cancelled, false);
    }

    #[test]
    fn test_remove_task() {
        let mut task_manager = TaskManager::new();
        task_manager.add_task(
            "Test Task 1".to_string(),
            "This is a test task".to_string(),
            1,
            Status::Todo,
            Utc::now() + Duration::days(1),
        );
        task_manager.add_task(
            "Test Task 2".to_string(),
            "This is a test task".to_string(),
            1,
            Status::Todo,
            Utc::now() + Duration::days(1),
        );
        assert_eq!(task_manager.remove_task(1), true);
        assert_eq!(task_manager.tasks.len(), 1);
        let task = &task_manager.tasks[0];
        assert_eq!(task.id, 2);
        assert_eq!(task.title, "Test Task 2");
    }

    #[test]
    fn test_remove_task_not_found() {
        let mut task_manager = TaskManager::new();
        assert_eq!(task_manager.remove_task(1), false);
    }

    #[test]
    fn test_update_task() {
        let mut task_manager = TaskManager::new();
        task_manager.add_task(
            "Test Task".to_string(),
            "This is a test task".to_string(),
            1,
            Status::Todo,
            Utc::now() + Duration::days(1),
        );
        task_manager.update_task(1, Status::InProgress).unwrap();
        let task = &task_manager.tasks[0];
        assert_eq!(task.id, 1);
        // assert_eq!(task.status, Status::InProgress);
    }

    #[test]
    fn test_update_task_not_found() {
        let mut task_manager = TaskManager::new();
        assert_eq!(task_manager.update_task(1, Status::InProgress), None);
    }

    #[test]
    fn test_list_tasks() {
        let mut task_manager = TaskManager::new();
        task_manager.add_task(
            "Test Task 1".to_string(),
            "This is a test task".to_string(),
            1,
            Status::Todo,
            Utc::now() + Duration::days(1),
        );
        task_manager.add_task(
            "Test Task 2".to_string(),
            "This is a test task".to_string(),
            1,
            Status::Todo,
            Utc::now() + Duration::days(1),
        );
        let tasks = task_manager.list_tasks();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }
}
