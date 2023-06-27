// task.rs
// This file contains the Task struct and its associated methods.
// A Task represents a task in the ToDoWin application.

pub struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    // Create a new Task with the given id and title.
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    // Mark the Task as completed.
    pub fn complete(&mut self) {
        self.completed = true;
    }
}
