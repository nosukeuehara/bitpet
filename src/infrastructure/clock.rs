use crate::domain::{time, Timestamp};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Clock {
    fn now(&self) -> Timestamp;

    fn day(&self, timestamp: Timestamp) -> Timestamp {
        time::day_index(timestamp)
    }
}

#[derive(Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Timestamp {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_secs())
    }

    fn day(&self, timestamp: Timestamp) -> Timestamp {
        local_day_index(timestamp).unwrap_or_else(|| time::day_index(timestamp))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FixedClock {
    now: Timestamp,
    local_day_offset_seconds: i64,
}

impl FixedClock {
    pub const fn new(now: Timestamp) -> Self {
        Self {
            now,
            local_day_offset_seconds: 0,
        }
    }

    pub const fn with_local_day_offset(now: Timestamp, local_day_offset_seconds: i64) -> Self {
        Self {
            now,
            local_day_offset_seconds,
        }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> Timestamp {
        self.now
    }

    fn day(&self, timestamp: Timestamp) -> Timestamp {
        time::day_index_with_offset(timestamp, self.local_day_offset_seconds)
    }
}

pub fn local_time_of_day(timestamp: Timestamp) -> Option<(u32, u32)> {
    local_hour_minute(timestamp)
}

#[cfg(unix)]
fn local_day_index(timestamp: Timestamp) -> Option<Timestamp> {
    use std::mem::MaybeUninit;
    use std::os::raw::{c_char, c_int, c_long};

    #[repr(C)]
    struct Tm {
        tm_sec: c_int,
        tm_min: c_int,
        tm_hour: c_int,
        tm_mday: c_int,
        tm_mon: c_int,
        tm_year: c_int,
        tm_wday: c_int,
        tm_yday: c_int,
        tm_isdst: c_int,
        tm_gmtoff: c_long,
        tm_zone: *const c_char,
    }

    unsafe extern "C" {
        fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
    }

    let time = i64::try_from(timestamp).ok()?;
    let mut local = MaybeUninit::<Tm>::uninit();
    let result = unsafe { localtime_r(&time, local.as_mut_ptr()) };
    if result.is_null() {
        return None;
    }

    let local = unsafe { local.assume_init() };
    Some(time::day_index_from_local_date(
        local.tm_year + 1900,
        local.tm_mon + 1,
        local.tm_mday,
    ))
}

#[cfg(unix)]
fn local_hour_minute(timestamp: Timestamp) -> Option<(u32, u32)> {
    use std::mem::MaybeUninit;
    use std::os::raw::{c_char, c_int, c_long};

    #[repr(C)]
    struct Tm {
        tm_sec: c_int,
        tm_min: c_int,
        tm_hour: c_int,
        tm_mday: c_int,
        tm_mon: c_int,
        tm_year: c_int,
        tm_wday: c_int,
        tm_yday: c_int,
        tm_isdst: c_int,
        tm_gmtoff: c_long,
        tm_zone: *const c_char,
    }

    unsafe extern "C" {
        fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
    }

    let time = i64::try_from(timestamp).ok()?;
    let mut local = MaybeUninit::<Tm>::uninit();
    let result = unsafe { localtime_r(&time, local.as_mut_ptr()) };
    if result.is_null() {
        return None;
    }

    let local = unsafe { local.assume_init() };
    Some((
        local.tm_hour.try_into().ok()?,
        local.tm_min.try_into().ok()?,
    ))
}

#[cfg(windows)]
fn local_day_index(timestamp: Timestamp) -> Option<Timestamp> {
    use std::mem::MaybeUninit;
    use std::os::raw::c_int;

    #[repr(C)]
    struct Tm {
        tm_sec: c_int,
        tm_min: c_int,
        tm_hour: c_int,
        tm_mday: c_int,
        tm_mon: c_int,
        tm_year: c_int,
        tm_wday: c_int,
        tm_yday: c_int,
        tm_isdst: c_int,
    }

    unsafe extern "C" {
        fn _localtime64_s(result: *mut Tm, time: *const i64) -> c_int;
    }

    let time = i64::try_from(timestamp).ok()?;
    let mut local = MaybeUninit::<Tm>::uninit();
    let result = unsafe { _localtime64_s(local.as_mut_ptr(), &time) };
    if result != 0 {
        return None;
    }

    let local = unsafe { local.assume_init() };
    Some(time::day_index_from_local_date(
        local.tm_year + 1900,
        local.tm_mon + 1,
        local.tm_mday,
    ))
}

#[cfg(windows)]
fn local_hour_minute(timestamp: Timestamp) -> Option<(u32, u32)> {
    use std::mem::MaybeUninit;
    use std::os::raw::c_int;

    #[repr(C)]
    struct Tm {
        tm_sec: c_int,
        tm_min: c_int,
        tm_hour: c_int,
        tm_mday: c_int,
        tm_mon: c_int,
        tm_year: c_int,
        tm_wday: c_int,
        tm_yday: c_int,
        tm_isdst: c_int,
    }

    unsafe extern "C" {
        fn _localtime64_s(result: *mut Tm, time: *const i64) -> c_int;
    }

    let time = i64::try_from(timestamp).ok()?;
    let mut local = MaybeUninit::<Tm>::uninit();
    let result = unsafe { _localtime64_s(local.as_mut_ptr(), &time) };
    if result != 0 {
        return None;
    }

    let local = unsafe { local.assume_init() };
    Some((
        local.tm_hour.try_into().ok()?,
        local.tm_min.try_into().ok()?,
    ))
}

#[cfg(target_arch = "wasm32")]
fn local_day_index(_timestamp: Timestamp) -> Option<Timestamp> {
    None
}

#[cfg(target_arch = "wasm32")]
fn local_hour_minute(_timestamp: Timestamp) -> Option<(u32, u32)> {
    None
}
