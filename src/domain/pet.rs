use super::action::{level_from_experience, CareStats};
use super::evolution::{EvolutionEvent, GrowthStage};
use super::monster::{definition, next_species, MonsterFamily, SpeciesId};
use super::status::Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pet {
    pub name: String,
    pub stage: GrowthStage,
    pub species_id: SpeciesId,
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
            stage: GrowthStage::Baby,
            species_id: SpeciesId::Baby,
            level,
            experience,
            status: Status {
                hunger,
                mood,
                energy,
            },
        }
    }

    pub fn update_growth(&mut self, care_stats: CareStats) -> Option<EvolutionEvent> {
        let event = self.evolution_candidate(care_stats)?;
        self.apply_evolution(event);
        Some(event)
    }

    pub fn evolution_candidate(&mut self, care_stats: CareStats) -> Option<EvolutionEvent> {
        if self.stage == GrowthStage::Egg {
            return None;
        }

        self.level = level_from_experience(self.experience);

        let can_evolve = matches!(
            (self.level, self.stage),
            (2.., GrowthStage::Baby) | (3.., GrowthStage::Stage1) | (4.., GrowthStage::Stage2)
        );
        if !can_evolve {
            return None;
        }

        let to_species_id = next_species(self.species_id, care_stats)?;
        let to_stage =
            definition(to_species_id).map_or(GrowthStage::Baby, |monster| monster.growth_stage);

        Some(EvolutionEvent {
            from_stage: self.stage,
            from_species_id: self.species_id,
            to_stage,
            to_species_id,
        })
    }

    pub fn family(&self) -> Option<MonsterFamily> {
        if self.stage == GrowthStage::Egg {
            return None;
        }

        definition(self.species_id).map(|monster| monster.family)
    }

    pub fn species_name(&self) -> &'static str {
        if self.stage == GrowthStage::Egg {
            return "Egg";
        }

        self.species_id.display_name()
    }

    pub fn is_egg(&self) -> bool {
        self.stage == GrowthStage::Egg
    }

    pub fn hatch(&mut self) {
        if self.stage == GrowthStage::Egg {
            self.stage = GrowthStage::Baby;
            self.species_id = SpeciesId::Baby;
            self.level = self.level.max(1);
        }
    }

    pub fn apply_evolution(&mut self, event: EvolutionEvent) {
        self.stage = event.to_stage;
        self.species_id = event.to_species_id;
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            name: "Mochi".to_string(),
            stage: GrowthStage::Egg,
            species_id: SpeciesId::Baby,
            level: 1,
            experience: 0,
            status: Status::default(),
        }
    }
}
