use super::monster::SpeciesId;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvolutionEvent {
    pub from_stage: GrowthStage,
    pub from_species_id: SpeciesId,
    pub to_stage: GrowthStage,
    pub to_species_id: SpeciesId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingEvolution {
    pub from_stage: GrowthStage,
    pub from_species_id: SpeciesId,
    pub to_stage: GrowthStage,
    pub to_species_id: SpeciesId,
}

impl From<EvolutionEvent> for PendingEvolution {
    fn from(event: EvolutionEvent) -> Self {
        Self {
            from_stage: event.from_stage,
            from_species_id: event.from_species_id,
            to_stage: event.to_stage,
            to_species_id: event.to_species_id,
        }
    }
}

impl From<PendingEvolution> for EvolutionEvent {
    fn from(pending: PendingEvolution) -> Self {
        Self {
            from_stage: pending.from_stage,
            from_species_id: pending.from_species_id,
            to_stage: pending.to_stage,
            to_species_id: pending.to_species_id,
        }
    }
}
