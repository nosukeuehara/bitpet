use crate::ascii::pets::pet_art;
use crate::domain::action::{Action, ActionOutcome};
use crate::domain::GameState;

pub fn render_status(state: &GameState) -> String {
    let pet = &state.pet;

    format!(
        "{}\n\n{}\n{}\nLv. {}\n\nStage    : {}\nMood     : {}%\nHunger   : {}%\nEnergy   : {}%",
        pet_art(pet),
        pet.name,
        pet.evolution.as_str(),
        pet.level,
        pet.stage.as_str(),
        pet.status.mood,
        pet.status.hunger,
        pet.status.energy
    )
}

pub fn render_not_implemented(command: &str) -> String {
    format!("{command} is not implemented yet.")
}

pub fn render_action_outcome(outcome: &ActionOutcome) -> String {
    let message = match outcome.action {
        Action::Feed => "Mochi ate a meal.",
        Action::Play => "You played with Mochi.",
        Action::Go => "go is not implemented yet.",
    };

    format!("{message}\n\n{}", render_status(&outcome.state))
}

pub fn render_action_limit_reached(action: Action) -> String {
    match action {
        Action::Feed => "Mochi looks full.\n\nMaybe try again tomorrow.".to_string(),
        Action::Play => "Mochi needs a break.\n\nMaybe try again tomorrow.".to_string(),
        Action::Go => "That action is not available today.".to_string(),
    }
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
