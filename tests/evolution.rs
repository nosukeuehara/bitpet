use bitpet::application::GameService;
use bitpet::domain::evolution::{EvolutionKind, GrowthStage};
use bitpet::domain::{
    CareStats, DailyActions, DailyReport, GameState, LoginState, Pet, SAVE_VERSION,
};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn play_can_level_up_and_evolve_to_sharp() {
    let save_dir = test_save_dir("play_can_level_up_and_evolve_to_sharp");
    let state = saved_state(5, 0, 3_600);
    let mut repository = FileRepository::new(save_dir.clone());
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let outcome = service.play().expect("play should evolve pet");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    assert_eq!(outcome.state.pet.level, 2);
    assert_eq!(outcome.state.pet.stage, GrowthStage::Stage1);
    assert_eq!(outcome.state.pet.evolution, EvolutionKind::Sharp);
    assert_eq!(loaded.pet.evolution, EvolutionKind::Sharp);

    cleanup(save_dir);
}

#[test]
fn feed_focused_pet_evolves_to_fluffy_after_level_threshold() {
    let mut state = saved_state(10, 3, 3_600);

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.level, 2);
    assert_eq!(state.pet.stage, GrowthStage::Stage1);
    assert_eq!(state.pet.evolution, EvolutionKind::Fluffy);
}

#[test]
fn balanced_care_evolves_to_weird_after_level_threshold() {
    let mut state = saved_state(10, 2, 3_600);
    state.care_stats.play_total = 2;

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.level, 2);
    assert_eq!(state.pet.stage, GrowthStage::Stage1);
    assert_eq!(state.pet.evolution, EvolutionKind::Weird);
}

#[test]
fn evolved_pet_does_not_change_evolution_on_later_growth_update() {
    let mut state = saved_state(10, 0, 3_600);
    state.care_stats.play_total = 2;
    state.pet.update_growth(state.care_stats);
    state.care_stats.feed_total = 10;

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.evolution, EvolutionKind::Sharp);
}

fn saved_state(experience: u32, feed_total: u32, last_updated_at: u64) -> GameState {
    GameState {
        version: SAVE_VERSION,
        pet: Pet::new("Mochi".to_string(), 1, experience, 72, 72, 72),
        last_updated_at,
        daily_actions: DailyActions::new(last_updated_at / 86_400),
        care_stats: CareStats {
            feed_total,
            play_total: 0,
        },
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
