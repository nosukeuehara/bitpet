use bitpet::application::GameService;
use bitpet::domain::{GameState, SAVE_VERSION};
use bitpet::infrastructure::clock::FixedClock;
use bitpet::infrastructure::storage::MemoryRepository;

#[test]
fn status_returns_initial_game_state() {
    let mut service = GameService::with_clock(
        MemoryRepository::new(GameState::new(3_600)),
        FixedClock::new(3_600),
    );
    let state = service.status().expect("status should load game state");

    assert_eq!(state.version, SAVE_VERSION);
    assert_eq!(state.pet.name, "Mochi");
    assert_eq!(state.pet.level, 1);
    assert_eq!(state.last_updated_at, 3_600);
}
