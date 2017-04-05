use std;



/// Struct that holds information about your app.
///
/// It's recommended to create a single `const` instance of `AppInfo`:
///
/// ```
/// use app_dirs::StaticAppInfo;
/// const APP_INFO: StaticAppInfo = StaticAppInfo{name: "Awesome App", author: "Dedicated Dev"};
/// ```
///
/// # Caveats
/// Functions in this library sanitize any characters that could be
/// non-filename-safe from `name` and `author`. The resulting paths will be
/// more human-readable if you stick to **letters, numbers, spaces, hyphens,
/// underscores, and periods** for both properties.
///
/// The `author` property is currently only used by Windows, as macOS and *nix
/// specifications don't require it. Make sure your `name` string is unique!
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticAppInfo {
    /// Name of your app (e.g. "Hearthstone").
    pub name: &'static str,
    /// Author of your app (e.g. "Blizzard").
    pub author: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwningAppInfo {
    /// Name of your app (e.g. "Hearthstone").
    pub name: String,
    /// Author of your app (e.g. "Blizzard").
    pub author: String,
}

/// Let's see...
///
/// ```
/// use app_dirs::*;
/// const APP_INFO: EAppInfo = EAppInfo::StaticAppInfo{name: "Awesome App", author: "Dedicated Dev"};
/// ```
pub enum EAppInfo {
    StaticAppInfo  {
        /// Name of your app (e.g. "Hearthstone").
        name: &'static str,
        /// Author of your app (e.g. "Blizzard").
        author: &'static str,
    },
    OwningAppInfo {
        /// Name of your app (e.g. "Hearthstone").
        name: String,
        /// Author of your app (e.g. "Blizzard").
        author: String,
    }
}

/// ```
/// use app_dirs::*;
/// use std::borrow::Cow;
/// const APP_INFO: CAppInfo = CAppInfo{name: Cow::Borrowed("Awesome App"), author: Cow::Borrowed("Dedicated Dev")};
/// let e = CAppInfo{name: Cow::Owned("Awesome App".to_owned()), author: Cow::Owned("Dedicated Dev".to_owned())};
/// ```
use std::borrow::Cow;
pub struct CAppInfo {
    pub name: Cow<'static, str>,
    pub author: Cow<'static, str>,
}

pub trait AppInfo {
    /// Name of your app (e.g. "Hearthstone").
    fn name(&self) -> &str;
    /// Author of your app (e.g. "Blizzard").
    fn author(&self) -> &str;
}

impl AppInfo for StaticAppInfo {
    fn name(&self) -> &str {
        self.name
    }

    fn author(&self) -> &str {
        self.author
    }
}

impl AppInfo for OwningAppInfo {
    fn name(&self) -> &str {
        &self.name
    }

    fn author(&self) -> &str {
        &self.author
    }
}


/// Enum specifying the type of app data you want to store.
///
/// **Different platforms are NOT guaranteed to distinguish between each data
/// type.** Keep this in mind when choosing data file paths.
///
/// Example: Windows does not supported shared application data and does not
/// distinguish between config and data. Therefore, on Windows, all variants
/// except `UserCache` return the same path.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AppDataType {
    /// User-specific app configuration data.
    UserConfig,
    /// User-specific arbitrary app data.
    UserData,
    /// User-specific app cache data.
    UserCache,
    /// System-wide arbitrary app data.
    SharedData,
    /// System-wide app configuration data.
    SharedConfig,
}

impl AppDataType {
    /// Returns `true` for non-user-specific data types.
    pub fn is_shared(&self) -> bool {
        use AppDataType::*;
        match *self {
            SharedData | SharedConfig => true,
            _ => false,
        }
    }
}

const ERR_NOT_SUPPORTED: &'static str = "App data directories not supported";
const ERR_INVALID_APP_INFO: &'static str = "Invalid app name or author";

/// Error type for any `app_dirs` operation.
#[derive(Debug)]
pub enum AppDirsError {
    /// An I/O error occurred during the operation.
    Io(std::io::Error),
    /// App-specific directories are not properly supported by the system
    /// (e.g. required environment variables don't exist).
    NotSupported,
    /// App info given to this library was invalid (e.g. app name or author
    /// were empty).
    InvalidAppInfo,
}

impl std::fmt::Display for AppDirsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use AppDirsError::*;
        match *self {
            Io(ref e) => e.fmt(f),
            NotSupported => f.write_str(ERR_NOT_SUPPORTED),
            InvalidAppInfo => f.write_str(ERR_INVALID_APP_INFO),
        }
    }
}

impl std::error::Error for AppDirsError {
    fn description(&self) -> &str {
        use AppDirsError::*;
        match *self {
            Io(ref e) => e.description(),
            NotSupported => "App data directories not supported",
            InvalidAppInfo => "Invalid app name or author",
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        use AppDirsError::*;
        match *self {
            Io(ref e) => Some(e),
            NotSupported => None,
            InvalidAppInfo => None,
        }
    }
}

impl From<std::io::Error> for AppDirsError {
    fn from(e: std::io::Error) -> Self {
        AppDirsError::Io(e)
    }
}
