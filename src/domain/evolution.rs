#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrowthStage {
    Egg,
    Baby,
    Stage1,
    Stage2,
    Final,
}

impl GrowthStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Egg => "Egg",
            Self::Baby => "Baby",
            Self::Stage1 => "Stage 1",
            Self::Stage2 => "Stage 2",
            Self::Final => "Final",
        }
    }
}
