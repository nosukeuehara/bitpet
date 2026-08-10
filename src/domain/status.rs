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
