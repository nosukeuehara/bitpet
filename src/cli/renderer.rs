use crate::application::StatusOutcome;
use crate::ascii::pets::{pet_art, species_art, DOOR_PET};
use crate::cli::commands::HelpTopic;
use crate::domain::action::{Action, ActionOutcome};
use crate::domain::expedition::ExpeditionOutcome;
use crate::domain::report::{ReportEvent, ReportEventKind};
use crate::domain::{DailyReport, EvolutionEvent, GameState, LoginState};
use crate::infrastructure::clock::local_time_of_day;

pub fn render_status_outcome(outcome: &StatusOutcome) -> String {
    render_with_evolution(outcome.evolution, &outcome.state)
}

pub fn render_help(topic: Option<HelpTopic>) -> String {
    match topic {
        None => render_main_help(),
        Some(HelpTopic::Status) => render_command_help(
            "status",
            "Show your BitPet",
            "bitpet status",
            "Displays your BitPet's current state.",
        ),
        Some(HelpTopic::Feed) => render_command_help(
            "feed",
            "Feed your BitPet",
            "bitpet feed",
            "Feeds your BitPet when it is home and hatched.",
        ),
        Some(HelpTopic::Play) => render_command_help(
            "play",
            "Play with your BitPet",
            "bitpet play",
            "Plays with your BitPet when it is home and hatched.",
        ),
        Some(HelpTopic::Go) => render_command_help(
            "go",
            "Send your BitPet on an expedition",
            "bitpet go",
            "Sends your BitPet on an expedition when expeditions are unlocked.",
        ),
        Some(HelpTopic::Report) => render_command_help(
            "report",
            "Show today's activity report",
            "bitpet report",
            "Shows today's feed, play, expedition, and event report.",
        ),
        Some(HelpTopic::Streak) => render_command_help(
            "streak",
            "Show your login streak",
            "bitpet streak",
            "Shows your current login streak.",
        ),
        Some(HelpTopic::Update) => render_command_help(
            "update",
            "Update BitPet",
            "bitpet update [--check]",
            "Checks GitHub Releases for a newer stable Native CLI release, or installs it.",
        ),
    }
}

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
            "{}\n\nOut now...\nBack at {}\n\nReturns in:\n{}",
            DOOR_PET,
            render_time_of_day(expedition.returns_at),
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
    let message = format!(
        "Mochi went exploring.\n\nExpected return:\n{}",
        render_time_of_day(outcome.returns_at)
    );

    if let Some(event) = outcome.evolution {
        return format!("{}\n\n{message}", render_evolution_effect(event));
    }

    message
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

    format!(
        "{message}\n\n{}",
        render_with_evolution(outcome.evolution, &outcome.state)
    )
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

fn render_main_help() -> String {
    "BitPet - a tiny CLI pet that grows while you work

Usage:
  bitpet [COMMAND]

Commands:
  status    Show your BitPet
  feed      Feed your BitPet
  play      Play with your BitPet
  go        Send your BitPet on an expedition
  report    Show today's activity report
  streak    Show your login streak
  update    Update BitPet
  help      Show help for a command

Options:
  -h, --help       Show help
  -V, --version    Show version"
        .to_string()
}

pub fn render_update_up_to_date(current: &str) -> String {
    format!("BitPet is already up to date.\n{current}")
}

pub fn render_update_available(current: &str, latest: &str) -> String {
    format!(
        "Current: {current}\nLatest : {latest}\n\nUpdate available.\nRun `bitpet update` to install."
    )
}

pub fn render_update_success(previous: &str, current: &str) -> String {
    format!(
        "Updating BitPet...\n\n{previous} -> {current}\n\nDownloading...\nVerifying...\nInstalling...\n\nBitPet updated successfully."
    )
}

fn render_command_help(command: &str, description: &str, usage: &str, details: &str) -> String {
    format!(
        "{description}

Usage:
  {usage}

{details}

Options:
  -h, --help    Show help for bitpet {command}"
    )
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

pub fn render_evolution_effect(event: EvolutionEvent) -> String {
    let old_art = species_art(event.from_species_id);
    let new_art = species_art(event.to_species_id);
    let clear = "\x1B[2J\x1B[H";
    let blank = "\n\n\n\n\n";

    format!(
        "{clear}{old_art}\n{clear}{blank}\n{clear}{old_art}\n{clear}{blank}\n{clear}{new_art}\n\nYour BitPet evolved!\n{} -> {}",
        event.from_species_id.display_name(),
        event.to_species_id.display_name()
    )
}

fn render_with_evolution(evolution: Option<EvolutionEvent>, state: &GameState) -> String {
    if let Some(event) = evolution {
        return format!(
            "{}\n\n{}",
            render_evolution_effect(event),
            render_status(state)
        );
    }

    render_status(state)
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
    use super::{
        render_evolution_effect, render_expedition_started, render_help, render_report,
        render_status, render_streak,
    };
    use crate::cli::commands::HelpTopic;
    use crate::domain::evolution::GrowthStage;
    use crate::domain::expedition::{Expedition, ExpeditionOutcome, ExpeditionType};
    use crate::domain::monster::SpeciesId;
    use crate::domain::report::{DailyReport, LoginState};
    use crate::domain::{EvolutionEvent, GameState};

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
        state.pet.species_id = SpeciesId::Mofflet;
        state.hatching = None;
        state.expedition = Some(Expedition {
            expedition_type: ExpeditionType::Explore,
            started_at: 3_600,
            returns_at: 7_200,
            seed: 3_600,
        });

        let output = render_status(&state);

        assert!(output.contains("+------+"));
        assert!(output.contains("Out now..."));
        assert!(output.contains("Back at"));
        assert!(output.contains("Returns in:"));
        assert!(output.contains("1h 0m"));
        assert!(!output.contains("Mofflet"));
        assert!(!output.contains("Family"));
    }

    #[test]
    fn renders_expedition_start() {
        let output = render_expedition_started(&ExpeditionOutcome {
            expedition_type: ExpeditionType::Explore,
            started_at: 3_600,
            returns_at: 7_200,
            evolution: None,
        });

        assert!(output.contains("Mochi went exploring."));
        assert!(output.contains("Expected return:"));
    }

    #[test]
    fn renders_main_help_with_commands_and_options() {
        let output = render_help(None);

        assert!(output.contains("BitPet - a tiny CLI pet"));
        assert!(output.contains("Usage:"));
        assert!(output.contains("status    Show your BitPet"));
        assert!(output.contains("feed      Feed your BitPet"));
        assert!(output.contains("play      Play with your BitPet"));
        assert!(output.contains("go        Send your BitPet on an expedition"));
        assert!(output.contains("report    Show today's activity report"));
        assert!(output.contains("streak    Show your login streak"));
        assert!(output.contains("-h, --help"));
        assert!(output.contains("-V, --version"));
    }

    #[test]
    fn renders_subcommand_help() {
        let output = render_help(Some(HelpTopic::Go));

        assert!(output.contains("Send your BitPet on an expedition"));
        assert!(output.contains("Usage:"));
        assert!(output.contains("bitpet go"));
        assert!(output.contains("-h, --help"));
    }

    #[test]
    fn renders_evolution_effect_without_waiting_for_tty() {
        let output = render_evolution_effect(EvolutionEvent {
            from_stage: GrowthStage::Baby,
            from_species_id: SpeciesId::Baby,
            to_stage: GrowthStage::Stage1,
            to_species_id: SpeciesId::Mofflet,
        });

        assert!(output.contains("\x1B[2J\x1B[H"));
        assert!(output.contains("Your BitPet evolved!"));
        assert!(output.contains("Baby -> Mofflet"));
    }
}
