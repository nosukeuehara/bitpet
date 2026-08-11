use crate::ascii::pets::pet_art;
use crate::domain::action::{Action, ActionOutcome};
use crate::domain::expedition::ExpeditionOutcome;
use crate::domain::report::{ReportEvent, ReportEventKind};
use crate::domain::{DailyReport, GameState, LoginState};
use crate::infrastructure::clock::local_time_of_day;

pub fn render_status(state: &GameState) -> String {
    if state.pet.is_egg() {
        let remaining = state
            .hatching
            .map(|hatching| hatching.hatches_at.saturating_sub(state.last_updated_at))
            .unwrap_or(0);

        return format!(
            "{}\n\nEgg\n\nHatching in {}",
            pet_art(&state.pet),
            render_duration(remaining)
        );
    }

    if let Some(expedition) = state.expedition {
        return format!(
            "{}\n\n{} is exploring.\n\nReturns in:\n{}",
            pet_art(&state.pet),
            state.pet.name,
            render_duration(expedition.returns_at.saturating_sub(state.last_updated_at))
        );
    }

    let pet = &state.pet;
    let family = pet.family().map_or("-", |family| family.as_str());

    format!(
        "{}\n\n{}\n{}\nLv. {}\n\nFamily   : {}\nStage    : {}\nMood     : {}%\nHunger   : {}%\nEnergy   : {}%",
        pet_art(pet),
        pet.name,
        pet.species_name(),
        pet.level,
        family,
        pet.stage.as_str(),
        pet.status.mood,
        pet.status.hunger,
        pet.status.energy
    )
}

pub fn render_expedition_started(outcome: &ExpeditionOutcome) -> String {
    format!(
        "Mochi went exploring.\n\nExpected return:\n{}",
        render_time_of_day(outcome.returns_at)
    )
}

pub fn render_expedition_locked() -> String {
    "Mochi is not ready to explore yet.\n\nReach Stage 1 first.".to_string()
}

pub fn render_pet_not_hatched() -> String {
    "The egg has not hatched yet.\n\nCheck back later.".to_string()
}

pub fn render_pet_away() -> String {
    "Mochi is exploring.\n\nPlease wait until Mochi returns.".to_string()
}

pub fn render_not_implemented(command: &str) -> String {
    format!("{command} is not implemented yet.")
}

pub fn render_action_outcome(outcome: &ActionOutcome) -> String {
    let message = match outcome.action {
        Action::Feed => "Mochi ate a meal.",
        Action::Play => "You played with Mochi.",
        Action::Go => "Mochi went exploring.",
    };

    format!("{message}\n\n{}", render_status(&outcome.state))
}

pub fn render_action_limit_reached(action: Action) -> String {
    match action {
        Action::Feed => "Mochi looks full.\n\nMaybe try again tomorrow.".to_string(),
        Action::Play => "Mochi needs a break.\n\nMaybe try again tomorrow.".to_string(),
        Action::Go => "That action is not available today.".to_string(),
    }
}

pub fn render_report(report: &DailyReport) -> String {
    let mut output = format!(
        "BitPet Daily Report\n\nFeed        {}\nPlay        {}\nAdventure   {}\n\nEXP gained  {}\nMood        {:+}",
        report.feed_count,
        report.play_count,
        report.adventure_count,
        report.experience_gained,
        report.mood_delta
    );

    if !report.events.is_empty() {
        output.push_str("\n\nEvents");
        for event in &report.events {
            output.push('\n');
            output.push_str(&render_report_event(event));
        }
    }

    output
}

pub fn render_streak(login: &LoginState) -> String {
    format!("Login streak\n\n{} day(s)", login.streak)
}

fn render_report_event(event: &ReportEvent) -> String {
    let (hour, minute) = local_time_or_utc(event.timestamp);
    let message = match event.kind {
        ReportEventKind::Login => "Checked in",
        ReportEventKind::Feed => "Fed Mochi",
        ReportEventKind::Play => "Played with Mochi",
        ReportEventKind::ExpeditionStarted => "Went exploring",
        ReportEventKind::ExpeditionCompleted => "Came home",
    };

    format!("{hour:02}:{minute:02} {message}")
}

fn render_time_of_day(timestamp: u64) -> String {
    let (hour, minute) = local_time_or_utc(timestamp);
    format!("{hour:02}:{minute:02}")
}

fn local_time_or_utc(timestamp: u64) -> (u64, u64) {
    local_time_of_day(timestamp).map_or_else(
        || {
            let seconds_of_day = timestamp % 86_400;
            (seconds_of_day / 3_600, seconds_of_day % 3_600 / 60)
        },
        |(hour, minute)| (u64::from(hour), u64::from(minute)),
    )
}

fn render_duration(seconds: u64) -> String {
    let hours = seconds / 3_600;
    let minutes = seconds % 3_600 / 60;

    if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

#[cfg(test)]
mod tests {
    use super::{render_expedition_started, render_report, render_status, render_streak};
    use crate::domain::evolution::GrowthStage;
    use crate::domain::expedition::{Expedition, ExpeditionOutcome, ExpeditionType};
    use crate::domain::report::{DailyReport, LoginState};
    use crate::domain::GameState;

    #[test]
    fn renders_default_pet_status() {
        let mut state = GameState::default();
        state.pet.hatch();
        state.hatching = None;
        let output = render_status(&state);

        assert!(output.contains("Mochi"));
        assert!(output.contains("Lv. 1"));
        assert!(output.contains("Hunger"));
    }

    #[test]
    fn renders_egg_status() {
        let output = render_status(&GameState::default());

        assert!(output.contains("Egg"));
        assert!(output.contains("Hatching in"));
    }

    #[test]
    fn renders_daily_report() {
        let mut report = DailyReport::new(0);
        report.record_feed(3_600, 5);
        report.record_play(3_660, 5, 10);

        let output = render_report(&report);

        assert!(output.contains("BitPet Daily Report"));
        assert!(output.contains("Feed        1"));
        assert!(output.contains("Play        1"));
        assert!(output.contains("EXP gained  5"));
        assert!(output.contains("Mood        +15"));
        assert!(output.contains("Fed Mochi"));
    }

    #[test]
    fn renders_login_streak() {
        let output = render_streak(&LoginState {
            last_login_day: Some(2),
            streak: 3,
        });

        assert!(output.contains("Login streak"));
        assert!(output.contains("3 day(s)"));
    }

    #[test]
    fn renders_away_status() {
        let mut state = GameState::new(3_600);
        state.pet.stage = GrowthStage::Stage1;
        state.hatching = None;
        state.expedition = Some(Expedition {
            expedition_type: ExpeditionType::Explore,
            started_at: 3_600,
            returns_at: 7_200,
            seed: 3_600,
        });

        let output = render_status(&state);

        assert!(output.contains("Mochi is exploring."));
        assert!(output.contains("Returns in:"));
        assert!(output.contains("1h 0m"));
    }

    #[test]
    fn renders_expedition_start() {
        let output = render_expedition_started(&ExpeditionOutcome {
            expedition_type: ExpeditionType::Explore,
            started_at: 3_600,
            returns_at: 7_200,
        });

        assert!(output.contains("Mochi went exploring."));
        assert!(output.contains("Expected return:"));
    }
}
