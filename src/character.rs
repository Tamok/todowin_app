pub struct Character {
    level: u32,
    experience: u32,
}

impl Character {
    pub fn new() -> Self {
        Self {
            level: 1,
            experience: 0,
        }
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        if self.experience >= self.level * 100 {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
    }
}
