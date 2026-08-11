use crate::domain::{GameState, Timestamp};

const SECONDS_PER_HOUR: Timestamp = 60 * 60;
const SECONDS_PER_DAY: Timestamp = 24 * SECONDS_PER_HOUR;
const HUNGER_DECAY_PER_HOUR: Timestamp = 3;
const ENERGY_RECOVERY_PER_HOUR: Timestamp = 2;

pub const fn day_index(timestamp: Timestamp) -> Timestamp {
    timestamp / SECONDS_PER_DAY
}

pub fn day_index_with_offset(timestamp: Timestamp, offset_seconds: i64) -> Timestamp {
    let shifted = if offset_seconds >= 0 {
        timestamp.saturating_add(offset_seconds as Timestamp)
    } else {
        timestamp.saturating_sub(offset_seconds.unsigned_abs())
    };

    day_index(shifted)
}

pub fn day_index_from_local_date(year: i32, month: i32, day: i32) -> Timestamp {
    days_from_civil(year, month, day).max(0) as Timestamp
}

pub fn apply_elapsed_time(state: &mut GameState, now: Timestamp) {
    if state.pet.is_egg() {
        state.last_updated_at = now;
        return;
    }

    let elapsed_seconds = now.saturating_sub(state.last_updated_at);
    let hunger_decay = elapsed_amount(elapsed_seconds, HUNGER_DECAY_PER_HOUR);
    let energy_recovery = elapsed_amount(elapsed_seconds, ENERGY_RECOVERY_PER_HOUR);

    state
        .pet
        .status
        .apply_elapsed(hunger_decay, energy_recovery);
    state.last_updated_at = now;
}

fn days_from_civil(year: i32, month: i32, day: i32) -> i64 {
    let year = year - i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month_adjusted = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month_adjusted + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    i64::from(era) * 146_097 + i64::from(doe) - 719_468
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
            hatching: None,
            pending_evolution: None,
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
            hatching: None,
            pending_evolution: None,
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
            hatching: None,
            pending_evolution: None,
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
            hatching: None,
            pending_evolution: None,
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
            hatching: None,
            pending_evolution: None,
        };

        apply_elapsed_time(&mut state, 9_000);

        assert_eq!(state.pet.status.hunger, 72);
        assert_eq!(state.pet.status.energy, 72);
        assert_eq!(state.last_updated_at, 9_000);
    }
}
