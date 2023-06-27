// character.rs
// This file contains the Character struct and its associated methods.
// A Character represents the user's character in the ToDoWin game.

pub struct Character {
    level: u32,
    experience: u32,
}

impl Character {
    // Create a new Character with level 1 and 0 experience.
    pub fn new() -> Self {
        Self {
            level: 1,
            experience: 0,
        }
    }

    // Increase the Character's experience by the given amount.
    // If the Character's experience reaches the level threshold, level up the Character.
    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        if self.experience >= self.level * 100 {
            self.level_up();
        }
    }

    // Level up the Character and reset the experience to 0.
    fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
    }
}
