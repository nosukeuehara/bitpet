use bitpet::application::ApplicationError;
use bitpet::domain::action::Action;

#[test]
fn user_facing_errors_do_not_expose_internal_details() {
    let invalid_save = ApplicationError::InvalidSaveData.to_string();
    let pet_away = ApplicationError::PetAway.to_string();
    let action_limit = ApplicationError::ActionLimitReached(Action::Feed).to_string();

    assert!(invalid_save.contains("couldn't read your save data"));
    assert!(!invalid_save.contains("serde"));
    assert!(!invalid_save.contains("panic"));
    assert!(pet_away.contains("Mochi is exploring."));
    assert!(action_limit.contains("Maybe try again tomorrow."));
}
