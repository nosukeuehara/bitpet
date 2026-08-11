use bitpet::application::{ApplicationError, GameService};
use bitpet::domain::evolution::GrowthStage;
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn new_game_starts_as_egg_with_deterministic_hatch_time() {
    let save_dir = test_save_dir("new_game_starts_as_egg_with_deterministic_hatch_time");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(1_000),
    );

    let state = service.status().expect("new game should save an egg");

    assert_eq!(state.pet.stage, GrowthStage::Egg);
    assert_eq!(state.last_updated_at, 1_000);
    let hatching = state.hatching.expect("egg should have hatching state");
    assert_eq!(hatching.egg_created_at, 1_000);
    assert_eq!(hatching.hatches_at, 4_600);

    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("egg save should load");
    assert_eq!(loaded, state);

    cleanup(save_dir);
}

#[test]
fn egg_cannot_be_fed_played_or_sent_out_before_hatching() {
    let save_dir = test_save_dir("egg_cannot_be_fed_played_or_sent_out_before_hatching");
    GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(1_000),
    )
    .status()
    .expect("new egg should save");

    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(2_000),
    );

    assert!(matches!(
        service.feed(),
        Err(ApplicationError::PetNotHatched)
    ));
    assert!(matches!(
        service.play(),
        Err(ApplicationError::PetNotHatched)
    ));
    assert!(matches!(
        service.start_expedition(),
        Err(ApplicationError::PetNotHatched)
    ));

    cleanup(save_dir);
}

#[test]
fn egg_hatches_at_exact_boundary_without_status_decay() {
    let save_dir = test_save_dir("egg_hatches_at_exact_boundary_without_status_decay");
    GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(1_000),
    )
    .status()
    .expect("new egg should save");

    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(4_600),
    );
    let state = service.status().expect("egg should hatch");

    assert_eq!(state.pet.stage, GrowthStage::Baby);
    assert!(state.hatching.is_none());
    assert_eq!(state.pet.status.hunger, 72);
    assert_eq!(state.pet.status.energy, 72);
    assert_eq!(state.last_updated_at, 4_600);

    cleanup(save_dir);
}

#[test]
fn hatched_baby_gets_elapsed_time_after_hatch_time() {
    let save_dir = test_save_dir("hatched_baby_gets_elapsed_time_after_hatch_time");
    GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(1_000),
    )
    .status()
    .expect("new egg should save");

    GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(2_000),
    )
    .status()
    .expect("egg status before hatch should save");

    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(8_200),
    );
    let state = service.status().expect("egg should hatch and age");

    assert_eq!(state.pet.stage, GrowthStage::Baby);
    assert_eq!(state.pet.status.hunger, 69);
    assert_eq!(state.pet.status.energy, 74);
    assert_eq!(state.last_updated_at, 8_200);

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
