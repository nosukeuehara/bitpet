use bitpet::application::{ApplicationError, GameService};
use bitpet::domain::{
    CareStats, DailyActions, DailyReport, GameState, LoginState, Pet, SAVE_VERSION,
};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn creates_new_game_when_save_data_does_not_exist() {
    let save_dir = test_save_dir("creates_new_game_when_save_data_does_not_exist");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let state = service.status().expect("new game should be created");

    assert_eq!(state.version, SAVE_VERSION);
    assert_eq!(state.pet, GameState::new(3_600).pet);
    assert_eq!(state.last_updated_at, 3_600);
    assert_eq!(state.login.streak, 1);
    assert_eq!(state.daily_report.events.len(), 1);
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
    assert!(contents.contains(r#""version": 5"#));
    assert!(contents.contains(r#""last_updated_at": 0"#));
    assert!(contents.contains(r#""daily_actions""#));
    assert!(contents.contains(r#""care_stats""#));
    assert!(contents.contains(r#""daily_report""#));
    assert!(contents.contains(r#""login""#));
    assert!(contents.contains(r#""stage": "Baby""#));
    assert!(contents.contains(r#""evolution": "Baby""#));
    assert!(contents.contains(r#""name": "Mochi""#));

    cleanup(save_dir);
}

#[test]
fn loads_saved_game() {
    let save_dir = test_save_dir("loads_saved_game");
    let mut repository = FileRepository::new(save_dir.clone());
    let state = GameState {
        version: SAVE_VERSION,
        pet: Pet::new("Mochi".to_string(), 1, 0, 68, 77, 88),
        last_updated_at: 3_600,
        daily_actions: DailyActions::new(0),
        care_stats: CareStats::new(),
        daily_report: DailyReport::new(0),
        login: LoginState::new(),
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
        version: SAVE_VERSION,
        pet: Pet::new("Mochi".to_string(), 3, 24, 72, 81, 64),
        last_updated_at: 7_200,
        daily_actions: DailyActions::new(0),
        care_stats: CareStats::new(),
        daily_report: DailyReport::new(0),
        login: LoginState::new(),
    };

    repository.save(&state).expect("game should be saved");
    let loaded = repository.load().expect("game should be loaded");

    assert_eq!(loaded.pet.name, state.pet.name);
    assert_eq!(loaded.pet.level, state.pet.level);
    assert_eq!(loaded.pet.experience, state.pet.experience);
    assert_eq!(loaded.pet.status, state.pet.status);
    assert_eq!(loaded.last_updated_at, state.last_updated_at);

    cleanup(save_dir);
}

#[test]
fn migrates_phase2_save_without_last_updated_at_without_panic() {
    let save_dir = test_save_dir("migrates_phase2_save_without_last_updated_at_without_panic");
    fs::create_dir_all(&save_dir).expect("save directory should be created");
    fs::write(
        save_dir.join("save.json"),
        r#"{
  "version": 1,
  "pet": {
    "name": "Mochi",
    "level": 1,
    "experience": 0,
    "hunger": 72,
    "mood": 72,
    "energy": 72
  }
}"#,
    )
    .expect("phase 2 save should be written");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );

    let state = service.status().expect("old save should be migrated");

    assert_eq!(state.version, SAVE_VERSION);
    assert_eq!(state.pet.status.hunger, 72);
    assert_eq!(state.last_updated_at, 9_000);
    assert_eq!(state.daily_actions.day, 0);
    assert_eq!(state.care_stats.feed_total, 0);
    assert_eq!(state.care_stats.play_total, 0);
    assert_eq!(state.daily_report.day, 0);
    assert_eq!(state.login.streak, 1);

    cleanup(save_dir);
}

#[test]
fn migrates_phase3_save_without_daily_actions_without_panic() {
    let save_dir = test_save_dir("migrates_phase3_save_without_daily_actions_without_panic");
    fs::create_dir_all(&save_dir).expect("save directory should be created");
    fs::write(
        save_dir.join("save.json"),
        r#"{
  "version": 2,
  "last_updated_at": 9000,
  "pet": {
    "name": "Mochi",
    "level": 1,
    "experience": 0,
    "hunger": 72,
    "mood": 72,
    "energy": 72
  }
}"#,
    )
    .expect("phase 3 save should be written");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );

    let state = service.status().expect("old save should be migrated");

    assert_eq!(state.version, SAVE_VERSION);
    assert_eq!(state.daily_actions.day, 0);
    assert_eq!(state.daily_actions.feed_count, 0);
    assert_eq!(state.daily_actions.play_count, 0);
    assert_eq!(state.care_stats.feed_total, 0);
    assert_eq!(state.care_stats.play_total, 0);
    assert_eq!(state.daily_report.feed_count, 0);
    assert_eq!(state.login.streak, 1);

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
