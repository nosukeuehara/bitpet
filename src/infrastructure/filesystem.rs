use std::env;
use std::path::PathBuf;

pub fn default_save_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .map(|path| path.join("BitPet"))
    } else {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| path.join(".bitpet"))
    }
}
