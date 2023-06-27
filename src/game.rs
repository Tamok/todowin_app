// game.rs
// This file contains the Game struct and its associated methods.
// A Game represents the main game logic of the ToDoWin application.

use crate::task::Task;
use crate::character::Character;

pub struct Game {
    tasks: Vec<Task>,
    character: Character,
}

impl Game {
    // Create a new Game with an empty task list and a new Character.
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            character: Character::new(),
        }
    }

    // Add a Task to the Game's task list.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // Complete a Task in the Game's task list by its id.
    // If the Task is found, mark it as completed and increase the Character's experience.
    pub fn complete_task(&mut self, task_id: u32) {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.complete();
                self.character.gain_experience(10);
                break;
            }
        }
    }
}
