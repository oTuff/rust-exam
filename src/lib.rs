pub mod task;              // Expose the task struct and enums
pub mod task_manager;      // Expose the task management functionality
pub mod file_handler;      // Expose file handling functions

// Optionally, you can re-export important components to simplify access from other modules or external usage
pub use task::{Task, Status};
pub use task_manager::TaskManager;
pub use file_handler::{load_from_file, save_to_file};

// Any shared utilities or common types can be defined or re-exported here if needed.
