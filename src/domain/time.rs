use crate::domain::{GameState, Timestamp};

const SECONDS_PER_HOUR: Timestamp = 60 * 60;
const SECONDS_PER_DAY: Timestamp = 24 * SECONDS_PER_HOUR;
const HUNGER_DECAY_PER_HOUR: Timestamp = 3;
const ENERGY_RECOVERY_PER_HOUR: Timestamp = 2;

pub const fn day_index(timestamp: Timestamp) -> Timestamp {
    timestamp / SECONDS_PER_DAY
}

pub fn apply_elapsed_time(state: &mut GameState, now: Timestamp) {
    let elapsed_seconds = now.saturating_sub(state.last_updated_at);
    let hunger_decay = elapsed_amount(elapsed_seconds, HUNGER_DECAY_PER_HOUR);
    let energy_recovery = elapsed_amount(elapsed_seconds, ENERGY_RECOVERY_PER_HOUR);

    state
        .pet
        .status
        .apply_elapsed(hunger_decay, energy_recovery);
    state.last_updated_at = now;
}

fn elapsed_amount(elapsed_seconds: Timestamp, amount_per_hour: Timestamp) -> u8 {
    let amount = elapsed_seconds.saturating_mul(amount_per_hour) / SECONDS_PER_HOUR;
    amount.min(u8::MAX.into()) as u8
}

#[cfg(test)]
mod tests {
    use super::apply_elapsed_time;
    use crate::domain::{CareStats, DailyActions, DailyReport, GameState, LoginState, Pet};

    #[test]
    fn no_elapsed_time_keeps_status() {
        let mut state = GameState {
            version: 2,
            pet: Pet::new("Mochi".to_string(), 1, 0, 72, 72, 72),
            last_updated_at: 3_600,
            daily_actions: DailyActions::new(0),
            care_stats: CareStats::new(),
            daily_report: DailyReport::new(0),
            login: LoginState::new(),
            expedition: None,
        };

        apply_elapsed_time(&mut state, 3_600);

        assert_eq!(state.pet.status.hunger, 72);
        assert_eq!(state.pet.status.energy, 72);
        assert_eq!(state.last_updated_at, 3_600);
    }

    #[test]
    fn one_hour_changes_status_by_phase3_rates() {
        let mut state = GameState {
            version: 2,
            pet: Pet::new("Mochi".to_string(), 1, 0, 72, 72, 72),
            last_updated_at: 0,
            daily_actions: DailyActions::new(0),
            care_stats: CareStats::new(),
            daily_report: DailyReport::new(0),
            login: LoginState::new(),
            expedition: None,
        };

        apply_elapsed_time(&mut state, 3_600);

        assert_eq!(state.pet.status.hunger, 69);
        assert_eq!(state.pet.status.energy, 74);
        assert_eq!(state.last_updated_at, 3_600);
    }

    #[test]
    fn multiple_hours_accumulate_status_changes() {
        let mut state = GameState {
            version: 2,
            pet: Pet::new("Mochi".to_string(), 1, 0, 72, 72, 72),
            last_updated_at: 0,
            daily_actions: DailyActions::new(0),
            care_stats: CareStats::new(),
            daily_report: DailyReport::new(0),
            login: LoginState::new(),
            expedition: None,
        };

        apply_elapsed_time(&mut state, 10_800);

        assert_eq!(state.pet.status.hunger, 63);
        assert_eq!(state.pet.status.energy, 78);
    }

    #[test]
    fn status_values_stay_in_range_after_long_elapsed_time() {
        let mut state = GameState {
            version: 2,
            pet: Pet::new("Mochi".to_string(), 1, 0, 2, 72, 99),
            last_updated_at: 0,
            daily_actions: DailyActions::new(0),
            care_stats: CareStats::new(),
            daily_report: DailyReport::new(0),
            login: LoginState::new(),
            expedition: None,
        };

        apply_elapsed_time(&mut state, 360_000);

        assert_eq!(state.pet.status.hunger, 0);
        assert_eq!(state.pet.status.energy, 100);
    }

    #[test]
    fn future_last_updated_at_does_not_panic_or_change_status() {
        let mut state = GameState {
            version: 2,
            pet: Pet::new("Mochi".to_string(), 1, 0, 72, 72, 72),
            last_updated_at: 10_000,
            daily_actions: DailyActions::new(0),
            care_stats: CareStats::new(),
            daily_report: DailyReport::new(0),
            login: LoginState::new(),
            expedition: None,
        };

        apply_elapsed_time(&mut state, 9_000);

        assert_eq!(state.pet.status.hunger, 72);
        assert_eq!(state.pet.status.energy, 72);
        assert_eq!(state.last_updated_at, 9_000);
    }
}
