use bitpet::application::{ApplicationError, GameService};
use bitpet::domain::evolution::GrowthStage;
use bitpet::domain::expedition::ExpeditionType;
use bitpet::domain::monster::{MonsterFamily, SpeciesId};
use bitpet::domain::report::ReportEventKind;
use bitpet::domain::{
    CareStats, DailyActions, DailyReport, GameState, LoginState, Pet, SAVE_VERSION,
};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{
    state_from_json, state_to_json, FileRepository, GameRepository,
};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const SAVE_V0_1_0: &str = include_str!("fixtures/save_v0_1_0.json");
const SAVE_V0_1_1: &str = include_str!("fixtures/save_v0_1_1.json");

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
    assert!(contents.contains(r#""version": 8"#));
    assert!(contents.contains(r#""last_updated_at": 0"#));
    assert!(contents.contains(r#""daily_actions""#));
    assert!(contents.contains(r#""care_stats""#));
    assert!(contents.contains(r#""daily_report""#));
    assert!(contents.contains(r#""login""#));
    assert!(contents.contains(r#""expedition": null"#));
    assert!(contents.contains(r#""hatching""#));
    assert!(contents.contains(r#""egg_created_at": 0"#));
    assert!(contents.contains(r#""hatches_at": 3600"#));
    assert!(contents.contains(r#""stage": "Egg""#));
    assert!(contents.contains(r#""species_id": "baby""#));
    assert!(!contents.contains(r#""evolution""#));
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
        expedition: None,
        hatching: None,
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
        expedition: None,
        hatching: None,
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
fn species_id_survives_save_load_roundtrip() {
    let save_dir = test_save_dir("species_id_survives_save_load_roundtrip");
    let mut repository = FileRepository::new(save_dir.clone());
    let mut state = GameState {
        version: SAVE_VERSION,
        pet: Pet::new("Mochi".to_string(), 4, 30, 72, 81, 64),
        last_updated_at: 7_200,
        daily_actions: DailyActions::new(0),
        care_stats: CareStats::new(),
        daily_report: DailyReport::new(0),
        login: LoginState::new(),
        expedition: None,
        hatching: None,
    };
    state.pet.stage = GrowthStage::Final;
    state.pet.species_id = SpeciesId::Starwing;

    repository.save(&state).expect("game should be saved");
    let loaded = repository.load().expect("game should be loaded");

    assert_eq!(loaded.pet.stage, GrowthStage::Final);
    assert_eq!(loaded.pet.species_id, SpeciesId::Starwing);

    cleanup(save_dir);
}

#[test]
fn wasm_compatible_json_roundtrip_preserves_species_id() {
    let mut state = GameState::default();
    state.pet.stage = GrowthStage::Stage2;
    state.pet.species_id = SpeciesId::Fuzzard;
    state.hatching = None;

    let json = state_to_json(&state).expect("state should serialize");
    let loaded = state_from_json(&json).expect("state should deserialize");

    assert_eq!(loaded.pet.stage, GrowthStage::Stage2);
    assert_eq!(loaded.pet.species_id, SpeciesId::Fuzzard);
}

#[test]
fn migrates_v0_1_x_fixtures_to_v0_2_0_and_roundtrips() {
    assert_v0_1_fixture_migrates("v0_1_0", SAVE_V0_1_0);
    assert_v0_1_fixture_migrates("v0_1_1", SAVE_V0_1_1);
}

#[test]
fn legacy_report_without_event_history_reaches_migration() {
    let state = state_from_json(
        r#"{
  "version": 5,
  "last_updated_at": 9000,
  "daily_actions": {
    "day": 0,
    "feed_count": 1,
    "play_count": 1
  },
  "care_stats": {
    "feed_total": 1,
    "play_total": 1
  },
  "daily_report": {
    "day": 0,
    "feed_count": 1,
    "play_count": 1,
    "adventure_count": 0,
    "experience_gained": 5,
    "mood_delta": 15
  },
  "login": {
    "last_login_day": 0,
    "streak": 1
  },
  "pet": {
    "name": "Mochi",
    "stage": "Stage 1",
    "evolution": "Weird",
    "level": 2,
    "experience": 10,
    "hunger": 72,
    "mood": 72,
    "energy": 72
  }
}"#,
    )
    .expect("legacy save should deserialize before migration defaults are applied");

    assert_eq!(state.version, 5);
    assert_eq!(state.daily_report.feed_count, 1);
    assert!(state.daily_report.events.is_empty());
    assert_eq!(state.pet.species_id, SpeciesId::Wormlet);
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
fn migrates_phase6_save_without_expedition_without_panic() {
    let save_dir = test_save_dir("migrates_phase6_save_without_expedition_without_panic");
    fs::create_dir_all(&save_dir).expect("save directory should be created");
    fs::write(
        save_dir.join("save.json"),
        r#"{
  "version": 5,
  "last_updated_at": 9000,
  "daily_actions": {
    "day": 0,
    "feed_count": 0,
    "play_count": 0
  },
  "care_stats": {
    "feed_total": 0,
    "play_total": 0
  },
  "daily_report": {
    "day": 0,
    "feed_count": 0,
    "play_count": 0,
    "adventure_count": 0,
    "experience_gained": 0,
    "mood_delta": 0,
    "events": []
  },
  "login": {
    "last_login_day": null,
    "streak": 0
  },
  "pet": {
    "name": "Mochi",
    "stage": "Baby",
    "evolution": "Baby",
    "level": 1,
    "experience": 0,
    "hunger": 72,
    "mood": 72,
    "energy": 72
  }
}"#,
    )
    .expect("phase 6 save should be written");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );

    let state = service.status().expect("old save should be migrated");

    assert_eq!(state.version, SAVE_VERSION);
    assert!(state.expedition.is_none());
    assert_eq!(state.login.streak, 1);

    cleanup(save_dir);
}

#[test]
fn migrates_legacy_fluffy_evolution_to_mofflet_species() {
    let save_dir = test_save_dir("migrates_legacy_fluffy_evolution_to_mofflet_species");
    write_legacy_stage1_save(&save_dir, "Fluffy");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );

    let state = service.status().expect("legacy save should migrate");

    assert_eq!(state.version, SAVE_VERSION);
    assert_eq!(state.pet.stage, GrowthStage::Stage1);
    assert_eq!(state.pet.species_id, SpeciesId::Mofflet);
    assert_eq!(state.pet.family(), Some(MonsterFamily::Fuzz));

    cleanup(save_dir);
}

#[test]
fn migrates_legacy_sharp_evolution_to_spindle_species() {
    let save_dir = test_save_dir("migrates_legacy_sharp_evolution_to_spindle_species");
    write_legacy_stage1_save(&save_dir, "Sharp");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );

    let state = service.status().expect("legacy save should migrate");

    assert_eq!(state.pet.species_id, SpeciesId::Spindle);

    cleanup(save_dir);
}

#[test]
fn migrates_legacy_weird_evolution_to_wormlet_species() {
    let save_dir = test_save_dir("migrates_legacy_weird_evolution_to_wormlet_species");
    write_legacy_stage1_save(&save_dir, "Weird");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );

    let state = service.status().expect("legacy save should migrate");

    assert_eq!(state.pet.species_id, SpeciesId::Wormlet);

    cleanup(save_dir);
}

#[test]
fn invalid_expedition_timestamp_returns_error_without_panic() {
    let save_dir = test_save_dir("invalid_expedition_timestamp_returns_error_without_panic");
    fs::create_dir_all(&save_dir).expect("save directory should be created");
    fs::write(
        save_dir.join("save.json"),
        r#"{
  "version": 6,
  "last_updated_at": 9000,
  "daily_actions": {
    "day": 0,
    "feed_count": 0,
    "play_count": 0
  },
  "care_stats": {
    "feed_total": 0,
    "play_total": 0
  },
  "daily_report": {
    "day": 0,
    "feed_count": 0,
    "play_count": 0,
    "adventure_count": 0,
    "experience_gained": 0,
    "mood_delta": 0,
    "events": []
  },
  "login": {
    "last_login_day": null,
    "streak": 0
  },
  "expedition": {
    "expedition_type": "Explore",
    "started_at": 9000,
    "returns_at": 8000,
    "seed": 1
  },
  "pet": {
    "name": "Mochi",
    "stage": "Stage 1",
    "evolution": "Fluffy",
    "level": 2,
    "experience": 10,
    "hunger": 72,
    "mood": 72,
    "energy": 72
  }
}"#,
    )
    .expect("invalid save should be written");
    let repository = FileRepository::new(save_dir.clone());

    let result = repository.load();

    assert!(matches!(result, Err(ApplicationError::InvalidSaveData)));

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

fn assert_v0_1_fixture_migrates(test_name: &str, fixture: &str) {
    let state = state_from_json(fixture).expect("v0.1.x fixture should load");

    assert_eq!(state.version, 6);
    assert_eq!(state.last_updated_at, 9_000);
    assert_eq!(state.pet.name, "Mochi");
    assert_eq!(state.pet.stage, GrowthStage::Stage1);
    assert_eq!(state.pet.species_id, SpeciesId::Mofflet);
    assert_eq!(state.pet.level, 2);
    assert_eq!(state.pet.experience, 15);
    assert_eq!(state.pet.status.hunger, 82);
    assert_eq!(state.pet.status.mood, 76);
    assert_eq!(state.pet.status.energy, 58);
    assert_eq!(state.daily_actions.feed_count, 2);
    assert_eq!(state.daily_actions.play_count, 1);
    assert_eq!(state.care_stats.feed_total, 5);
    assert_eq!(state.care_stats.play_total, 3);
    assert_eq!(state.daily_report.feed_count, 1);
    assert_eq!(state.daily_report.play_count, 1);
    assert_eq!(state.daily_report.adventure_count, 1);
    assert_eq!(state.daily_report.experience_gained, 10);
    assert_eq!(state.daily_report.mood_delta, 20);
    assert_eq!(state.daily_report.events.len(), 4);
    assert_eq!(
        state.daily_report.events[3].kind,
        ReportEventKind::ExpeditionStarted
    );
    assert_eq!(state.login.last_login_day, Some(0));
    assert_eq!(state.login.streak, 4);
    let expedition = state
        .expedition
        .as_ref()
        .expect("expedition should migrate");
    assert_eq!(expedition.expedition_type, ExpeditionType::Explore);
    assert_eq!(expedition.started_at, 8_500);
    assert_eq!(expedition.returns_at, 12_000);
    assert_eq!(expedition.seed, 42);

    let save_dir = test_save_dir(test_name);
    fs::create_dir_all(&save_dir).expect("save directory should be created");
    fs::write(save_dir.join("save.json"), fixture).expect("fixture save should be written");

    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );
    let migrated = service.status().expect("v0.1.x save should migrate");

    assert_eq!(migrated.version, SAVE_VERSION);
    assert_eq!(migrated.pet.species_id, SpeciesId::Mofflet);
    assert_eq!(migrated.pet.family(), Some(MonsterFamily::Fuzz));
    assert_eq!(migrated.daily_actions.feed_count, 2);
    assert_eq!(migrated.daily_actions.play_count, 1);
    assert_eq!(migrated.care_stats.feed_total, 5);
    assert_eq!(migrated.care_stats.play_total, 3);
    assert_eq!(migrated.daily_report.events.len(), 4);
    assert_eq!(migrated.login.streak, 4);
    assert!(migrated.expedition.is_some());

    let saved_json =
        fs::read_to_string(save_dir.join("save.json")).expect("migrated save should exist");
    assert!(saved_json.contains(r#""version": 8"#));
    assert!(saved_json.contains(r#""species_id": "mofflet""#));
    assert!(!saved_json.contains(r#""evolution""#));

    let roundtripped = FileRepository::new(save_dir.clone())
        .load()
        .expect("migrated save should roundtrip");
    assert_eq!(roundtripped, migrated);

    cleanup(save_dir);
}

fn write_legacy_stage1_save(save_dir: &PathBuf, evolution: &str) {
    fs::create_dir_all(save_dir).expect("save directory should be created");
    fs::write(
        save_dir.join("save.json"),
        format!(
            r#"{{
  "version": 6,
  "last_updated_at": 9000,
  "daily_actions": {{
    "day": 0,
    "feed_count": 0,
    "play_count": 0
  }},
  "care_stats": {{
    "feed_total": 0,
    "play_total": 0
  }},
  "daily_report": {{
    "day": 0,
    "feed_count": 0,
    "play_count": 0,
    "adventure_count": 0,
    "experience_gained": 0,
    "mood_delta": 0,
    "events": []
  }},
  "login": {{
    "last_login_day": null,
    "streak": 0
  }},
  "expedition": null,
  "pet": {{
    "name": "Mochi",
    "stage": "Stage 1",
    "evolution": "{evolution}",
    "level": 2,
    "experience": 10,
    "hunger": 72,
    "mood": 72,
    "energy": 72
  }}
}}"#
        ),
    )
    .expect("legacy save should be written");
}

fn cleanup(save_dir: PathBuf) {
    if save_dir.exists() {
        fs::remove_dir_all(save_dir).expect("test save directory should be removable");
    }
}
