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
fn feed_updates_hunger_and_mood_and_saves() {
    let save_dir = test_save_dir("feed_updates_hunger_and_mood_and_saves");
    let state = saved_state(60, 70, 50, 0, 0, 3_600);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let outcome = service.feed().expect("feed should succeed");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    assert_eq!(outcome.state.pet.status.hunger, 80);
    assert_eq!(outcome.state.pet.status.mood, 75);
    assert_eq!(outcome.state.daily_actions.feed_count, 1);
    assert_eq!(outcome.state.daily_report.feed_count, 1);
    assert_eq!(loaded, outcome.state);

    cleanup(save_dir);
}

#[test]
fn play_updates_mood_energy_experience_and_saves() {
    let save_dir = test_save_dir("play_updates_mood_energy_experience_and_saves");
    let state = saved_state(72, 70, 50, 8, 0, 3_600);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let outcome = service.play().expect("play should succeed");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    assert_eq!(outcome.state.pet.status.mood, 80);
    assert_eq!(outcome.state.pet.status.energy, 40);
    assert_eq!(outcome.state.pet.experience, 13);
    assert_eq!(outcome.state.daily_actions.play_count, 1);
    assert_eq!(outcome.state.daily_report.play_count, 1);
    assert_eq!(outcome.state.daily_report.experience_gained, 5);
    assert_eq!(loaded, outcome.state);

    cleanup(save_dir);
}

#[test]
fn daily_feed_limit_returns_error_without_extra_count() {
    let save_dir = test_save_dir("daily_feed_limit_returns_error_without_extra_count");
    let state = saved_state(60, 70, 50, 0, 3, 3_600);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let result = service.feed();
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("game should still load");

    assert!(matches!(
        result,
        Err(ApplicationError::ActionLimitReached(_))
    ));
    assert_eq!(loaded.daily_actions.feed_count, 3);
    assert_eq!(loaded.daily_report.feed_count, 0);
    assert_eq!(loaded.pet.status.hunger, 60);

    cleanup(save_dir);
}

#[test]
fn daily_counts_reset_on_next_day() {
    let save_dir = test_save_dir("daily_counts_reset_on_next_day");
    let state = saved_state(60, 70, 50, 0, 3, 86_000);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(86_500),
    );

    let outcome = service.feed().expect("feed should be available next day");

    assert_eq!(outcome.state.daily_actions.day, 1);
    assert_eq!(outcome.state.daily_actions.feed_count, 1);
    assert_eq!(outcome.state.pet.status.hunger, 80);

    cleanup(save_dir);
}

fn saved_state(
    hunger: u8,
    mood: u8,
    energy: u8,
    experience: u32,
    feed_count: u32,
    last_updated_at: u64,
) -> GameState {
    GameState {
        version: SAVE_VERSION,
        pet: Pet::new("Mochi".to_string(), 1, experience, hunger, mood, energy),
        last_updated_at,
        daily_actions: DailyActions {
            day: last_updated_at / 86_400,
            feed_count,
            play_count: 0,
        },
        care_stats: CareStats {
            feed_total: feed_count,
            play_total: 0,
        },
        daily_report: DailyReport::new(last_updated_at / 86_400),
        login: LoginState::new(),
        expedition: None,
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
