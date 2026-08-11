use bitpet::application::{ApplicationError, GameService};
use bitpet::domain::evolution::GrowthStage;
use bitpet::domain::expedition::ExpeditionType;
use bitpet::domain::monster::SpeciesId;
use bitpet::domain::{
    CareStats, DailyActions, DailyReport, GameState, LoginState, PendingEvolution, Pet,
    SAVE_VERSION,
};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn baby_pet_cannot_start_expedition() {
    let save_dir = test_save_dir("baby_pet_cannot_start_expedition");
    let mut repository = FileRepository::new(save_dir.clone());
    repository
        .save(&saved_state(3_600, GrowthStage::Baby))
        .expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let result = service.start_expedition();

    assert!(matches!(result, Err(ApplicationError::ExpeditionLocked)));

    cleanup(save_dir);
}

#[test]
fn stage1_pet_can_start_expedition_and_save_away_state() {
    let save_dir = test_save_dir("stage1_pet_can_start_expedition_and_save_away_state");
    let mut repository = FileRepository::new(save_dir.clone());
    repository
        .save(&saved_state(3_600, GrowthStage::Stage1))
        .expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(3_600),
    );

    let outcome = service.start_expedition().expect("expedition should start");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    assert_eq!(outcome.expedition_type, ExpeditionType::Explore);
    assert_eq!(outcome.returns_at, 7_200);
    assert!(loaded.expedition.is_some());
    assert_eq!(loaded.pet.status.energy, 62);
    assert_eq!(loaded.daily_report.adventure_count, 1);

    cleanup(save_dir);
}

#[test]
fn pet_cannot_be_fed_or_played_with_while_away() {
    let save_dir = test_save_dir("pet_cannot_be_fed_or_played_with_while_away");
    let mut repository = FileRepository::new(save_dir.clone());
    let mut state = saved_state(3_600, GrowthStage::Stage1);
    state
        .start_expedition(3_600, 0, 3_600)
        .expect("expedition should start");
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(4_000),
    );

    assert!(matches!(service.feed(), Err(ApplicationError::PetAway)));
    assert!(matches!(service.play(), Err(ApplicationError::PetAway)));

    cleanup(save_dir);
}

#[test]
fn expedition_state_survives_save_and_load() {
    let save_dir = test_save_dir("expedition_state_survives_save_and_load");
    let mut repository = FileRepository::new(save_dir.clone());
    let mut state = saved_state(3_600, GrowthStage::Stage1);
    state
        .start_expedition(3_600, 0, 3_600)
        .expect("expedition should start");

    repository.save(&state).expect("game should be saved");
    let loaded = repository.load().expect("game should load");

    assert_eq!(loaded.expedition, state.expedition);

    cleanup(save_dir);
}

#[test]
fn completed_expedition_applies_reward_and_clears_away_state() {
    let save_dir = test_save_dir("completed_expedition_applies_reward_and_clears_away_state");
    let mut repository = FileRepository::new(save_dir.clone());
    let mut state = saved_state(3_600, GrowthStage::Stage1);
    state
        .start_expedition(3_600, 0, 3_600)
        .expect("expedition should start");
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(7_200),
    );

    let updated = service.status().expect("status should complete expedition");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    assert!(updated.expedition.is_none());
    assert_eq!(updated.pet.experience, 5);
    assert_eq!(updated.pet.status.mood, 77);
    assert_eq!(updated.daily_report.experience_gained, 5);
    assert_eq!(loaded, updated.state);

    cleanup(save_dir);
}

#[test]
fn expedition_completion_queues_evolution_without_changing_visible_species() {
    let mut state = saved_state(3_600, GrowthStage::Stage1);
    state.pet.experience = 15;
    state
        .start_expedition(3_600, 0, 3_600)
        .expect("expedition should start");

    state.complete_expedition_if_due(7_200);

    assert!(state.expedition.is_none());
    assert_eq!(state.pet.level, 3);
    assert_eq!(state.pet.stage, GrowthStage::Stage1);
    assert_eq!(state.pet.species_id, SpeciesId::Mofflet);
    let pending = state
        .pending_evolution
        .expect("expedition reward should queue pending evolution");
    assert_eq!(pending.from_species_id, SpeciesId::Mofflet);
    assert_eq!(pending.to_species_id, SpeciesId::Fuzzard);
}

#[test]
fn returning_status_resolves_pending_evolution_and_emits_event() {
    let save_dir = test_save_dir("returning_status_resolves_pending_evolution_and_emits_event");
    let mut repository = FileRepository::new(save_dir.clone());
    let mut state = saved_state(3_600, GrowthStage::Stage1);
    state.pet.experience = 15;
    state
        .start_expedition(3_600, 0, 3_600)
        .expect("expedition should start");
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(7_200),
    );

    let outcome = service.status().expect("status should complete expedition");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    let evolution = outcome
        .evolution
        .expect("returning status should emit evolution event");
    assert_eq!(evolution.from_species_id, SpeciesId::Mofflet);
    assert_eq!(evolution.to_species_id, SpeciesId::Fuzzard);
    assert_eq!(outcome.state.pet.stage, GrowthStage::Stage2);
    assert_eq!(outcome.state.pet.species_id, SpeciesId::Fuzzard);
    assert!(outcome.state.pending_evolution.is_none());
    assert_eq!(loaded, outcome.state);

    cleanup(save_dir);
}

#[test]
fn pending_evolution_survives_save_load_until_pet_facing_status() {
    let save_dir = test_save_dir("pending_evolution_survives_save_load_until_pet_facing_status");
    let mut repository = FileRepository::new(save_dir.clone());
    let mut state = saved_state(3_600, GrowthStage::Stage1);
    state.pet.experience = 15;
    state
        .start_expedition(3_600, 0, 3_600)
        .expect("expedition should start");
    state.complete_expedition_if_due(7_200);

    repository
        .save(&state)
        .expect("pending game should be saved");
    let loaded = repository.load().expect("pending game should load");

    assert_eq!(loaded.pet.species_id, SpeciesId::Mofflet);
    assert_eq!(
        loaded
            .pending_evolution
            .expect("pending evolution should roundtrip")
            .to_species_id,
        SpeciesId::Fuzzard
    );

    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(7_200),
    );
    let outcome = service
        .status()
        .expect("pet-facing status should resolve pending evolution");

    assert_eq!(outcome.state.pet.species_id, SpeciesId::Fuzzard);
    assert!(outcome.evolution.is_some());

    cleanup(save_dir);
}

#[test]
fn starting_expedition_after_pending_evolution_preserves_evolution_event() {
    let save_dir = test_save_dir("starting_expedition_after_pending_evolution_preserves_event");
    let mut repository = FileRepository::new(save_dir.clone());
    let mut state = saved_state(7_200, GrowthStage::Stage1);
    state.pet.level = 3;
    state.pet.experience = 20;
    state.pending_evolution = Some(PendingEvolution {
        from_stage: GrowthStage::Stage1,
        from_species_id: SpeciesId::Mofflet,
        to_stage: GrowthStage::Stage2,
        to_species_id: SpeciesId::Fuzzard,
    });
    repository.save(&state).expect("game should be saved");
    let mut service = GameService::with_clock(
        FileRepository::new(save_dir.clone()),
        FixedClock::new(7_200),
    );

    let outcome = service
        .start_expedition()
        .expect("expedition should start after resolving pending evolution");
    let loaded = FileRepository::new(save_dir.clone())
        .load()
        .expect("updated game should load");

    let evolution = outcome
        .evolution
        .expect("go should preserve resolved evolution event");
    assert_eq!(evolution.to_species_id, SpeciesId::Fuzzard);
    assert_eq!(loaded.pet.species_id, SpeciesId::Fuzzard);
    assert!(loaded.pending_evolution.is_none());
    assert!(loaded.expedition.is_some());

    cleanup(save_dir);
}

fn saved_state(last_updated_at: u64, stage: GrowthStage) -> GameState {
    let mut pet = Pet::new("Mochi".to_string(), 1, 0, 72, 72, 72);
    pet.stage = stage;
    pet.species_id = match stage {
        GrowthStage::Egg | GrowthStage::Baby => SpeciesId::Baby,
        GrowthStage::Stage1 | GrowthStage::Stage2 | GrowthStage::Final => SpeciesId::Mofflet,
    };

    GameState {
        version: SAVE_VERSION,
        pet,
        last_updated_at,
        daily_actions: DailyActions::new(last_updated_at / 86_400),
        care_stats: CareStats::new(),
        daily_report: DailyReport::new(last_updated_at / 86_400),
        login: LoginState::new(),
        expedition: None,
        hatching: None,
        pending_evolution: None,
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
