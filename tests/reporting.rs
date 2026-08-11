use bitpet::application::GameService;
use bitpet::domain::{
    CareStats, DailyActions, DailyReport, GameState, LoginState, Pet, SAVE_VERSION,
};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn report_records_actions_and_persists() {
    let save_dir = test_save_dir("report_records_actions_and_persists");
    let mut repository = FileRepository::new(save_dir.clone());
    repository
        .save(&saved_state(3_600))
        .expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    service.feed().expect("feed should succeed");
    service.play().expect("play should succeed");
    let report = service.report().expect("report should load");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    assert_eq!(report.feed_count, 1);
    assert_eq!(report.play_count, 1);
    assert_eq!(report.adventure_count, 0);
    assert_eq!(report.experience_gained, 5);
    assert_eq!(report.mood_delta, 15);
    assert_eq!(report.events.len(), 3);
    assert_eq!(loaded.daily_report, report);

    cleanup(save_dir);
}

#[test]
fn daily_report_resets_on_new_day() {
    let save_dir = test_save_dir("daily_report_resets_on_new_day");
    let mut state = saved_state(3_600);
    state.daily_report.record_feed(3_600, 5);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(90_000),
    );

    let report = service.report().expect("report should load");

    assert_eq!(report.day, 1);
    assert_eq!(report.feed_count, 0);
    assert_eq!(report.play_count, 0);
    assert_eq!(report.events.len(), 1);

    cleanup(save_dir);
}

#[test]
fn streak_counts_consecutive_login_days() {
    let save_dir = test_save_dir("streak_counts_consecutive_login_days");
    let mut repository = FileRepository::new(save_dir.clone());
    repository
        .save(&saved_state(3_600))
        .expect("game should be saved");

    let mut first_day = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );
    assert_eq!(first_day.streak().expect("streak should load").streak, 1);

    let mut second_day = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(90_000),
    );
    assert_eq!(
        second_day.streak().expect("streak should increment").streak,
        2
    );

    cleanup(save_dir);
}

#[test]
fn streak_resets_after_missed_day() {
    let save_dir = test_save_dir("streak_resets_after_missed_day");
    let mut state = saved_state(3_600);
    state.login = LoginState {
        last_login_day: Some(0),
        streak: 2,
    };
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(180_000),
    );

    let login = service.streak().expect("streak should load");

    assert_eq!(login.last_login_day, Some(2));
    assert_eq!(login.streak, 1);

    cleanup(save_dir);
}

#[test]
fn phase5_save_migrates_to_report_and_streak() {
    let save_dir = test_save_dir("phase5_save_migrates_to_report_and_streak");
    fs::create_dir_all(&save_dir).expect("save directory should be created");
    fs::write(
        save_dir.join("save.json"),
        r#"{
  "version": 4,
  "last_updated_at": 3600,
  "daily_actions": {
    "day": 0,
    "feed_count": 1,
    "play_count": 0
  },
  "care_stats": {
    "feed_total": 1,
    "play_total": 0
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
    .expect("phase 5 save should be written");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let state = service.status().expect("old save should migrate");

    assert_eq!(state.version, SAVE_VERSION);
    assert_eq!(state.daily_report.day, 0);
    assert_eq!(state.daily_report.feed_count, 0);
    assert_eq!(state.login.streak, 1);

    cleanup(save_dir);
}

fn saved_state(last_updated_at: u64) -> GameState {
    GameState {
        version: SAVE_VERSION,
        pet: Pet::new("Mochi".to_string(), 1, 0, 72, 72, 72),
        last_updated_at,
        daily_actions: DailyActions::new(last_updated_at / 86_400),
        care_stats: CareStats::new(),
        daily_report: DailyReport::new(last_updated_at / 86_400),
        login: LoginState::new(),
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
