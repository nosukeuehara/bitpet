use crate::ascii::pets::pet_art;
use crate::domain::action::{Action, ActionOutcome};
use crate::domain::report::{ReportEvent, ReportEventKind};
use crate::domain::{DailyReport, GameState, LoginState};

pub fn render_status(state: &GameState) -> String {
    let pet = &state.pet;

    format!(
        "{}\n\n{}\n{}\nLv. {}\n\nStage    : {}\nMood     : {}%\nHunger   : {}%\nEnergy   : {}%",
        pet_art(pet),
        pet.name,
        pet.evolution.as_str(),
        pet.level,
        pet.stage.as_str(),
        pet.status.mood,
        pet.status.hunger,
        pet.status.energy
    )
}

pub fn render_not_implemented(command: &str) -> String {
    format!("{command} is not implemented yet.")
}

pub fn render_action_outcome(outcome: &ActionOutcome) -> String {
    let message = match outcome.action {
        Action::Feed => "Mochi ate a meal.",
        Action::Play => "You played with Mochi.",
        Action::Go => "go is not implemented yet.",
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
    let seconds_of_day = event.timestamp % 86_400;
    let hour = seconds_of_day / 3_600;
    let minute = seconds_of_day % 3_600 / 60;
    let message = match event.kind {
        ReportEventKind::Login => "Checked in",
        ReportEventKind::Feed => "Fed Mochi",
        ReportEventKind::Play => "Played with Mochi",
    };

    format!("{hour:02}:{minute:02} {message}")
}

#[cfg(test)]
mod tests {
    use super::{render_report, render_status, render_streak};
    use crate::domain::report::{DailyReport, LoginState};
    use crate::domain::GameState;

    #[test]
    fn renders_default_pet_status() {
        let output = render_status(&GameState::default());

        assert!(output.contains("Mochi"));
        assert!(output.contains("Lv. 1"));
        assert!(output.contains("Hunger"));
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
        assert!(output.contains("01:00 Fed Mochi"));
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
}
