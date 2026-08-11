use bitpet::application::{ApplicationError, GameService};
use bitpet::domain::{GameState, Pet};
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn creates_new_game_when_save_data_does_not_exist() {
    let save_dir = test_save_dir("creates_new_game_when_save_data_does_not_exist");
    let mut service = GameService::new(FileRepository::new(save_dir.clone()));

    let state = service.status().expect("new game should be created");

    assert_eq!(state, GameState::default());
    assert!(save_dir.join("save.json").is_file());

    cleanup(save_dir);
}

#[test]
fn saves_new_game() {
    let save_dir = test_save_dir("saves_new_game");
    let mut repository = FileRepository::new(save_dir.clone());
    let state = GameState::default();

    repository.save(&state).expect("game should be saved");

    let contents = fs::read_to_string(save_dir.join("save.json")).expect("save file should exist");
    assert!(contents.contains(r#""version": 1"#));
    assert!(contents.contains(r#""name": "Mochi""#));

    cleanup(save_dir);
}

#[test]
fn loads_saved_game() {
    let save_dir = test_save_dir("loads_saved_game");
    let mut repository = FileRepository::new(save_dir.clone());
    let state = GameState {
        version: 1,
        pet: Pet::new("Mochi".to_string(), 1, 0, 68, 77, 88),
    };

    repository.save(&state).expect("game should be saved");
    let loaded = repository.load().expect("game should be loaded");

    assert_eq!(loaded, state);

    cleanup(save_dir);
}

#[test]
fn save_then_load_keeps_pet_main_state() {
    let save_dir = test_save_dir("save_then_load_keeps_pet_main_state");
    let mut repository = FileRepository::new(save_dir.clone());
    let state = GameState {
        version: 1,
        pet: Pet::new("Mochi".to_string(), 3, 24, 72, 81, 64),
    };

    repository.save(&state).expect("game should be saved");
    let loaded = repository.load().expect("game should be loaded");

    assert_eq!(loaded.pet.name, state.pet.name);
    assert_eq!(loaded.pet.level, state.pet.level);
    assert_eq!(loaded.pet.experience, state.pet.experience);
    assert_eq!(loaded.pet.status, state.pet.status);

    cleanup(save_dir);
}

#[test]
fn invalid_save_data_returns_error_without_panic() {
    let save_dir = test_save_dir("invalid_save_data_returns_error_without_panic");
    fs::create_dir_all(&save_dir).expect("save directory should be created");
    fs::write(save_dir.join("save.json"), "not json").expect("invalid save should be written");
    let repository = FileRepository::new(save_dir.clone());

    let result = repository.load();

    assert!(matches!(result, Err(ApplicationError::InvalidSaveData)));

    cleanup(save_dir);
}

#[test]
fn detects_first_run_and_existing_game_run() {
    let save_dir = test_save_dir("detects_first_run_and_existing_game_run");
    let mut repository = FileRepository::new(save_dir.clone());

    assert!(!repository.exists());

    repository
        .save(&GameState::default())
        .expect("game should be saved");

    assert!(repository.exists());

    cleanup(save_dir);
}

fn test_save_dir(test_name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("bitpet-{test_name}-{unique}"))
}

fn cleanup(save_dir: PathBuf) {
    if save_dir.exists() {
        fs::remove_dir_all(save_dir).expect("test save directory should be removable");
    }
}
