use bitpet::application::GameService;
use bitpet::domain::{CareStats, DailyActions, GameState, Pet, SAVE_VERSION};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn zero_elapsed_time_does_not_change_state() {
    let save_dir = test_save_dir("zero_elapsed_time_does_not_change_state");
    let state = saved_state(72, 72, 3_600);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let updated = service.status().expect("status should load");

    assert_eq!(updated.pet.status.hunger, 72);
    assert_eq!(updated.pet.status.energy, 72);
    assert_eq!(updated.last_updated_at, 3_600);

    cleanup(save_dir);
}

#[test]
fn one_hour_elapsed_updates_status_and_save() {
    let save_dir = test_save_dir("one_hour_elapsed_updates_status_and_save");
    let state = saved_state(72, 72, 3_600);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(7_200),
    );

    let updated = service.status().expect("status should update");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated save should load");

    assert_eq!(updated.pet.status.hunger, 69);
    assert_eq!(updated.pet.status.energy, 74);
    assert_eq!(updated.last_updated_at, 7_200);
    assert_eq!(loaded, updated);

    cleanup(save_dir);
}

#[test]
fn multiple_hours_elapsed_accumulates_changes() {
    let save_dir = test_save_dir("multiple_hours_elapsed_accumulates_changes");
    let state = saved_state(72, 72, 0);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(10_800),
    );

    let updated = service.status().expect("status should update");

    assert_eq!(updated.pet.status.hunger, 63);
    assert_eq!(updated.pet.status.energy, 78);

    cleanup(save_dir);
}

#[test]
fn status_bounds_are_clamped_after_long_elapsed_time() {
    let save_dir = test_save_dir("status_bounds_are_clamped_after_long_elapsed_time");
    let state = saved_state(2, 99, 3_600);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(363_600),
    );

    let updated = service.status().expect("status should update");

    assert_eq!(updated.pet.status.hunger, 0);
    assert_eq!(updated.pet.status.energy, 100);

    cleanup(save_dir);
}

#[test]
fn crossing_date_boundary_uses_elapsed_seconds() {
    let save_dir = test_save_dir("crossing_date_boundary_uses_elapsed_seconds");
    let state = saved_state(72, 72, 86_100);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(90_000),
    );

    let updated = service.status().expect("status should update");

    assert_eq!(updated.pet.status.hunger, 69);
    assert_eq!(updated.pet.status.energy, 74);
    assert_eq!(updated.last_updated_at, 90_000);

    cleanup(save_dir);
}

#[test]
fn future_timestamp_does_not_panic() {
    let save_dir = test_save_dir("future_timestamp_does_not_panic");
    let state = saved_state(72, 72, 10_000);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(9_000),
    );

    let updated = service.status().expect("future timestamp should be safe");

    assert_eq!(updated.pet.status.hunger, 72);
    assert_eq!(updated.pet.status.energy, 72);
    assert_eq!(updated.last_updated_at, 9_000);

    cleanup(save_dir);
}

fn saved_state(hunger: u8, energy: u8, last_updated_at: u64) -> GameState {
    GameState {
        version: SAVE_VERSION,
        pet: Pet::new("Mochi".to_string(), 1, 0, hunger, 72, energy),
        last_updated_at,
        daily_actions: DailyActions::new(last_updated_at / 86_400),
        care_stats: CareStats::new(),
    }
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
