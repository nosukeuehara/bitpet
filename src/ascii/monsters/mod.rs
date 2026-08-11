//! Static ASCII assets for the expanded BitPet monster catalog.
//!
//! This module owns rendering assets only. Evolution rules belong in the domain layer.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonsterArt {
    pub id: &'static str,
    pub name: &'static str,
    pub family: &'static str,
    pub stage: &'static str,
    pub art: &'static str,
}

pub const MOFFLET: &str = include_str!("fuzz/mofflet.txt");
pub const FUZZARD: &str = include_str!("fuzz/fuzzard.txt");
pub const BRUMRUFF: &str = include_str!("fuzz/brumruff.txt");
pub const WOOLRAM: &str = include_str!("fuzz/woolram.txt");
pub const FLITTER: &str = include_str!("wing/flitter.txt");
pub const FUZZWING: &str = include_str!("wing/fuzzwing.txt");
pub const GRANDWING: &str = include_str!("wing/grandwing.txt");
pub const MANTARA: &str = include_str!("wing/mantara.txt");
pub const BLOBLET: &str = include_str!("drift/bloblet.txt");
pub const FLOATLE: &str = include_str!("drift/floatle.txt");
pub const CLOUDRUFF: &str = include_str!("drift/cloudruff.txt");
pub const DRIFTLE: &str = include_str!("drift/driftle.txt");
pub const SPINDLE: &str = include_str!("spike/spindle.txt");
pub const PRICKLET: &str = include_str!("spike/pricklet.txt");
pub const STARWING: &str = include_str!("spike/starwing.txt");
pub const BEAKRUFF: &str = include_str!("spike/beakruff.txt");
pub const BUDDLE: &str = include_str!("colony/buddle.txt");
pub const TWINDLE: &str = include_str!("colony/twindle.txt");
pub const TRIBBLE: &str = include_str!("colony/tribble.txt");
pub const CERBLOOP: &str = include_str!("colony/cerbloop.txt");
pub const SPRIGLET: &str = include_str!("flora/spriglet.txt");
pub const DEWBUD: &str = include_str!("flora/dewbud.txt");
pub const BLOOMUFF: &str = include_str!("flora/bloomuff.txt");
pub const RAINROOT: &str = include_str!("flora/rainroot.txt");
pub const WORMLET: &str = include_str!("oddling/wormlet.txt");
pub const WHISKERP: &str = include_str!("oddling/whiskerp.txt");
pub const CROWNRUFF: &str = include_str!("oddling/crownruff.txt");
pub const MANELET: &str = include_str!("oddling/manelet.txt");

pub const MONSTER_ARTS: &[MonsterArt] = &[
    MonsterArt {
        id: "mofflet",
        name: "Mofflet",
        family: "fuzz",
        stage: "stage1",
        art: MOFFLET,
    },
    MonsterArt {
        id: "fuzzard",
        name: "Fuzzard",
        family: "fuzz",
        stage: "stage2",
        art: FUZZARD,
    },
    MonsterArt {
        id: "brumruff",
        name: "Brumruff",
        family: "fuzz",
        stage: "final",
        art: BRUMRUFF,
    },
    MonsterArt {
        id: "woolram",
        name: "Woolram",
        family: "fuzz",
        stage: "final",
        art: WOOLRAM,
    },
    MonsterArt {
        id: "flitter",
        name: "Flitter",
        family: "wing",
        stage: "stage1",
        art: FLITTER,
    },
    MonsterArt {
        id: "fuzzwing",
        name: "Fuzzwing",
        family: "wing",
        stage: "stage2",
        art: FUZZWING,
    },
    MonsterArt {
        id: "grandwing",
        name: "Grandwing",
        family: "wing",
        stage: "final",
        art: GRANDWING,
    },
    MonsterArt {
        id: "mantara",
        name: "Mantara",
        family: "wing",
        stage: "final",
        art: MANTARA,
    },
    MonsterArt {
        id: "bloblet",
        name: "Bloblet",
        family: "drift",
        stage: "stage1",
        art: BLOBLET,
    },
    MonsterArt {
        id: "floatle",
        name: "Floatle",
        family: "drift",
        stage: "stage2",
        art: FLOATLE,
    },
    MonsterArt {
        id: "cloudruff",
        name: "Cloudruff",
        family: "drift",
        stage: "final",
        art: CLOUDRUFF,
    },
    MonsterArt {
        id: "driftle",
        name: "Driftle",
        family: "drift",
        stage: "final",
        art: DRIFTLE,
    },
    MonsterArt {
        id: "spindle",
        name: "Spindle",
        family: "spike",
        stage: "stage1",
        art: SPINDLE,
    },
    MonsterArt {
        id: "pricklet",
        name: "Pricklet",
        family: "spike",
        stage: "stage2",
        art: PRICKLET,
    },
    MonsterArt {
        id: "starwing",
        name: "Starwing",
        family: "spike",
        stage: "final",
        art: STARWING,
    },
    MonsterArt {
        id: "beakruff",
        name: "Beakruff",
        family: "spike",
        stage: "final",
        art: BEAKRUFF,
    },
    MonsterArt {
        id: "buddle",
        name: "Buddle",
        family: "colony",
        stage: "stage1",
        art: BUDDLE,
    },
    MonsterArt {
        id: "twindle",
        name: "Twindle",
        family: "colony",
        stage: "stage2",
        art: TWINDLE,
    },
    MonsterArt {
        id: "tribble",
        name: "Tribble",
        family: "colony",
        stage: "final",
        art: TRIBBLE,
    },
    MonsterArt {
        id: "cerbloop",
        name: "Cerbloop",
        family: "colony",
        stage: "final",
        art: CERBLOOP,
    },
    MonsterArt {
        id: "spriglet",
        name: "Spriglet",
        family: "flora",
        stage: "stage1",
        art: SPRIGLET,
    },
    MonsterArt {
        id: "dewbud",
        name: "Dewbud",
        family: "flora",
        stage: "stage2",
        art: DEWBUD,
    },
    MonsterArt {
        id: "bloomuff",
        name: "Bloomuff",
        family: "flora",
        stage: "final",
        art: BLOOMUFF,
    },
    MonsterArt {
        id: "rainroot",
        name: "Rainroot",
        family: "flora",
        stage: "final",
        art: RAINROOT,
    },
    MonsterArt {
        id: "wormlet",
        name: "Wormlet",
        family: "oddling",
        stage: "stage1",
        art: WORMLET,
    },
    MonsterArt {
        id: "whiskerp",
        name: "Whiskerp",
        family: "oddling",
        stage: "stage2",
        art: WHISKERP,
    },
    MonsterArt {
        id: "crownruff",
        name: "Crownruff",
        family: "oddling",
        stage: "final",
        art: CROWNRUFF,
    },
    MonsterArt {
        id: "manelet",
        name: "Manelet",
        family: "oddling",
        stage: "final",
        art: MANELET,
    },
];

pub fn art_by_id(id: &str) -> Option<&'static MonsterArt> {
    MONSTER_ARTS.iter().find(|monster| monster.id == id)
}
