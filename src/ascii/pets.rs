use crate::ascii::monsters::art_by_id;
use crate::domain::monster::SpeciesId;
use crate::domain::Pet;

pub const BABY_PET: &str = r"  /\_/\
 ( o.o )
  > ^ <";

pub fn pet_art(pet: &Pet) -> &'static str {
    match pet.species_id {
        SpeciesId::Baby => BABY_PET,
        species_id => art_by_id(species_id.as_str())
            .map(|monster| monster.art)
            .unwrap_or(BABY_PET),
    }
}
