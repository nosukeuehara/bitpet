use crate::domain::{GameState, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyReport {
    pub day: Timestamp,
    pub feed_count: u32,
    pub play_count: u32,
    pub adventure_count: u32,
    pub experience_gained: u32,
    pub mood_delta: i32,
    pub events: Vec<ReportEvent>,
}

impl DailyReport {
    pub const fn new(day: Timestamp) -> Self {
        Self {
            day,
            feed_count: 0,
            play_count: 0,
            adventure_count: 0,
            experience_gained: 0,
            mood_delta: 0,
            events: Vec::new(),
        }
    }

    pub fn reset_if_new_day(&mut self, day: Timestamp) {
        if self.day != day {
            *self = Self::new(day);
        }
    }

    pub fn record_login(&mut self, timestamp: Timestamp) {
        self.events.push(ReportEvent {
            timestamp,
            kind: ReportEventKind::Login,
        });
    }

    pub fn record_feed(&mut self, timestamp: Timestamp, mood_delta: i32) {
        self.feed_count = self.feed_count.saturating_add(1);
        self.mood_delta = self.mood_delta.saturating_add(mood_delta);
        self.events.push(ReportEvent {
            timestamp,
            kind: ReportEventKind::Feed,
        });
    }

    pub fn record_play(&mut self, timestamp: Timestamp, experience_gained: u32, mood_delta: i32) {
        self.play_count = self.play_count.saturating_add(1);
        self.experience_gained = self.experience_gained.saturating_add(experience_gained);
        self.mood_delta = self.mood_delta.saturating_add(mood_delta);
        self.events.push(ReportEvent {
            timestamp,
            kind: ReportEventKind::Play,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportEvent {
    pub timestamp: Timestamp,
    pub kind: ReportEventKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportEventKind {
    Login,
    Feed,
    Play,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoginState {
    pub last_login_day: Option<Timestamp>,
    pub streak: u32,
}

impl LoginState {
    pub const fn new() -> Self {
        Self {
            last_login_day: None,
            streak: 0,
        }
    }

    pub fn record_login(&mut self, day: Timestamp) -> bool {
        match self.last_login_day {
            None => {
                self.last_login_day = Some(day);
                self.streak = 1;
                true
            }
            Some(last_day) if day == last_day => false,
            Some(last_day) if day == last_day.saturating_add(1) => {
                self.last_login_day = Some(day);
                self.streak = self.streak.saturating_add(1).max(1);
                true
            }
            Some(last_day) if day > last_day => {
                self.last_login_day = Some(day);
                self.streak = 1;
                true
            }
            Some(_) => false,
        }
    }
}

impl Default for LoginState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn reset_daily_records_if_new_day(&mut self, day: Timestamp) {
        self.daily_actions.reset_if_new_day(day);
        self.daily_report.reset_if_new_day(day);
    }

    pub fn record_login(&mut self, day: Timestamp, timestamp: Timestamp) {
        if self.login.record_login(day) {
            self.daily_report.record_login(timestamp);
        }
    }
}
