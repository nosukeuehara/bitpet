use crate::application::{ApplicationError, ApplicationResult};
use crate::domain::evolution::GrowthStage;
use crate::domain::expedition::{Expedition, ExpeditionType};
use crate::domain::monster::{legacy_evolution_species, species_from_str, SpeciesId};
use crate::domain::report::{ReportEvent, ReportEventKind};
use crate::domain::{
    CareStats, DailyActions, DailyReport, GameState, HatchingState, LoginState, Pet, Timestamp,
    SAVE_VERSION,
};
use crate::infrastructure::filesystem;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub trait GameRepository {
    fn exists(&self) -> bool;
    fn load(&self) -> ApplicationResult<GameState>;
    fn save(&mut self, state: &GameState) -> ApplicationResult<()>;
}

#[derive(Debug, Default)]
pub struct MemoryRepository {
    state: Option<GameState>,
}

impl MemoryRepository {
    pub fn new(state: GameState) -> Self {
        Self { state: Some(state) }
    }

    pub const fn empty() -> Self {
        Self { state: None }
    }
}

impl GameRepository for MemoryRepository {
    fn exists(&self) -> bool {
        self.state.is_some()
    }

    fn load(&self) -> ApplicationResult<GameState> {
        self.state.clone().ok_or(ApplicationError::InvalidSaveData)
    }

    fn save(&mut self, state: &GameState) -> ApplicationResult<()> {
        self.state = Some(state.clone());
        Ok(())
    }
}

pub fn storage_error(message: impl Into<String>) -> ApplicationError {
    ApplicationError::Storage(message.into())
}

#[derive(Debug, Clone)]
pub struct FileRepository {
    save_dir: PathBuf,
}

impl FileRepository {
    pub fn from_default_save_dir() -> ApplicationResult<Self> {
        let save_dir =
            filesystem::default_save_dir().ok_or(ApplicationError::SaveDirectoryUnavailable)?;
        Ok(Self::new(save_dir))
    }

    pub fn new(save_dir: PathBuf) -> Self {
        Self { save_dir }
    }

    pub fn save_dir(&self) -> &Path {
        &self.save_dir
    }

    pub fn save_path(&self) -> PathBuf {
        self.save_dir.join("save.json")
    }
}

impl GameRepository for FileRepository {
    fn exists(&self) -> bool {
        self.save_path().is_file()
    }

    fn load(&self) -> ApplicationResult<GameState> {
        let contents = fs::read_to_string(self.save_path()).map_err(storage_io_error)?;
        state_from_json(&contents)
    }

    fn save(&mut self, state: &GameState) -> ApplicationResult<()> {
        fs::create_dir_all(&self.save_dir).map_err(storage_io_error)?;

        let contents = state_to_json(state)?;
        let save_path = self.save_path();
        let temp_path = self.save_dir.join("save.json.tmp");

        fs::write(&temp_path, contents).map_err(storage_io_error)?;
        replace_file(&temp_path, &save_path).map_err(storage_io_error)?;

        Ok(())
    }
}

fn replace_file(from: &Path, to: &Path) -> io::Result<()> {
    match fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(error) if to.exists() => {
            fs::remove_file(to)?;
            fs::rename(from, to).map_err(|_| error)
        }
        Err(error) => Err(error),
    }
}

fn storage_io_error(error: io::Error) -> ApplicationError {
    storage_error(format!("BitPet couldn't access save data: {error}"))
}

fn storage_serde_error(error: serde_json::Error) -> ApplicationError {
    storage_error(format!("BitPet couldn't write save data: {error}"))
}

pub fn state_from_json(contents: &str) -> ApplicationResult<GameState> {
    let raw: serde_json::Value =
        serde_json::from_str(contents).map_err(|_| ApplicationError::InvalidSaveData)?;
    let version = raw_save_version(&raw)?;

    match version {
        1..=7 => {
            let save: LegacySaveData =
                serde_json::from_value(raw).map_err(|_| ApplicationError::InvalidSaveData)?;
            save.try_into()
        }
        SAVE_VERSION => {
            let save: CurrentSaveData =
                serde_json::from_value(raw).map_err(|_| ApplicationError::InvalidSaveData)?;
            save.try_into()
        }
        _ => Err(ApplicationError::InvalidSaveData),
    }
}

pub fn state_to_json(state: &GameState) -> ApplicationResult<String> {
    let save = CurrentSaveData::from(state);
    serde_json::to_string_pretty(&save).map_err(storage_serde_error)
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentSaveData {
    version: u32,
    last_updated_at: Timestamp,
    daily_actions: SaveDailyActions,
    care_stats: SaveCareStats,
    daily_report: SaveDailyReport,
    login: SaveLoginState,
    #[serde(default)]
    expedition: Option<SaveExpedition>,
    hatching: Option<SaveHatchingState>,
    pet: CurrentSavePet,
}

#[derive(Debug, Deserialize)]
struct LegacySaveData {
    version: u32,
    #[serde(default)]
    last_updated_at: Option<Timestamp>,
    #[serde(default)]
    daily_actions: Option<SaveDailyActions>,
    #[serde(default)]
    care_stats: Option<SaveCareStats>,
    #[serde(default)]
    daily_report: Option<LegacySaveDailyReport>,
    #[serde(default)]
    login: Option<LegacySaveLoginState>,
    #[serde(default)]
    expedition: Option<SaveExpedition>,
    #[serde(default)]
    hatching: Option<SaveHatchingState>,
    pet: LegacySavePet,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveDailyActions {
    day: Timestamp,
    feed_count: u32,
    play_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveCareStats {
    feed_total: u32,
    play_total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveDailyReport {
    day: Timestamp,
    feed_count: u32,
    play_count: u32,
    adventure_count: u32,
    experience_gained: u32,
    mood_delta: i32,
    events: Vec<SaveReportEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveReportEvent {
    timestamp: Timestamp,
    kind: String,
}

#[derive(Debug, Deserialize)]
struct LegacySaveDailyReport {
    #[serde(default)]
    day: Timestamp,
    #[serde(default)]
    feed_count: u32,
    #[serde(default)]
    play_count: u32,
    #[serde(default)]
    adventure_count: u32,
    #[serde(default)]
    experience_gained: u32,
    #[serde(default)]
    mood_delta: i32,
    #[serde(default)]
    events: Vec<SaveReportEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveLoginState {
    last_login_day: Option<Timestamp>,
    streak: u32,
}

#[derive(Debug, Deserialize)]
struct LegacySaveLoginState {
    #[serde(default)]
    last_login_day: Option<Timestamp>,
    #[serde(default)]
    streak: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveExpedition {
    expedition_type: String,
    started_at: Timestamp,
    returns_at: Timestamp,
    seed: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveHatchingState {
    egg_created_at: Timestamp,
    hatches_at: Timestamp,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentSavePet {
    name: String,
    stage: String,
    species_id: String,
    level: u32,
    experience: u32,
    hunger: u8,
    mood: u8,
    energy: u8,
}

#[derive(Debug, Deserialize)]
struct LegacySavePet {
    name: String,
    #[serde(default)]
    stage: Option<String>,
    #[serde(default)]
    species_id: Option<String>,
    #[serde(default)]
    evolution: Option<String>,
    level: u32,
    experience: u32,
    hunger: u8,
    mood: u8,
    energy: u8,
}

impl From<&GameState> for CurrentSaveData {
    fn from(state: &GameState) -> Self {
        Self {
            version: state.version,
            last_updated_at: state.last_updated_at,
            daily_actions: SaveDailyActions {
                day: state.daily_actions.day,
                feed_count: state.daily_actions.feed_count,
                play_count: state.daily_actions.play_count,
            },
            care_stats: SaveCareStats {
                feed_total: state.care_stats.feed_total,
                play_total: state.care_stats.play_total,
            },
            daily_report: SaveDailyReport {
                day: state.daily_report.day,
                feed_count: state.daily_report.feed_count,
                play_count: state.daily_report.play_count,
                adventure_count: state.daily_report.adventure_count,
                experience_gained: state.daily_report.experience_gained,
                mood_delta: state.daily_report.mood_delta,
                events: state
                    .daily_report
                    .events
                    .iter()
                    .map(SaveReportEvent::from)
                    .collect(),
            },
            login: SaveLoginState {
                last_login_day: state.login.last_login_day,
                streak: state.login.streak,
            },
            expedition: state.expedition.map(SaveExpedition::from),
            hatching: state.hatching.map(SaveHatchingState::from),
            pet: CurrentSavePet {
                name: state.pet.name.clone(),
                stage: state.pet.stage.as_str().to_string(),
                species_id: state.pet.species_id.as_str().to_string(),
                level: state.pet.level,
                experience: state.pet.experience,
                hunger: state.pet.status.hunger,
                mood: state.pet.status.mood,
                energy: state.pet.status.energy,
            },
        }
    }
}

impl TryFrom<CurrentSaveData> for GameState {
    type Error = ApplicationError;

    fn try_from(save: CurrentSaveData) -> Result<Self, Self::Error> {
        if save.version != SAVE_VERSION {
            return Err(ApplicationError::InvalidSaveData);
        }

        let expedition = save.expedition.map(TryInto::try_into).transpose()?;

        let stage = parse_growth_stage(Some(save.pet.stage.as_str()))?;
        let hatching = parse_hatching(stage, save.hatching)?;
        let species_id = species_from_str(save.pet.species_id.as_str())
            .ok_or(ApplicationError::InvalidSaveData)?;
        let mut pet = Pet::new(
            save.pet.name,
            save.pet.level,
            save.pet.experience,
            save.pet.hunger,
            save.pet.mood,
            save.pet.energy,
        );
        pet.stage = stage;
        pet.species_id = species_id;

        Ok(Self {
            version: save.version,
            pet,
            last_updated_at: save.last_updated_at,
            daily_actions: DailyActions {
                day: save.daily_actions.day,
                feed_count: save.daily_actions.feed_count,
                play_count: save.daily_actions.play_count,
            },
            care_stats: CareStats {
                feed_total: save.care_stats.feed_total,
                play_total: save.care_stats.play_total,
            },
            daily_report: save.daily_report.try_into()?,
            login: LoginState {
                last_login_day: save.login.last_login_day,
                streak: save.login.streak,
            },
            expedition,
            hatching,
        })
    }
}

impl TryFrom<LegacySaveData> for GameState {
    type Error = ApplicationError;

    fn try_from(save: LegacySaveData) -> Result<Self, Self::Error> {
        if !matches!(save.version, 1..=7) {
            return Err(ApplicationError::InvalidSaveData);
        }

        let daily_actions = save.daily_actions.map_or_else(
            || DailyActions::new(0),
            |actions| DailyActions {
                day: actions.day,
                feed_count: actions.feed_count,
                play_count: actions.play_count,
            },
        );

        let care_stats = save
            .care_stats
            .map_or_else(CareStats::new, |stats| CareStats {
                feed_total: stats.feed_total,
                play_total: stats.play_total,
            });

        let daily_report = save
            .daily_report
            .map(TryInto::try_into)
            .transpose()?
            .unwrap_or_else(|| DailyReport::new(0));

        let login = save.login.map_or_else(LoginState::new, |login| LoginState {
            last_login_day: login.last_login_day,
            streak: login.streak,
        });

        let expedition = save.expedition.map(TryInto::try_into).transpose()?;

        let stage = parse_growth_stage(save.pet.stage.as_deref())?;
        let hatching = parse_hatching(stage, save.hatching)?;
        let species_id = parse_species_id(
            save.pet.species_id.as_deref(),
            save.pet.evolution.as_deref(),
        )?;
        let mut pet = Pet::new(
            save.pet.name,
            save.pet.level,
            save.pet.experience,
            save.pet.hunger,
            save.pet.mood,
            save.pet.energy,
        );
        pet.stage = stage;
        pet.species_id = species_id;

        Ok(Self {
            version: save.version,
            pet,
            last_updated_at: save.last_updated_at.unwrap_or(0),
            daily_actions,
            care_stats,
            daily_report,
            login,
            expedition,
            hatching,
        })
    }
}

impl From<&ReportEvent> for SaveReportEvent {
    fn from(event: &ReportEvent) -> Self {
        Self {
            timestamp: event.timestamp,
            kind: match event.kind {
                ReportEventKind::Login => "login",
                ReportEventKind::Feed => "feed",
                ReportEventKind::Play => "play",
                ReportEventKind::ExpeditionStarted => "expedition_started",
                ReportEventKind::ExpeditionCompleted => "expedition_completed",
            }
            .to_string(),
        }
    }
}

impl TryFrom<SaveDailyReport> for DailyReport {
    type Error = ApplicationError;

    fn try_from(report: SaveDailyReport) -> Result<Self, Self::Error> {
        Ok(Self {
            day: report.day,
            feed_count: report.feed_count,
            play_count: report.play_count,
            adventure_count: report.adventure_count,
            experience_gained: report.experience_gained,
            mood_delta: report.mood_delta,
            events: parse_report_events(report.events)?,
        })
    }
}

impl TryFrom<LegacySaveDailyReport> for DailyReport {
    type Error = ApplicationError;

    fn try_from(report: LegacySaveDailyReport) -> Result<Self, Self::Error> {
        Ok(Self {
            day: report.day,
            feed_count: report.feed_count,
            play_count: report.play_count,
            adventure_count: report.adventure_count,
            experience_gained: report.experience_gained,
            mood_delta: report.mood_delta,
            events: parse_report_events(report.events)?,
        })
    }
}

impl TryFrom<SaveReportEvent> for ReportEvent {
    type Error = ApplicationError;

    fn try_from(event: SaveReportEvent) -> Result<Self, Self::Error> {
        let kind = match event.kind.as_str() {
            "login" => ReportEventKind::Login,
            "feed" => ReportEventKind::Feed,
            "play" => ReportEventKind::Play,
            "expedition_started" => ReportEventKind::ExpeditionStarted,
            "expedition_completed" => ReportEventKind::ExpeditionCompleted,
            _ => return Err(ApplicationError::InvalidSaveData),
        };

        Ok(Self {
            timestamp: event.timestamp,
            kind,
        })
    }
}

impl From<Expedition> for SaveExpedition {
    fn from(expedition: Expedition) -> Self {
        Self {
            expedition_type: expedition.expedition_type.as_str().to_string(),
            started_at: expedition.started_at,
            returns_at: expedition.returns_at,
            seed: expedition.seed,
        }
    }
}

impl TryFrom<SaveExpedition> for Expedition {
    type Error = ApplicationError;

    fn try_from(expedition: SaveExpedition) -> Result<Self, Self::Error> {
        if expedition.returns_at < expedition.started_at {
            return Err(ApplicationError::InvalidSaveData);
        }

        let expedition_type = match expedition.expedition_type.as_str() {
            "Explore" => ExpeditionType::Explore,
            _ => return Err(ApplicationError::InvalidSaveData),
        };

        Ok(Self {
            expedition_type,
            started_at: expedition.started_at,
            returns_at: expedition.returns_at,
            seed: expedition.seed,
        })
    }
}

impl From<HatchingState> for SaveHatchingState {
    fn from(hatching: HatchingState) -> Self {
        Self {
            egg_created_at: hatching.egg_created_at,
            hatches_at: hatching.hatches_at,
        }
    }
}

impl TryFrom<SaveHatchingState> for HatchingState {
    type Error = ApplicationError;

    fn try_from(hatching: SaveHatchingState) -> Result<Self, Self::Error> {
        if hatching.hatches_at < hatching.egg_created_at {
            return Err(ApplicationError::InvalidSaveData);
        }

        Ok(Self {
            egg_created_at: hatching.egg_created_at,
            hatches_at: hatching.hatches_at,
        })
    }
}

fn parse_growth_stage(value: Option<&str>) -> ApplicationResult<GrowthStage> {
    match value {
        Some("Egg") => Ok(GrowthStage::Egg),
        None | Some("Baby") => Ok(GrowthStage::Baby),
        Some("Stage 1" | "Stage1") => Ok(GrowthStage::Stage1),
        Some("Stage 2" | "Stage2") => Ok(GrowthStage::Stage2),
        Some("Final") => Ok(GrowthStage::Final),
        Some(_) => Err(ApplicationError::InvalidSaveData),
    }
}

fn parse_hatching(
    stage: GrowthStage,
    hatching: Option<SaveHatchingState>,
) -> ApplicationResult<Option<HatchingState>> {
    match (stage, hatching) {
        (GrowthStage::Egg, Some(hatching)) => Ok(Some(hatching.try_into()?)),
        (GrowthStage::Egg, None) => Err(ApplicationError::InvalidSaveData),
        (_, None) => Ok(None),
        (_, Some(_)) => Err(ApplicationError::InvalidSaveData),
    }
}

fn parse_species_id(
    species_id: Option<&str>,
    legacy_evolution: Option<&str>,
) -> ApplicationResult<SpeciesId> {
    if let Some(species_id) = species_id {
        return species_from_str(species_id).ok_or(ApplicationError::InvalidSaveData);
    }

    legacy_evolution_species(legacy_evolution).ok_or(ApplicationError::InvalidSaveData)
}

fn raw_save_version(raw: &serde_json::Value) -> ApplicationResult<u32> {
    let Some(version) = raw.get("version").and_then(serde_json::Value::as_u64) else {
        return Err(ApplicationError::InvalidSaveData);
    };

    u32::try_from(version).map_err(|_| ApplicationError::InvalidSaveData)
}

fn parse_report_events(events: Vec<SaveReportEvent>) -> ApplicationResult<Vec<ReportEvent>> {
    events
        .into_iter()
        .map(TryInto::try_into)
        .collect::<ApplicationResult<Vec<_>>>()
}
