use chrono::{Local, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        let now = Local::now();

        Self {
            id,
            description,
            status: TaskStatus::Todo,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
        self.updated_at = Local::now();
    }

    pub fn update_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
        self.updated_at = Local::now();
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_create_a_task() {
        let task = Task::new(1, String::from("Implement Rust API"));
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Implement Rust API");
        assert_eq!(task.status, TaskStatus::Todo);
    }

    #[test]
    fn test_update_task_state() {
        let mut task = Task::new(1, String::from("Implement Go API"));
        task.update_status(TaskStatus::InProgress);
        assert_eq!(task.status, TaskStatus::InProgress);
    }

    #[test]
    fn test_update_task_description() {
        let mut task = Task::new(1, String::from("Implement Python API"));
        task.update_description(String::from("Implement Python API with FastAPI"));
        assert_eq!(task.description, "Implement Python API with FastAPI");
    }
}
