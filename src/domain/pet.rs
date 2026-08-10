use super::status::Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pet {
    pub name: String,
    pub level: u32,
    pub experience: u32,
    pub status: Status,
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
