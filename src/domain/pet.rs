use super::action::{level_from_experience, CareStats};
use super::evolution::{EvolutionKind, GrowthStage};
use super::status::Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pet {
    pub name: String,
    pub stage: GrowthStage,
    pub evolution: EvolutionKind,
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
            evolution: EvolutionKind::Baby,
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
        self.level = level_from_experience(self.experience);
        if self.level >= 2 && self.stage == GrowthStage::Baby {
            self.stage = GrowthStage::Stage1;
            self.evolution = choose_stage1_evolution(care_stats);
        }
    }
}

fn choose_stage1_evolution(care_stats: CareStats) -> EvolutionKind {
    match care_stats.feed_total.cmp(&care_stats.play_total) {
        std::cmp::Ordering::Greater => EvolutionKind::Fluffy,
        std::cmp::Ordering::Less => EvolutionKind::Sharp,
        std::cmp::Ordering::Equal => EvolutionKind::Weird,
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            name: "Mochi".to_string(),
            stage: GrowthStage::Baby,
            evolution: EvolutionKind::Baby,
            level: 1,
            experience: 0,
            status: Status::default(),
        }
    }
}
