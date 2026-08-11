use crate::application::{ApplicationResult, GameService};
use crate::domain::GameState;
use crate::infrastructure::clock::FixedClock;
use crate::infrastructure::storage::{
    state_from_json, state_to_json, GameRepository, MemoryRepository,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BitPetWasm {
    state: GameState,
}

#[wasm_bindgen]
impl BitPetWasm {
    pub fn new_game(now: u64) -> Result<BitPetWasm, JsValue> {
        let mut bitpet = Self {
            state: GameState::new(now),
        };
        bitpet.status(now)?;
        Ok(bitpet)
    }

    pub fn from_save_json(save_json: &str, now: u64) -> Result<BitPetWasm, JsValue> {
        let state = state_from_json(save_json).map_err(js_error)?;
        let mut bitpet = Self { state };
        bitpet.status(now)?;
        Ok(bitpet)
    }

    pub fn status(&mut self, now: u64) -> Result<String, JsValue> {
        self.run(now, |service| service.status())
    }

    pub fn feed(&mut self, now: u64) -> Result<String, JsValue> {
        self.run(now, |service| service.feed())
    }

    pub fn play(&mut self, now: u64) -> Result<String, JsValue> {
        self.run(now, |service| service.play())
    }

    pub fn go(&mut self, now: u64) -> Result<String, JsValue> {
        self.run(now, |service| service.start_expedition())
    }

    pub fn report(&mut self, now: u64) -> Result<String, JsValue> {
        self.run(now, |service| service.report())
    }

    pub fn streak(&mut self, now: u64) -> Result<String, JsValue> {
        self.run(now, |service| service.streak())
    }

    pub fn save_json(&self) -> Result<String, JsValue> {
        state_to_json(&self.state).map_err(js_error)
    }
}

impl BitPetWasm {
    fn run<T>(
        &mut self,
        now: u64,
        operation: impl FnOnce(&mut GameService<MemoryRepository, FixedClock>) -> ApplicationResult<T>,
    ) -> Result<String, JsValue> {
        let repository = MemoryRepository::new(self.state.clone());
        let mut service = GameService::with_clock(repository, FixedClock::new(now));
        operation(&mut service).map_err(js_error)?;
        self.state = service.into_repository().load().map_err(js_error)?;
        self.save_json()
    }
}

fn js_error(error: impl ToString) -> JsValue {
    JsValue::from_str(&error.to_string())
}
