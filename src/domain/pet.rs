use super::action::{level_from_experience, CareStats};
use super::evolution::GrowthStage;
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

    pub fn update_growth(&mut self, care_stats: CareStats) {
        if self.stage == GrowthStage::Egg {
            return;
        }

        self.level = level_from_experience(self.experience);

        if self.level >= 2 && self.stage == GrowthStage::Baby {
            self.evolve(care_stats);
        }

        if self.level >= 3 && self.stage == GrowthStage::Stage1 {
            self.evolve(care_stats);
        }

        if self.level >= 4 && self.stage == GrowthStage::Stage2 {
            self.evolve(care_stats);
        }
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

    fn evolve(&mut self, care_stats: CareStats) {
        if let Some(species_id) = next_species(self.species_id, care_stats) {
            self.species_id = species_id;
            self.stage =
                definition(species_id).map_or(GrowthStage::Baby, |monster| monster.growth_stage);
        }
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
