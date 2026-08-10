use crate::ascii::pets::DEFAULT_PET;
use crate::domain::GameState;

pub fn render_status(state: &GameState) -> String {
    let pet = &state.pet;

    format!(
        "{DEFAULT_PET}\n\n{}\nLv. {}\n\nMood     : {}%\nHunger   : {}%\nEnergy   : {}%",
        pet.name, pet.level, pet.status.mood, pet.status.hunger, pet.status.energy
    )
}

pub fn render_not_implemented(command: &str) -> String {
    format!("{command} is not implemented yet.")
}

#[cfg(test)]
mod tests {
    use super::render_status;
    use crate::domain::GameState;

    #[test]
    fn renders_default_pet_status() {
        let output = render_status(&GameState::default());

        assert!(output.contains("Mochi"));
        assert!(output.contains("Lv. 1"));
        assert!(output.contains("Hunger"));
    }
}
