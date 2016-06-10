use std;

#[derive(Clone, Debug)]
pub struct AppInfo {
    name: String,
    author: String,
    safe_name: String,
    safe_author: String,
}

impl AppInfo {
    pub fn new(name: &str, author: &str) -> Self {
        AppInfo {
            name: name.into(),
            author: author.into(),
            safe_name: name.into(), // TODO
            safe_author: author.into(), // TODO
        }
    }
    pub fn safe_name(&self) -> &str {
        &self.safe_name
    }
    pub fn safe_author(&self) -> &str {
        &self.safe_author
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_author(&self) -> &str {
        &self.author
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppDirType {
    UserConfig,
    UserData,
    UserCache,
    SharedData,
    SharedConfig,
}

impl AppDirType {
    pub fn is_shared(&self) -> bool {
        use AppDirType::*;
        match *self {
            SharedData | SharedConfig => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum AppDirError {
    Io(std::io::Error),
    // NotFound(PathBuf),
    NotSupported,
}

impl From<std::io::Error> for AppDirError {
    fn from(e: std::io::Error) -> Self {
        AppDirError::Io(e)
    }
}

pub type AppDirResult<T> = Result<T, AppDirError>;