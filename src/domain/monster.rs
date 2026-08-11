use super::action::CareStats;
use super::evolution::GrowthStage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterFamily {
    Fuzz,
    Wing,
    Drift,
    Spike,
    Colony,
    Flora,
    Oddling,
}

impl MonsterFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fuzz => "Fuzz",
            Self::Wing => "Wing",
            Self::Drift => "Drift",
            Self::Spike => "Spike",
            Self::Colony => "Colony",
            Self::Flora => "Flora",
            Self::Oddling => "Oddling",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeciesId {
    Baby,
    Mofflet,
    Fuzzard,
    Brumruff,
    Woolram,
    Flitter,
    Fuzzwing,
    Grandwing,
    Mantara,
    Bloblet,
    Floatle,
    Cloudruff,
    Driftle,
    Spindle,
    Pricklet,
    Starwing,
    Beakruff,
    Buddle,
    Twindle,
    Tribble,
    Cerbloop,
    Spriglet,
    Dewbud,
    Bloomuff,
    Rainroot,
    Wormlet,
    Whiskerp,
    Crownruff,
    Manelet,
}

impl SpeciesId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Baby => "baby",
            Self::Mofflet => "mofflet",
            Self::Fuzzard => "fuzzard",
            Self::Brumruff => "brumruff",
            Self::Woolram => "woolram",
            Self::Flitter => "flitter",
            Self::Fuzzwing => "fuzzwing",
            Self::Grandwing => "grandwing",
            Self::Mantara => "mantara",
            Self::Bloblet => "bloblet",
            Self::Floatle => "floatle",
            Self::Cloudruff => "cloudruff",
            Self::Driftle => "driftle",
            Self::Spindle => "spindle",
            Self::Pricklet => "pricklet",
            Self::Starwing => "starwing",
            Self::Beakruff => "beakruff",
            Self::Buddle => "buddle",
            Self::Twindle => "twindle",
            Self::Tribble => "tribble",
            Self::Cerbloop => "cerbloop",
            Self::Spriglet => "spriglet",
            Self::Dewbud => "dewbud",
            Self::Bloomuff => "bloomuff",
            Self::Rainroot => "rainroot",
            Self::Wormlet => "wormlet",
            Self::Whiskerp => "whiskerp",
            Self::Crownruff => "crownruff",
            Self::Manelet => "manelet",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Baby => "Baby",
            Self::Mofflet => "Mofflet",
            Self::Fuzzard => "Fuzzard",
            Self::Brumruff => "Brumruff",
            Self::Woolram => "Woolram",
            Self::Flitter => "Flitter",
            Self::Fuzzwing => "Fuzzwing",
            Self::Grandwing => "Grandwing",
            Self::Mantara => "Mantara",
            Self::Bloblet => "Bloblet",
            Self::Floatle => "Floatle",
            Self::Cloudruff => "Cloudruff",
            Self::Driftle => "Driftle",
            Self::Spindle => "Spindle",
            Self::Pricklet => "Pricklet",
            Self::Starwing => "Starwing",
            Self::Beakruff => "Beakruff",
            Self::Buddle => "Buddle",
            Self::Twindle => "Twindle",
            Self::Tribble => "Tribble",
            Self::Cerbloop => "Cerbloop",
            Self::Spriglet => "Spriglet",
            Self::Dewbud => "Dewbud",
            Self::Bloomuff => "Bloomuff",
            Self::Rainroot => "Rainroot",
            Self::Wormlet => "Wormlet",
            Self::Whiskerp => "Whiskerp",
            Self::Crownruff => "Crownruff",
            Self::Manelet => "Manelet",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterDefinition {
    pub species_id: SpeciesId,
    pub display_name: &'static str,
    pub family: MonsterFamily,
    pub growth_stage: GrowthStage,
}

pub const MONSTER_CATALOG: &[MonsterDefinition] = &[
    define(SpeciesId::Mofflet, MonsterFamily::Fuzz, GrowthStage::Stage1),
    define(SpeciesId::Fuzzard, MonsterFamily::Fuzz, GrowthStage::Stage2),
    define(SpeciesId::Brumruff, MonsterFamily::Fuzz, GrowthStage::Final),
    define(SpeciesId::Woolram, MonsterFamily::Fuzz, GrowthStage::Final),
    define(SpeciesId::Flitter, MonsterFamily::Wing, GrowthStage::Stage1),
    define(
        SpeciesId::Fuzzwing,
        MonsterFamily::Wing,
        GrowthStage::Stage2,
    ),
    define(
        SpeciesId::Grandwing,
        MonsterFamily::Wing,
        GrowthStage::Final,
    ),
    define(SpeciesId::Mantara, MonsterFamily::Wing, GrowthStage::Final),
    define(
        SpeciesId::Bloblet,
        MonsterFamily::Drift,
        GrowthStage::Stage1,
    ),
    define(
        SpeciesId::Floatle,
        MonsterFamily::Drift,
        GrowthStage::Stage2,
    ),
    define(
        SpeciesId::Cloudruff,
        MonsterFamily::Drift,
        GrowthStage::Final,
    ),
    define(SpeciesId::Driftle, MonsterFamily::Drift, GrowthStage::Final),
    define(
        SpeciesId::Spindle,
        MonsterFamily::Spike,
        GrowthStage::Stage1,
    ),
    define(
        SpeciesId::Pricklet,
        MonsterFamily::Spike,
        GrowthStage::Stage2,
    ),
    define(
        SpeciesId::Starwing,
        MonsterFamily::Spike,
        GrowthStage::Final,
    ),
    define(
        SpeciesId::Beakruff,
        MonsterFamily::Spike,
        GrowthStage::Final,
    ),
    define(
        SpeciesId::Buddle,
        MonsterFamily::Colony,
        GrowthStage::Stage1,
    ),
    define(
        SpeciesId::Twindle,
        MonsterFamily::Colony,
        GrowthStage::Stage2,
    ),
    define(
        SpeciesId::Tribble,
        MonsterFamily::Colony,
        GrowthStage::Final,
    ),
    define(
        SpeciesId::Cerbloop,
        MonsterFamily::Colony,
        GrowthStage::Final,
    ),
    define(
        SpeciesId::Spriglet,
        MonsterFamily::Flora,
        GrowthStage::Stage1,
    ),
    define(SpeciesId::Dewbud, MonsterFamily::Flora, GrowthStage::Stage2),
    define(
        SpeciesId::Bloomuff,
        MonsterFamily::Flora,
        GrowthStage::Final,
    ),
    define(
        SpeciesId::Rainroot,
        MonsterFamily::Flora,
        GrowthStage::Final,
    ),
    define(
        SpeciesId::Wormlet,
        MonsterFamily::Oddling,
        GrowthStage::Stage1,
    ),
    define(
        SpeciesId::Whiskerp,
        MonsterFamily::Oddling,
        GrowthStage::Stage2,
    ),
    define(
        SpeciesId::Crownruff,
        MonsterFamily::Oddling,
        GrowthStage::Final,
    ),
    define(
        SpeciesId::Manelet,
        MonsterFamily::Oddling,
        GrowthStage::Final,
    ),
];

const fn define(
    species_id: SpeciesId,
    family: MonsterFamily,
    growth_stage: GrowthStage,
) -> MonsterDefinition {
    MonsterDefinition {
        species_id,
        display_name: species_id.display_name(),
        family,
        growth_stage,
    }
}

pub fn definition(species_id: SpeciesId) -> Option<&'static MonsterDefinition> {
    MONSTER_CATALOG
        .iter()
        .find(|monster| monster.species_id == species_id)
}

pub fn species_from_str(value: &str) -> Option<SpeciesId> {
    Some(match value {
        "baby" => SpeciesId::Baby,
        "mofflet" => SpeciesId::Mofflet,
        "fuzzard" => SpeciesId::Fuzzard,
        "brumruff" => SpeciesId::Brumruff,
        "woolram" => SpeciesId::Woolram,
        "flitter" => SpeciesId::Flitter,
        "fuzzwing" => SpeciesId::Fuzzwing,
        "grandwing" => SpeciesId::Grandwing,
        "mantara" => SpeciesId::Mantara,
        "bloblet" => SpeciesId::Bloblet,
        "floatle" => SpeciesId::Floatle,
        "cloudruff" => SpeciesId::Cloudruff,
        "driftle" => SpeciesId::Driftle,
        "spindle" => SpeciesId::Spindle,
        "pricklet" => SpeciesId::Pricklet,
        "starwing" => SpeciesId::Starwing,
        "beakruff" => SpeciesId::Beakruff,
        "buddle" => SpeciesId::Buddle,
        "twindle" => SpeciesId::Twindle,
        "tribble" => SpeciesId::Tribble,
        "cerbloop" => SpeciesId::Cerbloop,
        "spriglet" => SpeciesId::Spriglet,
        "dewbud" => SpeciesId::Dewbud,
        "bloomuff" => SpeciesId::Bloomuff,
        "rainroot" => SpeciesId::Rainroot,
        "wormlet" => SpeciesId::Wormlet,
        "whiskerp" => SpeciesId::Whiskerp,
        "crownruff" => SpeciesId::Crownruff,
        "manelet" => SpeciesId::Manelet,
        _ => return None,
    })
}

pub fn legacy_evolution_species(value: Option<&str>) -> Option<SpeciesId> {
    Some(match value {
        None | Some("Baby") => SpeciesId::Baby,
        Some("Fluffy") => SpeciesId::Mofflet,
        Some("Sharp") => SpeciesId::Spindle,
        Some("Weird") => SpeciesId::Wormlet,
        Some(_) => return None,
    })
}

pub fn next_species(current: SpeciesId, care_stats: CareStats) -> Option<SpeciesId> {
    match current {
        SpeciesId::Baby => Some(select_stage1_species(care_stats)),
        SpeciesId::Mofflet => Some(SpeciesId::Fuzzard),
        SpeciesId::Flitter => Some(SpeciesId::Fuzzwing),
        SpeciesId::Bloblet => Some(SpeciesId::Floatle),
        SpeciesId::Spindle => Some(SpeciesId::Pricklet),
        SpeciesId::Buddle => Some(SpeciesId::Twindle),
        SpeciesId::Spriglet => Some(SpeciesId::Dewbud),
        SpeciesId::Wormlet => Some(SpeciesId::Whiskerp),
        SpeciesId::Fuzzard => Some(select_fuzz_final(care_stats)),
        SpeciesId::Fuzzwing => Some(select_wing_final(care_stats)),
        SpeciesId::Floatle => Some(select_drift_final(care_stats)),
        SpeciesId::Pricklet => Some(select_spike_final(care_stats)),
        SpeciesId::Twindle => Some(select_colony_final(care_stats)),
        SpeciesId::Dewbud => Some(select_flora_final(care_stats)),
        SpeciesId::Whiskerp => Some(select_oddling_final(care_stats)),
        _ => None,
    }
}

fn select_stage1_species(care_stats: CareStats) -> SpeciesId {
    match care_stats.feed_total.cmp(&care_stats.play_total) {
        std::cmp::Ordering::Greater => {
            if care_stats.feed_total >= care_stats.play_total.saturating_add(2) {
                SpeciesId::Spriglet
            } else {
                SpeciesId::Mofflet
            }
        }
        std::cmp::Ordering::Less => {
            if care_stats.play_total >= care_stats.feed_total.saturating_add(2) {
                SpeciesId::Flitter
            } else {
                SpeciesId::Spindle
            }
        }
        std::cmp::Ordering::Equal => {
            match care_stats.feed_total.saturating_add(care_stats.play_total) % 3 {
                0 => SpeciesId::Bloblet,
                1 => SpeciesId::Buddle,
                _ => SpeciesId::Wormlet,
            }
        }
    }
}

fn select_fuzz_final(care_stats: CareStats) -> SpeciesId {
    if care_stats.feed_total >= care_stats.play_total {
        SpeciesId::Brumruff
    } else {
        SpeciesId::Woolram
    }
}

fn select_wing_final(care_stats: CareStats) -> SpeciesId {
    if care_stats.play_total > care_stats.feed_total.saturating_add(1) {
        SpeciesId::Grandwing
    } else {
        SpeciesId::Mantara
    }
}

fn select_drift_final(care_stats: CareStats) -> SpeciesId {
    if care_stats.feed_total >= care_stats.play_total {
        SpeciesId::Cloudruff
    } else {
        SpeciesId::Driftle
    }
}

fn select_spike_final(care_stats: CareStats) -> SpeciesId {
    if care_stats.play_total >= care_stats.feed_total {
        SpeciesId::Starwing
    } else {
        SpeciesId::Beakruff
    }
}

fn select_colony_final(care_stats: CareStats) -> SpeciesId {
    if care_stats.play_total >= care_stats.feed_total {
        SpeciesId::Tribble
    } else {
        SpeciesId::Cerbloop
    }
}

fn select_flora_final(care_stats: CareStats) -> SpeciesId {
    if care_stats.feed_total >= care_stats.play_total.saturating_add(2) {
        SpeciesId::Bloomuff
    } else {
        SpeciesId::Rainroot
    }
}

fn select_oddling_final(care_stats: CareStats) -> SpeciesId {
    if care_stats.feed_total == care_stats.play_total {
        SpeciesId::Crownruff
    } else {
        SpeciesId::Manelet
    }
}
