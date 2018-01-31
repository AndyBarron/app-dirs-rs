use AppDataType::*;
use common::*;
use std::env::home_dir;
use std::path::{Component, PathBuf};

pub const USE_AUTHOR: bool = false;

pub fn get_app_dir(t: AppDataType) -> Result<PathBuf, AppDirsError> {
    let dir_base: Result<PathBuf, AppDirsError> = if t.is_shared() {
        Ok(Component::RootDir.as_os_str().into())
    } else {
        home_dir().ok_or_else(|| AppDirsError::NotSupported)
    };
    dir_base.map(|mut path| {
        match t {
            UserConfig | UserData | SharedConfig | SharedData => {
                path.push("Library");
                path.push("Application Support");
            },
            UserCache => {
                path.push("Library");
                path.push("Caches");
            },
        };
        path
    })
}
