use super::status::Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pet {
    pub name: String,
    pub level: u32,
    pub experience: u32,
    pub status: Status,
}

impl Pet {
    pub fn new(
        name: String,
        level: u32,
        experience: u32,
        hunger: u8,
        mood: u8,
        energy: u8,
    ) -> Self {
        Self {
            name,
            level,
            experience,
            status: Status {
                hunger,
                mood,
                energy,
            },
        }
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            name: "Mochi".to_string(),
            level: 1,
            experience: 0,
            status: Status::default(),
        }
    }
}
