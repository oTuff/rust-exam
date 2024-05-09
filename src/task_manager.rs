use crate::task::{Task, Status};
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
    pub fn add_task(&mut self, title: String, description: String, priority: u8, status: Status, deadline: DateTime<Utc>) {
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


    pub fn remove_task(&mut self, task_id: usize) -> bool {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != task_id);
        initial_len != self.tasks.len()
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
