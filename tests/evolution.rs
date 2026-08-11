use bitpet::application::GameService;
use bitpet::domain::evolution::GrowthStage;
use bitpet::domain::monster::{definition, MonsterFamily, SpeciesId, MONSTER_CATALOG};
use bitpet::domain::{
    CareStats, DailyActions, DailyReport, GameState, LoginState, Pet, SAVE_VERSION,
};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::{FileRepository, GameRepository};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn play_can_level_up_and_evolve_to_spindle() {
    let save_dir = test_save_dir("play_can_level_up_and_evolve_to_spindle");
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
    assert_eq!(outcome.state.pet.species_id, SpeciesId::Spindle);
    assert_eq!(loaded.pet.species_id, SpeciesId::Spindle);

    cleanup(save_dir);
}

#[test]
fn baby_evolves_to_stage1_species_after_level_threshold() {
    let mut state = saved_state(10, 1, 3_600);

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.level, 2);
    assert_eq!(state.pet.stage, GrowthStage::Stage1);
    assert_eq!(state.pet.species_id, SpeciesId::Mofflet);
}

#[test]
fn stage1_evolves_to_stage2_species_after_level_threshold() {
    let mut state = saved_state(20, 1, 3_600);
    state.pet.stage = GrowthStage::Stage1;
    state.pet.species_id = SpeciesId::Mofflet;

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.level, 3);
    assert_eq!(state.pet.stage, GrowthStage::Stage2);
    assert_eq!(state.pet.species_id, SpeciesId::Fuzzard);
}

#[test]
fn stage2_evolves_to_final_species_after_level_threshold() {
    let mut state = saved_state(30, 4, 3_600);
    state.care_stats.play_total = 2;
    state.pet.stage = GrowthStage::Stage2;
    state.pet.species_id = SpeciesId::Fuzzard;

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.level, 4);
    assert_eq!(state.pet.stage, GrowthStage::Final);
    assert_eq!(state.pet.species_id, SpeciesId::Brumruff);
}

#[test]
fn family_selection_uses_care_totals() {
    let cases = [
        (3, 2, SpeciesId::Mofflet, MonsterFamily::Fuzz),
        (4, 2, SpeciesId::Spriglet, MonsterFamily::Flora),
        (2, 3, SpeciesId::Spindle, MonsterFamily::Spike),
        (2, 4, SpeciesId::Flitter, MonsterFamily::Wing),
        (3, 3, SpeciesId::Bloblet, MonsterFamily::Drift),
        (2, 2, SpeciesId::Buddle, MonsterFamily::Colony),
        (1, 1, SpeciesId::Wormlet, MonsterFamily::Oddling),
    ];

    for (feed_total, play_total, species_id, family) in cases {
        let mut state = saved_state(10, feed_total, 3_600);
        state.care_stats.play_total = play_total;

        state.pet.update_growth(state.care_stats);

        assert_eq!(state.pet.species_id, species_id);
        assert_eq!(state.pet.family(), Some(family));
    }
}

#[test]
fn final_branch_selection_stays_within_family() {
    let mut state = saved_state(30, 1, 3_600);
    state.care_stats.play_total = 5;
    state.pet.stage = GrowthStage::Stage2;
    state.pet.species_id = SpeciesId::Pricklet;

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.stage, GrowthStage::Final);
    assert_eq!(state.pet.species_id, SpeciesId::Starwing);
    assert_eq!(state.pet.family(), Some(MonsterFamily::Spike));
}

#[test]
fn catalog_contains_all_monster_definitions() {
    assert_eq!(MONSTER_CATALOG.len(), 28);

    let mofflet = definition(SpeciesId::Mofflet).expect("mofflet should exist");

    assert_eq!(mofflet.display_name, "Mofflet");
    assert_eq!(mofflet.family, MonsterFamily::Fuzz);
    assert_eq!(mofflet.growth_stage, GrowthStage::Stage1);
}

#[test]
fn final_pet_does_not_change_species_on_later_growth_update() {
    let mut state = saved_state(10, 0, 3_600);
    state.pet.stage = GrowthStage::Final;
    state.pet.species_id = SpeciesId::Starwing;
    state.care_stats.feed_total = 10;

    state.pet.update_growth(state.care_stats);

    assert_eq!(state.pet.species_id, SpeciesId::Starwing);
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
