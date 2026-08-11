#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrowthStage {
    Baby,
    Stage1,
    Stage2,
    Final,
}

impl GrowthStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Baby => "Baby",
            Self::Stage1 => "Stage 1",
            Self::Stage2 => "Stage 2",
            Self::Final => "Final",
        }
    }
}
