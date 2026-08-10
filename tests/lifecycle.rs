use bitpet::application::GameService;
use bitpet::domain::GameState;
use bitpet::infrastructure::storage::MemoryRepository;

#[test]
fn status_returns_initial_game_state() {
    let service = GameService::new(MemoryRepository::new(GameState::default()));
    let state = service.status().expect("status should load game state");

    assert_eq!(state.version, 1);
    assert_eq!(state.pet.name, "Mochi");
    assert_eq!(state.pet.level, 1);
}
