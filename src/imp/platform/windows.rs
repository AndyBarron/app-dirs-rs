use common::*;
use std::path::PathBuf;
use std::env;
use AppDirType::*;

pub fn get_app_dir(t: AppDirType) -> AppDirResult<PathBuf> {
    match t {
        UserConfig | UserData | SharedConfig | SharedData =>
            env::var("APPDATA"),
        UserCache =>
            env::var("LOCALAPPDATA"),
    }.and_then(|x| Ok(PathBuf::from(x))).or_else(|_| Err(AppDirError::NotSupported))
}
