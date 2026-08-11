use crate::domain::evolution::EvolutionKind;
use crate::domain::Pet;

pub const BABY_PET: &str = r"  /\_/\
 ( o.o )
  > ^ <";

pub const FLUFFY_PET: &str = r"  /\_/\
 ( =.= )
  (___)";

pub const SHARP_PET: &str = r"  /\_/\
 ( >.< )
  / ^ \";

pub const WEIRD_PET: &str = r"  /\_/\
 ( @.@ )
  <___>";

pub fn pet_art(pet: &Pet) -> &'static str {
    match pet.evolution {
        EvolutionKind::Baby => BABY_PET,
        EvolutionKind::Fluffy => FLUFFY_PET,
        EvolutionKind::Sharp => SHARP_PET,
        EvolutionKind::Weird => WEIRD_PET,
    }
}
