#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrowthStage {
    Baby,
    Stage1,
}

impl GrowthStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Baby => "Baby",
            Self::Stage1 => "Stage 1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvolutionKind {
    Baby,
    Fluffy,
    Sharp,
    Weird,
}

impl EvolutionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Baby => "Baby",
            Self::Fluffy => "Fluffy",
            Self::Sharp => "Sharp",
            Self::Weird => "Weird",
        }
    }
}
