use crate::task::Task;
use crate::character::Character;

pub struct Game {
    tasks: Vec<Task>,
    character: Character,
}

impl Game {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            character: Character::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

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
