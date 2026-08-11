#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub hunger: u8,
    pub mood: u8,
    pub energy: u8,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            hunger: 72,
            mood: 72,
            energy: 72,
        }
    }
}

impl Status {
    pub fn apply_elapsed(&mut self, hunger_decay: u8, energy_recovery: u8) {
        self.hunger = self.hunger.saturating_sub(hunger_decay);
        self.energy = self.energy.saturating_add(energy_recovery).min(100);
    }
}
