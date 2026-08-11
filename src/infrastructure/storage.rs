use crate::application::{ApplicationError, ApplicationResult};
use crate::domain::evolution::{EvolutionKind, GrowthStage};
use crate::domain::{CareStats, DailyActions, GameState, Pet, Timestamp, SAVE_VERSION};
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
        let save: SaveData =
            serde_json::from_str(&contents).map_err(|_| ApplicationError::InvalidSaveData)?;
        save.try_into()
    }

    fn save(&mut self, state: &GameState) -> ApplicationResult<()> {
        fs::create_dir_all(&self.save_dir).map_err(storage_io_error)?;

        let save = SaveData::from(state);
        let contents = serde_json::to_string_pretty(&save).map_err(storage_serde_error)?;
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

#[derive(Debug, Serialize, Deserialize)]
struct SaveData {
    version: u32,
    last_updated_at: Option<Timestamp>,
    daily_actions: Option<SaveDailyActions>,
    care_stats: Option<SaveCareStats>,
    pet: SavePet,
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
struct SavePet {
    name: String,
    stage: Option<String>,
    evolution: Option<String>,
    level: u32,
    experience: u32,
    hunger: u8,
    mood: u8,
    energy: u8,
}

impl From<&GameState> for SaveData {
    fn from(state: &GameState) -> Self {
        Self {
            version: state.version,
            last_updated_at: Some(state.last_updated_at),
            daily_actions: Some(SaveDailyActions {
                day: state.daily_actions.day,
                feed_count: state.daily_actions.feed_count,
                play_count: state.daily_actions.play_count,
            }),
            care_stats: Some(SaveCareStats {
                feed_total: state.care_stats.feed_total,
                play_total: state.care_stats.play_total,
            }),
            pet: SavePet {
                name: state.pet.name.clone(),
                stage: Some(state.pet.stage.as_str().to_string()),
                evolution: Some(state.pet.evolution.as_str().to_string()),
                level: state.pet.level,
                experience: state.pet.experience,
                hunger: state.pet.status.hunger,
                mood: state.pet.status.mood,
                energy: state.pet.status.energy,
            },
        }
    }
}

impl TryFrom<SaveData> for GameState {
    type Error = ApplicationError;

    fn try_from(save: SaveData) -> Result<Self, Self::Error> {
        if !matches!(save.version, 1..=SAVE_VERSION) {
            return Err(ApplicationError::InvalidSaveData);
        }

        if save.version == SAVE_VERSION && save.last_updated_at.is_none() {
            return Err(ApplicationError::InvalidSaveData);
        }

        if save.version == SAVE_VERSION && save.daily_actions.is_none() {
            return Err(ApplicationError::InvalidSaveData);
        }

        if save.version == SAVE_VERSION && save.care_stats.is_none() {
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

        let stage = parse_growth_stage(save.pet.stage.as_deref())?;
        let evolution = parse_evolution_kind(save.pet.evolution.as_deref())?;
        let mut pet = Pet::new(
            save.pet.name,
            save.pet.level,
            save.pet.experience,
            save.pet.hunger,
            save.pet.mood,
            save.pet.energy,
        );
        pet.stage = stage;
        pet.evolution = evolution;

        Ok(Self {
            version: save.version,
            pet,
            last_updated_at: save.last_updated_at.unwrap_or(0),
            daily_actions,
            care_stats,
        })
    }
}

fn parse_growth_stage(value: Option<&str>) -> ApplicationResult<GrowthStage> {
    match value {
        None | Some("Baby") => Ok(GrowthStage::Baby),
        Some("Stage 1") => Ok(GrowthStage::Stage1),
        Some(_) => Err(ApplicationError::InvalidSaveData),
    }
}

fn parse_evolution_kind(value: Option<&str>) -> ApplicationResult<EvolutionKind> {
    match value {
        None | Some("Baby") => Ok(EvolutionKind::Baby),
        Some("Fluffy") => Ok(EvolutionKind::Fluffy),
        Some("Sharp") => Ok(EvolutionKind::Sharp),
        Some("Weird") => Ok(EvolutionKind::Weird),
        Some(_) => Err(ApplicationError::InvalidSaveData),
    }
}
